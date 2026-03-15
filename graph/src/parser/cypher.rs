//! Cypher query parser for FalkorDB.
//!
//! This module implements a hand-written recursive descent parser for the Cypher
//! query language. It converts Cypher query strings into an Abstract Syntax Tree
//! (AST) defined in [`crate::parser::ast`].
//! Lexical analysis is provided by [`crate::parser::lexer`], and parser helper
//! macros are defined in [`crate::parser::r#macro`].
//!
//! ## Architecture
//!
//! This module contains the **parser** component:
//!
//! 1. **Parser** (`Parser`): Consumes tokens and builds the AST using recursive
//!    descent with operator precedence parsing for expressions.
//!
//! Related modules:
//! - [`crate::parser::lexer`]: token stream production
//! - [`crate::parser::r#macro`]: parser helper macros
//!
//! ## Entry Point
//!
//! The main entry point is [`parse`], which takes a query string and returns
//! a [`RawQueryIR`] (unbound AST).
//!
//! ```text
//! "MATCH (n) RETURN n"
//!         │
//!         ▼
//!     Lexer → [MATCH, LPAREN, IDENT("n"), RPAREN, RETURN, IDENT("n")]
//!         │
//!         ▼
//!     Parser → QueryIR::Query([
//!                  QueryIR::Match { pattern: ..., filter: None, optional: false },
//!                  QueryIR::Return { exprs: [("n", var("n"))], ... }
//!              ])
//! ```
//!
//! ## Expression Precedence
//!
//! Expressions are parsed with the following precedence (lowest to highest):
//! 1. OR
//! 2. XOR
//! 3. AND
//! 4. NOT
//! 5. Comparison (=, <>, <, >, <=, >=, IN, STARTS WITH, etc.)
//! 6. Addition/Subtraction (+, -)
//! 7. Multiplication/Division (*, /, %)
//! 8. Power (^)
//! 9. Unary minus (-)
//! 10. Property access, indexing, function calls
//!
//! ## Error Handling
//!
//! Parse errors return `Err(String)` with a descriptive message including
//! the position in the query where the error occurred.

use crate::entity_type::EntityType;
use crate::index::indexer::IndexType;
use crate::parser::ast::{
    ExprIR, QuantifierType, QueryExpr, QueryGraph, QueryIR, QueryNode, QueryPath,
    QueryRelationship, RawQueryIR, SetItem,
};
use crate::parser::lexer::{Keyword, Lexer, Token};
use crate::runtime::orderset::OrderSet;
use crate::tree;
use crate::{
    parser::lexer::Token::RParen,
    runtime::{
        functions::{FnType, get_functions},
        value::Value,
    },
};
use itertools::Itertools;
use orx_tree::{DynTree, NodeRef};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
#[derive(Debug)]
enum ExpressionListType {
    OneOrMore,
    ZeroOrMoreClosedBy(Token),
}

impl ExpressionListType {
    fn is_end_token(
        &self,
        current_token: &Token,
    ) -> bool {
        match self {
            Self::OneOrMore => false,
            Self::ZeroOrMoreClosedBy(token) => token == current_token,
        }
    }
}

#[derive(Clone, Copy)]
struct ParserState {
    pos: usize,
    anon_counter: u32,
}

/// Cypher query parser.
///
/// The parser implements a recursive descent parser with operator precedence
/// for expressions. It consumes tokens from the lexer and builds an AST.
///
/// # Usage
/// ```ignore
/// let mut parser = Parser::new("MATCH (n:Person) RETURN n.name");
/// let ast = parser.parse()?;
/// ```
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    /// Counter for generating unique anonymous variable names
    anon_counter: u32,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given query string.
    #[must_use]
    pub fn new(str: &'a str) -> Self {
        Self {
            lexer: Lexer::new(str),
            anon_counter: 0,
        }
    }

    fn save_state(&self) -> ParserState {
        ParserState {
            pos: self.lexer.pos(true),
            anon_counter: self.anon_counter,
        }
    }

    fn restore_state(
        &mut self,
        state: ParserState,
    ) {
        self.lexer.set_pos(state.pos);
        self.anon_counter = state.anon_counter;
    }

    /// Checks if a tree or its descendants contain an aggregate function using DFS traversal
    fn contains_nested_aggregate(tree: &DynTree<ExprIR<Arc<String>>>) -> bool {
        use orx_tree::Dfs;

        // Traverse all nodes in the tree using DFS
        for idx in tree.root().indices::<Dfs>() {
            if let ExprIR::FuncInvocation(func) = tree.node(idx).data()
                && func.is_aggregate()
            {
                return true;
            }
        }

        false
    }

    /// Parses query parameters from CYPHER prefix.
    ///
    /// Handles queries like: `CYPHER param1=value1 param2=value2 MATCH ...`
    /// Returns the parameters map and the remaining query string.
    pub fn parse_parameters(
        &mut self
    ) -> Result<(HashMap<String, DynTree<ExprIR<Arc<String>>>>, &'a str), String> {
        let mut params = HashMap::new();
        while let Ok(Token::Ident(id)) = self.lexer.current() {
            if id.as_str() == "CYPHER" {
                self.lexer.next();
                let mut state = self.save_state();
                while let Ok(id) = self.parse_ident() {
                    if !optional_match_token!(self.lexer, Equal) {
                        self.restore_state(state);
                        break;
                    }
                    params.insert(String::from(id.as_str()), self.parse_expr(false)?);
                    state = self.save_state();
                }
            } else {
                break;
            }
        }
        Ok((params, &self.lexer.str[self.lexer.pos(true)..]))
    }

    /// Consumes any trailing semicolons, then verifies end-of-file.
    fn expect_end_of_input(&mut self) -> Result<(), String> {
        while matches!(self.lexer.current()?, Token::Semicolon) {
            self.lexer.next();
        }
        if !matches!(self.lexer.current()?, Token::EndOfFile) {
            return Err("query with more than one statement is not supported".to_string());
        }
        Ok(())
    }

    /// Parses a complete Cypher query.
    ///
    /// This is the main entry point for parsing. It first tries to parse index
    /// operations (CREATE INDEX, DROP INDEX), then falls back to regular queries.
    /// Trailing semicolons are consumed; multi-statement queries (content after
    /// a semicolon) are rejected.
    ///
    /// # Errors
    /// Returns an error string if the query has syntax errors.
    pub fn parse(&mut self) -> Result<RawQueryIR, String> {
        let state = self.save_state();
        let result = if let Some(ir) = self.parse_index_ops()? {
            ir
        } else {
            self.restore_state(state);
            self.parse_query()?
        };

        self.expect_end_of_input()?;

        Ok(result)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn parse_index_ops(&mut self) -> Result<Option<RawQueryIR>, String> {
        if optional_match_token!(self.lexer => Create) {
            let fulltext = optional_match_token!(self.lexer => Fulltext);
            let vector = !fulltext && optional_match_token!(self.lexer => Vector);
            let index = optional_match_token!(self.lexer => Index);
            if !index {
                return Ok(None);
            }
            if !fulltext && !vector && optional_match_token!(self.lexer => On) {
                match_token!(self.lexer, Colon);
                let label = self.parse_ident()?;
                match_token!(self.lexer, LParen);
                let mut attrs = vec![self.parse_ident()?];
                while optional_match_token!(self.lexer, Comma) {
                    attrs.push(self.parse_ident()?);
                }
                match_token!(self.lexer, RParen);
                let index_type = IndexType::Range;
                let entity_type = EntityType::Node;
                return Ok(Some(QueryIR::CreateIndex {
                    label,
                    attrs,
                    index_type,
                    entity_type,
                    options: None,
                }));
            }
            match_token!(self.lexer => For);
            match_token!(self.lexer, LParen);
            let (nkey, label, entity_type) = if optional_match_token!(self.lexer, RParen) {
                match_token!(self.lexer, Dash);
                match_token!(self.lexer, LBrace);
                let nkey = self.parse_ident()?;
                match_token!(self.lexer, Colon);
                let label = self.parse_ident()?;
                match_token!(self.lexer, RBrace);
                match_token!(self.lexer, Dash);
                optional_match_token!(self.lexer, GreaterThan);
                match_token!(self.lexer, LParen);
                match_token!(self.lexer, RParen);
                (nkey, label, EntityType::Relationship)
            } else {
                let nkey = self.parse_ident()?;
                if !matches!(self.lexer.current()?, Token::Colon) {
                    return Err(self.lexer.format_error(&format!(
                        "Invalid input '{}': expected a label",
                        self.lexer.current_str()
                    )));
                }
                self.lexer.next();
                let label = self.parse_ident()?;
                match_token!(self.lexer, RParen);
                (nkey, label, EntityType::Node)
            };
            match_token!(self.lexer => On);
            match_token!(self.lexer, LParen);
            let key = self.parse_ident()?;
            self.match_dot_property_separator()?;
            if nkey.as_str() != key.as_str() {
                return Err(self.lexer.format_error(&format!("'{key}' not defined")));
            }
            let mut attrs = vec![self.parse_property_name()?];
            while optional_match_token!(self.lexer, Comma) {
                let key = self.parse_ident()?;
                self.match_dot_property_separator()?;
                if nkey.as_str() != key.as_str() {
                    return Err(self.lexer.format_error(&format!("'{key}' not defined")));
                }
                attrs.push(self.parse_property_name()?);
            }
            match_token!(self.lexer, RParen);
            let index_type = if fulltext {
                IndexType::Fulltext
            } else if vector {
                IndexType::Vector
            } else {
                IndexType::Range
            };
            let options = if (vector || fulltext) && optional_match_token!(self.lexer => Options) {
                Some(Arc::new(self.parse_map()?))
            } else {
                None
            };
            return Ok(Some(QueryIR::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options,
            }));
        }
        if optional_match_token!(self.lexer => Drop) {
            let fulltext = optional_match_token!(self.lexer => Fulltext);
            let vector = !fulltext && optional_match_token!(self.lexer => Vector);
            let index = optional_match_token!(self.lexer => Index);
            if !index {
                return Ok(None);
            }
            if !fulltext && !vector && optional_match_token!(self.lexer => On) {
                match_token!(self.lexer, Colon);
                let label = self.parse_ident()?;
                match_token!(self.lexer, LParen);
                let mut attrs = vec![self.parse_ident()?];
                while optional_match_token!(self.lexer, Comma) {
                    attrs.push(self.parse_ident()?);
                }
                match_token!(self.lexer, RParen);
                let index_type = IndexType::Range;
                let entity_type = EntityType::Node;
                return Ok(Some(QueryIR::DropIndex {
                    label,
                    attrs,
                    index_type,
                    entity_type,
                }));
            }
            match_token!(self.lexer => For);
            match_token!(self.lexer, LParen);
            let (nkey, label, entity_type) = if optional_match_token!(self.lexer, RParen) {
                match_token!(self.lexer, Dash);
                match_token!(self.lexer, LBrace);
                let nkey = self.parse_ident()?;
                match_token!(self.lexer, Colon);
                let label = self.parse_ident()?;
                match_token!(self.lexer, RBrace);
                match_token!(self.lexer, Dash);
                optional_match_token!(self.lexer, GreaterThan);
                match_token!(self.lexer, LParen);
                match_token!(self.lexer, RParen);
                (nkey, label, EntityType::Relationship)
            } else {
                let nkey = self.parse_ident()?;
                if !matches!(self.lexer.current()?, Token::Colon) {
                    return Err(self.lexer.format_error(&format!(
                        "Invalid input '{}': expected a label",
                        self.lexer.current_str()
                    )));
                }
                self.lexer.next();
                let label = self.parse_ident()?;
                match_token!(self.lexer, RParen);
                (nkey, label, EntityType::Node)
            };
            match_token!(self.lexer => On);
            match_token!(self.lexer, LParen);
            let key = self.parse_ident()?;
            self.match_dot_property_separator()?;
            if nkey.as_str() != key.as_str() {
                return Err(self.lexer.format_error(&format!("'{key}' not defined")));
            }
            let mut attrs = vec![self.parse_property_name()?];
            while optional_match_token!(self.lexer, Comma) {
                let key = self.parse_ident()?;
                self.match_dot_property_separator()?;
                if nkey.as_str() != key.as_str() {
                    return Err(self.lexer.format_error(&format!("'{key}' not defined")));
                }
                attrs.push(self.parse_property_name()?);
            }
            match_token!(self.lexer, RParen);
            let index_type = if fulltext {
                IndexType::Fulltext
            } else if vector {
                IndexType::Vector
            } else {
                IndexType::Range
            };
            return Ok(Some(QueryIR::DropIndex {
                label,
                attrs,
                index_type,
                entity_type,
            }));
        }
        Ok(None)
    }

    /// Parses a complete query, handling UNION if present.
    ///
    /// A Cypher query may consist of multiple sub-queries joined by UNION:
    ///
    /// ```cypher
    /// MATCH (n:Person) RETURN n.name AS name
    /// UNION
    /// MATCH (n:Company) RETURN n.title AS name
    /// ```
    ///
    /// Each sub-query is parsed by `parse_single_query()`.  If UNION appears
    /// after the first sub-query's RETURN, additional sub-queries are parsed
    /// and wrapped in `QueryIR::Union`.
    fn parse_query(&mut self) -> Result<RawQueryIR, String> {
        let first = self.parse_single_query()?;
        if optional_match_token!(self.lexer => Union) {
            let all = optional_match_token!(self.lexer => All);
            let mut branches = vec![first];
            branches.push(self.parse_single_query()?);
            while optional_match_token!(self.lexer => Union) {
                let next_all = optional_match_token!(self.lexer => All);
                if next_all != all {
                    return Err(self
                        .lexer
                        .format_error("Invalid combination of UNION and UNION ALL."));
                }
                branches.push(self.parse_single_query()?);
            }
            return Ok(QueryIR::Union(branches, all));
        }
        Ok(first)
    }

    /// Parses a single sub-query (clauses up through an optional RETURN).
    ///
    /// This handles the reading/writing clause loops and WITH/RETURN
    /// projection.  Called once for standalone queries and once per branch
    /// in a UNION query.
    fn parse_single_query(&mut self) -> Result<RawQueryIR, String> {
        let mut clauses = Vec::new();
        let mut write = false;
        loop {
            while let Token::Keyword(
                Keyword::Optional
                | Keyword::Match
                | Keyword::Unwind
                | Keyword::Call
                | Keyword::Load,
                _,
            ) = self.lexer.current()?
            {
                let clause = self.parse_reading_clasue()?;
                clauses.push(clause);
            }
            while let Token::Keyword(
                Keyword::Create
                | Keyword::Merge
                | Keyword::Delete
                | Keyword::Detach
                | Keyword::Set
                | Keyword::Remove,
                _,
            ) = self.lexer.current()?
            {
                write = true;
                let clause = self.parse_writing_clause()?;
                clauses.push(clause);
            }
            if optional_match_token!(self.lexer => With) {
                clauses.push(self.parse_with_clause(write)?);
            } else {
                break;
            }
            write = false;
        }
        if optional_match_token!(self.lexer => Return) {
            clauses.push(self.parse_return_clause(write)?);
            write = false;
            // After RETURN, only UNION, semicolons, or end-of-file may follow.
            match self.lexer.current()? {
                Token::EndOfFile | Token::Keyword(Keyword::Union, _) | Token::Semicolon => {}
                _ => {
                    return Err(self
                        .lexer
                        .format_error("Unexpected clause following RETURN"));
                }
            }
        }
        if !matches!(
            self.lexer.current()?,
            Token::Keyword(Keyword::Union, _) | Token::Semicolon
        ) {
            match_token!(self.lexer, EndOfFile);
        }
        Ok(QueryIR::Query(clauses, write))
    }

    fn parse_reading_clasue(&mut self) -> Result<RawQueryIR, String> {
        if optional_match_token!(self.lexer => Optional) {
            match_token!(self.lexer => Match);
            return self.parse_match_clause(true);
        }
        match self.lexer.current()? {
            Token::Keyword(Keyword::Match, _) => {
                self.lexer.next();
                optional_match_token!(self.lexer => Match);
                self.parse_match_clause(false)
            }
            Token::Keyword(Keyword::Unwind, _) => {
                self.lexer.next();
                self.parse_unwind_clause()
            }
            Token::Keyword(Keyword::Call, _) => {
                self.lexer.next();
                self.parse_call_clause()
            }
            Token::Keyword(Keyword::Load, _) => {
                self.lexer.next();
                match_token!(self.lexer => Csv);
                let headers = optional_match_token!(self.lexer => With)
                    && optional_match_token!(self.lexer => Headers);
                let delimiter = if optional_match_token!(self.lexer => Delimiter) {
                    Arc::new(self.parse_expr(false)?)
                } else {
                    Arc::new(tree!(ExprIR::String(Arc::new(String::from(',')))))
                };
                match_token!(self.lexer => From);
                let file_path = Arc::new(self.parse_expr(false)?);
                match_token!(self.lexer => As);
                let ident = self.parse_ident()?;
                Ok(QueryIR::LoadCsv {
                    file_path,
                    headers,
                    delimiter,
                    var: ident,
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_writing_clause(&mut self) -> Result<RawQueryIR, String> {
        match self.lexer.current()? {
            Token::Keyword(Keyword::Create, _) => {
                self.lexer.next();
                self.parse_create_clause()
            }
            Token::Keyword(Keyword::Merge, _) => {
                self.lexer.next();
                self.parse_merge_clause()
            }
            Token::Keyword(Keyword::Detach | Keyword::Delete, _) => {
                let is_detach = optional_match_token!(self.lexer => Detach);
                match_token!(self.lexer => Delete);
                self.parse_delete_clause(is_detach)
            }
            Token::Keyword(Keyword::Set, _) => {
                self.lexer.next();
                self.parse_set_clause()
            }
            Token::Keyword(Keyword::Remove, _) => {
                self.lexer.next();
                self.parse_remove_clause()
            }
            _ => unreachable!(),
        }
    }

    fn parse_call_clause(&mut self) -> Result<RawQueryIR, String> {
        let function_name = self.parse_dotted_ident()?;
        let func = get_functions().get(function_name.as_str(), &FnType::Procedure(vec![]))?;
        match_token!(self.lexer, LParen);
        let args = self
            .parse_expression_list(ExpressionListType::ZeroOrMoreClosedBy(RParen), false)?
            .into_iter()
            .map(Arc::new)
            .collect();
        let mut named_outputs = vec![];
        let filter = if optional_match_token!(self.lexer => Yield) {
            let ident = self.parse_ident()?;
            named_outputs.push(ident);
            while optional_match_token!(self.lexer, Comma) {
                let ident = self.parse_ident()?;
                named_outputs.push(ident);
            }
            self.parse_where()?
        } else if let FnType::Procedure(defult_outputs) = &func.fn_type {
            for output in defult_outputs {
                named_outputs.push(Arc::new(output.clone()));
            }
            None
        } else {
            None
        };

        Ok(QueryIR::Call(func, args, named_outputs, filter))
    }

    fn parse_dotted_ident(&mut self) -> Result<Arc<String>, String> {
        let mut idents = vec![self.parse_ident()?];
        while self.lexer.current()? == Token::Dot {
            self.lexer.next();
            idents.push(self.parse_ident()?);
        }
        Ok(Arc::new(
            idents.iter().map(|label| label.as_str()).join("."),
        ))
    }

    fn parse_match_clause(
        &mut self,
        optional: bool,
    ) -> Result<RawQueryIR, String> {
        Ok(QueryIR::Match {
            pattern: self.parse_pattern(&Keyword::Match)?,
            filter: self.parse_where()?,
            optional,
        })
    }

    fn parse_unwind_clause(&mut self) -> Result<RawQueryIR, String> {
        let list = Arc::new(self.parse_expr(false)?);
        match_token!(self.lexer => As);
        let ident = self.parse_ident()?;
        Ok(QueryIR::Unwind(list, ident))
    }

    fn parse_create_clause(&mut self) -> Result<RawQueryIR, String> {
        Ok(QueryIR::Create(self.parse_pattern(&Keyword::Create)?))
    }

    fn parse_merge_clause(&mut self) -> Result<RawQueryIR, String> {
        let pattern = self.parse_pattern(&Keyword::Merge)?;
        let mut on_match_set_items = vec![];
        let mut on_create_set_items = vec![];
        while optional_match_token!(self.lexer => On) {
            if optional_match_token!(self.lexer => Match) {
                match_token!(self.lexer => Set);
                self.parse_set_items(&mut on_match_set_items)?;
            } else if optional_match_token!(self.lexer => Create) {
                match_token!(self.lexer => Set);
                self.parse_set_items(&mut on_create_set_items)?;
            } else {
                return Err(self.lexer.format_error("Expected MATCH or CREATE after ON"));
            }
        }
        Ok(QueryIR::Merge(
            pattern,
            on_create_set_items,
            on_match_set_items,
        ))
    }

    fn parse_delete_clause(
        &mut self,
        is_detach: bool,
    ) -> Result<RawQueryIR, String> {
        Ok(QueryIR::Delete(
            self.parse_expression_list(ExpressionListType::OneOrMore, false)?
                .into_iter()
                .map(Arc::new)
                .collect(),
            is_detach,
        ))
    }

    fn parse_where(&mut self) -> Result<Option<QueryExpr<Arc<String>>>, String> {
        if let Token::Keyword(Keyword::Where, _) = self.lexer.current()? {
            self.lexer.next();
            return Ok(Some(Arc::new(self.parse_expr(true)?)));
        }
        Ok(None)
    }

    fn parse_with_clause(
        &mut self,
        write: bool,
    ) -> Result<RawQueryIR, String> {
        let distinct = optional_match_token!(self.lexer => Distinct);
        let (all, exprs) = if optional_match_token!(self.lexer, Star) {
            // WITH * carries forward the current named_in_scope unchanged
            (true, vec![])
        } else {
            (false, self.parse_named_exprs(true)?)
        };
        let orderby = if optional_match_token!(self.lexer => Order) {
            self.parse_orderby()?
        } else {
            vec![]
        };
        let skip = if optional_match_token!(self.lexer => Skip) {
            let skip = Arc::new(self.parse_expr(false)?);
            match skip.root().data() {
                ExprIR::Integer(i) => {
                    if *i < 0 {
                        return Err(self.lexer.format_error(
                            "SKIP specified value of invalid type, must be a positive integer",
                        ));
                    }
                }
                ExprIR::Parameter(_) => {}
                _ => {
                    return Err(self.lexer.format_error(
                        "SKIP specified value of invalid type, must be a positive integer",
                    ));
                }
            }
            Some(skip)
        } else {
            None
        };
        let limit = if optional_match_token!(self.lexer => Limit) {
            let limit = Arc::new(self.parse_expr(false)?);
            match limit.root().data() {
                ExprIR::Integer(i) => {
                    if *i < 0 {
                        return Err(self.lexer.format_error(
                            "LIMIT specified value of invalid type, must be a positive integer",
                        ));
                    }
                }
                ExprIR::Parameter(_) => {}
                _ => {
                    return Err(self.lexer.format_error(
                        "LIMIT specified value of invalid type, must be a positive integer",
                    ));
                }
            }
            Some(limit)
        } else {
            None
        };
        let filter = self.parse_where()?;
        Ok(QueryIR::With {
            distinct,
            all,
            exprs,
            copy_from_parent: Vec::new(),
            orderby,
            skip,
            limit,
            filter,
            write,
        })
    }

    fn parse_return_clause(
        &mut self,
        write: bool,
    ) -> Result<RawQueryIR, String> {
        let distinct = optional_match_token!(self.lexer => Distinct);
        let (all, exprs) = if optional_match_token!(self.lexer, Star) {
            (true, vec![])
        } else {
            (false, self.parse_named_exprs(false)?)
        };
        let orderby = if optional_match_token!(self.lexer => Order) {
            self.parse_orderby()?
        } else {
            vec![]
        };
        let skip = if optional_match_token!(self.lexer => Skip) {
            let skip = Arc::new(self.parse_expr(false)?);
            match skip.root().data() {
                ExprIR::Integer(i) => {
                    if *i < 0 {
                        return Err(self.lexer.format_error(
                            "SKIP specified value of invalid type, must be a positive integer",
                        ));
                    }
                }
                ExprIR::Parameter(_) => {}
                _ => {
                    return Err(self.lexer.format_error(
                        "SKIP specified value of invalid type, must be a positive integer",
                    ));
                }
            }
            Some(skip)
        } else {
            None
        };
        let limit = if optional_match_token!(self.lexer => Limit) {
            let limit = Arc::new(self.parse_expr(false)?);
            match limit.root().data() {
                ExprIR::Integer(i) => {
                    if *i < 0 {
                        return Err(self.lexer.format_error(
                            "LIMIT specified value of invalid type, must be a positive integer",
                        ));
                    }
                }
                ExprIR::Parameter(_) => {}
                _ => {
                    return Err(self.lexer.format_error(
                        "LIMIT specified value of invalid type, must be a positive integer",
                    ));
                }
            }
            Some(limit)
        } else {
            None
        };

        Ok(QueryIR::Return {
            distinct,
            all,
            exprs,
            copy_from_parent: Vec::new(),
            orderby,
            skip,
            limit,
            write,
        })
    }

    fn parse_pattern(
        &mut self,
        clause: &Keyword,
    ) -> Result<QueryGraph<Arc<String>, Arc<String>, Arc<String>>, String> {
        let mut query_graph = QueryGraph::default();
        let mut nodes_alias = HashSet::new();
        loop {
            if let Ok(ident) = self.parse_ident() {
                match_token!(self.lexer, Equal);
                let mut vars = vec![];
                let mut left = self.parse_node_pattern()?;
                vars.push(left.alias.clone());
                if nodes_alias.insert(left.alias.clone()) {
                    query_graph.add_node(left.clone());
                }
                loop {
                    if let Token::Dash | Token::LessThan = self.lexer.current()? {
                        let (relationship, right) =
                            self.parse_relationship_pattern(left, clause)?;
                        vars.push(relationship.alias.clone());
                        vars.push(right.alias.clone());
                        left = right.clone();
                        if !query_graph.add_relationship(relationship.clone())
                            && clause == &Keyword::Match
                        {
                            return Err(format!(
                                "Cannot use the same relationship variable '{}' for multiple patterns.",
                                relationship.alias.as_str()
                            ));
                        }
                        if nodes_alias.insert(right.alias.clone()) {
                            query_graph.add_node(right);
                        }
                    } else {
                        query_graph.add_path(Arc::new(QueryPath::new(ident, vars)));
                        break;
                    }
                }
            } else {
                let mut left = self.parse_node_pattern()?;

                if nodes_alias.insert(left.alias.clone()) {
                    query_graph.add_node(left.clone());
                }
                while let Token::Dash | Token::LessThan = self.lexer.current()? {
                    let (relationship, right) = self.parse_relationship_pattern(left, clause)?;
                    left = right.clone();
                    if !query_graph.add_relationship(relationship.clone()) {
                        if clause == &Keyword::Match {
                            return Err(format!(
                                "Cannot use the same relationship variable '{}' for multiple patterns.",
                                relationship.alias.as_str()
                            ));
                        }
                        if clause == &Keyword::Create {
                            return Err(format!(
                                "The bound variable '{}' can't be redeclared in a CREATE clause",
                                relationship.alias.as_str()
                            ));
                        }
                    }
                    if nodes_alias.insert(right.alias.clone()) {
                        query_graph.add_node(right);
                    }
                }
            }

            if *clause == Keyword::Merge {
                break;
            }

            match self.lexer.current()? {
                Token::Comma => {
                    self.lexer.next();
                }
                Token::Keyword(token, _) => {
                    if token == *clause {
                        self.lexer.next();
                        continue;
                    }
                    break;
                }
                _ => break,
            }
        }

        Ok(query_graph)
    }

    fn parse_case_expression(&mut self) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        self.lexer.next();
        let mut children = vec![];
        if let Token::Keyword(Keyword::When, _) = self.lexer.current()? {
        } else {
            children.push(self.parse_expr(false)?);
        }
        let mut conditions = vec![];
        while optional_match_token!(self.lexer => When) {
            conditions.push(self.parse_expr(false)?);
            match_token!(self.lexer => Then);
            conditions.push(self.parse_expr(false)?);
        }
        if conditions.is_empty() {
            return Err(self.lexer.format_error("Invalid input"));
        }
        children.push(tree!(ExprIR::List ; conditions));
        if optional_match_token!(self.lexer => Else) {
            children.push(self.parse_expr(false)?);
        } else {
            children.push(tree!(ExprIR::Null));
        }
        match_token!(self.lexer => End);
        Ok(tree!(
            ExprIR::FuncInvocation(get_functions().get("case", &FnType::Internal)?); children
        ))
    }

    fn parse_quantifier_expr(
        &mut self,
        allow_pattern_predicate: bool,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let quantifier_type = match self.lexer.current()? {
            Token::Keyword(Keyword::All, _) => {
                self.lexer.next();
                QuantifierType::All
            }
            Token::Keyword(Keyword::Any, _) => {
                self.lexer.next();
                QuantifierType::Any
            }
            Token::Keyword(Keyword::None, _) => {
                self.lexer.next();
                QuantifierType::None
            }
            Token::Keyword(Keyword::Single, _) => {
                self.lexer.next();
                QuantifierType::Single
            }
            _ => unreachable!(),
        };

        match_token!(self.lexer, LParen);
        let var = self.parse_ident()?;
        match_token!(self.lexer => In);
        let expr = self.parse_expr(allow_pattern_predicate)?;
        if !optional_match_token!(self.lexer => Where) {
            return Err(self.lexer.format_error(&format!(
                "'{quantifier_type:?}' function requires a WHERE predicate"
            )));
        }
        let condition = self.parse_expr(allow_pattern_predicate)?;
        match_token!(self.lexer, RParen);
        Ok(tree!(
            ExprIR::Quantifier(quantifier_type, var),
            expr,
            condition
        ))
    }

    #[allow(clippy::too_many_lines)]
    fn parse_primary_expr(
        &mut self,
        allow_pattern_predicate: bool,
    ) -> Result<(DynTree<ExprIR<Arc<String>>>, bool), String> {
        match self.lexer.current()? {
            Token::Ident(_) => {
                let state = self.save_state();
                let ident = self.parse_dotted_ident()?;
                if optional_match_token!(self.lexer, LParen) {
                    let func = get_functions()
                        .get(&ident, &FnType::Function)
                        .or_else(|_| {
                            get_functions().get(&ident, &FnType::Aggregation(Value::Null, None))
                        })?;

                    let distinct = optional_match_token!(self.lexer => Distinct);

                    if func.is_aggregate() {
                        if optional_match_token!(self.lexer, Star) {
                            if func.name != "count" {
                                return Err(self.lexer.format_error(
                                    "COUNT is the only function which can accept * as an argument",
                                ));
                            }
                            if distinct {
                                return Err(self
                                    .lexer
                                    .format_error("COUNT(DISTINCT *) is not supported"));
                            }
                            // Create args array like count(x) does
                            let mut args = vec![tree!(ExprIR::Integer(1))]; // Dummy value for count(*)

                            if distinct {
                                args = vec![tree!(ExprIR::Distinct; args)];
                            }

                            args.push(tree!(ExprIR::Variable(Arc::new(String::from(
                                "__agg_order_by_placeholder__"
                            )))));

                            match_token!(self.lexer, RParen);
                            return Ok((tree!(ExprIR::FuncInvocation(func); args), false));
                        }

                        let mut args = self.parse_expression_list(
                            ExpressionListType::ZeroOrMoreClosedBy(RParen),
                            allow_pattern_predicate,
                        )?;
                        func.validate(args.len())?;

                        // Check for nested aggregate functions
                        for arg in &args {
                            if Self::contains_nested_aggregate(arg) {
                                return Err(self.lexer.format_error(
                                    "Can't use aggregate functions inside of aggregate functions",
                                ));
                            }
                        }

                        if distinct {
                            args = vec![tree!(ExprIR::Distinct; args)];
                        }
                        args.push(tree!(ExprIR::Variable(Arc::new(String::from(
                            "__agg_order_by_placeholder__"
                        )))));
                        return Ok((tree!(ExprIR::FuncInvocation(func); args), false));
                    }

                    let args = self.parse_expression_list(
                        ExpressionListType::ZeroOrMoreClosedBy(RParen),
                        allow_pattern_predicate,
                    )?;
                    func.validate(args.len())?;
                    if distinct && args.is_empty() {
                        return Err(self.lexer.format_error(
                            "DISTINCT can only be used with function calls that have arguments",
                        ));
                    }
                    return Ok((tree!(ExprIR::FuncInvocation(func); args), false));
                }
                self.restore_state(state);
                let ident = self.parse_ident()?;
                Ok((tree!(ExprIR::Variable(ident)), false))
            }
            Token::Parameter(param) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Parameter(param)), false))
            }
            Token::Keyword(Keyword::Case, _) => Ok((self.parse_case_expression()?, false)),
            Token::Keyword(Keyword::All | Keyword::Any | Keyword::None | Keyword::Single, _) => {
                Ok((self.parse_quantifier_expr(allow_pattern_predicate)?, false))
            }

            Token::Keyword(Keyword::Null, _) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Null), false))
            }
            Token::Keyword(Keyword::True, _) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Bool(true)), false))
            }
            Token::Keyword(Keyword::False, _) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Bool(false)), false))
            }
            Token::Integer(i) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Integer(i)), false))
            }
            Token::Float(f) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Float(f)), false))
            }
            Token::String(s) => {
                self.lexer.next();
                Ok((tree!(ExprIR::String(s)), false))
            }
            Token::LBrace => {
                self.lexer.next();
                self.parse_list_literal_or_comprehension(allow_pattern_predicate)
            }
            Token::LBracket => Ok((self.parse_map()?, false)),
            Token::LParen => {
                let checkpoint = self.save_state();
                // Try to detect pattern predicate: (ident)--(...) or (ident)<--(...)
                if allow_pattern_predicate
                    && let Ok(pattern) = self.parse_pattern(&Keyword::Match)
                    && !pattern.relationships().is_empty()
                {
                    return Ok((tree!(ExprIR::Pattern(pattern)), false));
                }

                // Not a pattern predicate - restore and parse normally
                self.restore_state(checkpoint);
                self.lexer.next(); // re-consume LParen
                let expr = tree!(ExprIR::Paren);
                Ok((expr, true))
            }
            token => Err(self.lexer.format_error(&format!("Invalid input {token:?}"))),
        }
    }

    // match one of those kind [..4], [4..], [4..5], [6]
    fn parse_list_operator_expression(
        &mut self,
        mut lhs: DynTree<ExprIR<Arc<String>>>,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let from = self.parse_expr(false);
        if optional_match_token!(self.lexer, DotDot) {
            let to = self.parse_expr(false);
            match_token!(self.lexer, RBrace);
            lhs = tree!(
                ExprIR::GetElements,
                lhs,
                from.unwrap_or_else(|_| tree!(ExprIR::Integer(0))),
                to.unwrap_or_else(|_| tree!(ExprIR::Integer(i64::MAX)))
            );
        } else {
            match_token!(self.lexer, RBrace);
            lhs = tree!(ExprIR::GetElement, lhs, from?);
        }

        Ok(lhs)
    }

    fn parse_property_lookup(
        &mut self,
        expr: DynTree<ExprIR<Arc<String>>>,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let ident = self.parse_ident()?;
        Ok(tree!(ExprIR::Property(ident), expr))
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn parse_expr(
        &mut self,
        allow_pattern_predicate: bool,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let mut stack = vec![(0, None::<DynTree<ExprIR<Arc<String>>>>)];
        while let Some((current, res)) = stack.pop() {
            let Some(res) = res else {
                if current < 3 || (current > 3 && current < 9) || current == 10 {
                    stack.push((current, None));
                    stack.push((current + 1, None));
                } else if current == 3 {
                    // Not
                    let mut not_count = 0;
                    while let Token::Keyword(Keyword::Not, _) = self.lexer.current()? {
                        self.lexer.next();
                        not_count += 1;
                    }
                    let res = if not_count % 2 == 1 {
                        Some(tree!(ExprIR::Not))
                    } else {
                        None
                    };
                    stack.push((current, res));
                    stack.push((current + 1, None));
                } else if current == 9 {
                    // unary add or subtract
                    optional_match_token!(self.lexer, Plus);
                    let is_negate = optional_match_token!(self.lexer, Dash);

                    // Handle integer overflow with negation
                    if is_negate && let Err(err) = self.lexer.current() {
                        return Err(err.replace("Integer overflow '", "Integer overflow '-"));
                    }

                    let res = if is_negate {
                        Some(tree!(ExprIR::Negate))
                    } else {
                        None
                    };
                    stack.push((current, res));
                    stack.push((current + 1, None));
                } else {
                    // primary expression
                    let (res, recurse) = self.parse_primary_expr(allow_pattern_predicate)?;
                    if recurse {
                        stack.push((current, Some(res)));
                        stack.push((0, None));
                        continue;
                    }
                    parse_expr_return!(stack, res);
                }
                continue;
            };
            match current {
                0 => {
                    // Or
                    parse_operators!(self, stack, res, current, Token::Keyword(Keyword::Or, _) => Or);
                }
                1 => {
                    // Xor
                    parse_operators!(self, stack, res, current, Token::Keyword(Keyword::Xor, _) => Xor);
                }
                2 => {
                    // And
                    parse_operators!(self, stack, res, current, Token::Keyword(Keyword::And, _) => And);
                }
                3 => {
                    // Not
                    parse_expr_return!(stack, res);
                }
                4 => {
                    // Comparison
                    parse_operators!(self, stack, res, current, Token::Equal => Eq, Token::NotEqual => Neq, Token::LessThan => Lt, Token::LessThanOrEqual => Le, Token::GreaterThan => Gt, Token::GreaterThanOrEqual => Ge);
                }
                5 => {
                    // String, List, Null predicates
                    let mut res = res;
                    match self.lexer.current()? {
                        Token::Keyword(Keyword::In, _) => {
                            self.lexer.next();
                            res = tree!(ExprIR::In, res);
                        }
                        Token::Keyword(Keyword::Starts, _) => {
                            self.lexer.next();
                            match_token!(self.lexer => With);
                            res = tree!(
                                ExprIR::FuncInvocation(
                                    get_functions().get("starts_with", &FnType::Internal)?,
                                ),
                                res
                            );
                        }
                        Token::Keyword(Keyword::Ends, _) => {
                            self.lexer.next();
                            match_token!(self.lexer => With);
                            res = tree!(
                                ExprIR::FuncInvocation(
                                    get_functions().get("ends_with", &FnType::Internal)?,
                                ),
                                res
                            );
                        }
                        Token::Keyword(Keyword::Contains, _) => {
                            self.lexer.next();
                            res = tree!(
                                ExprIR::FuncInvocation(
                                    get_functions().get("contains", &FnType::Internal)?,
                                ),
                                res
                            );
                        }
                        Token::RegexMatches => {
                            self.lexer.next();
                            res = tree!(
                                ExprIR::FuncInvocation(
                                    get_functions().get("regex_matches", &FnType::Internal)?,
                                ),
                                res
                            );
                        }
                        Token::Keyword(Keyword::Is, _) => {
                            while optional_match_token!(self.lexer => Is) {
                                let is_not =
                                    tree!(ExprIR::Bool(optional_match_token!(self.lexer => Not)));
                                match_token!(self.lexer => Null);
                                res = tree!(
                                    ExprIR::FuncInvocation(
                                        get_functions().get("is_null", &FnType::Internal)?
                                    ),
                                    is_not,
                                    res
                                );
                            }
                            parse_expr_return!(stack, res);
                            continue;
                        }
                        // Negated predicates: peek after NOT to decide
                        // whether it negates a predicate keyword.
                        //
                        // For recognized predicates we use a double-push:
                        //   1. Push Not() wrapper onto the stack.
                        //   2. Set `res` to the inner predicate with `lhs`
                        //      already attached.
                        // When the rhs returns from levels 6+, it becomes a
                        // child of the predicate, which in turn becomes a
                        // child of Not():
                        //   `x NOT IN [1,2]` → Not(In(x, [1,2]))
                        //
                        // Bare NOT (e.g. `u.v NOT NULL`) produces a binary
                        // Not(lhs, rhs) which the binder rejects.
                        Token::Keyword(Keyword::Not, _) => {
                            self.lexer.next();
                            match self.lexer.current()? {
                                // x NOT IN [1, 2, 3]
                                Token::Keyword(Keyword::In, _) => {
                                    self.lexer.next();
                                    stack.push((current, Some(tree!(ExprIR::Not))));
                                    res = tree!(ExprIR::In, res);
                                }
                                // name NOT STARTS WITH 'A'
                                Token::Keyword(Keyword::Starts, _) => {
                                    self.lexer.next();
                                    match_token!(self.lexer => With);
                                    stack.push((current, Some(tree!(ExprIR::Not))));
                                    res = tree!(
                                        ExprIR::FuncInvocation(
                                            get_functions()
                                                .get("starts_with", &FnType::Internal)?,
                                        ),
                                        res
                                    );
                                }
                                // name NOT ENDS WITH 'z'
                                Token::Keyword(Keyword::Ends, _) => {
                                    self.lexer.next();
                                    match_token!(self.lexer => With);
                                    stack.push((current, Some(tree!(ExprIR::Not))));
                                    res = tree!(
                                        ExprIR::FuncInvocation(
                                            get_functions().get("ends_with", &FnType::Internal)?,
                                        ),
                                        res
                                    );
                                }
                                // name NOT CONTAINS 'foo'
                                Token::Keyword(Keyword::Contains, _) => {
                                    self.lexer.next();
                                    stack.push((current, Some(tree!(ExprIR::Not))));
                                    res = tree!(
                                        ExprIR::FuncInvocation(
                                            get_functions().get("contains", &FnType::Internal)?,
                                        ),
                                        res
                                    );
                                }
                                // Bare NOT without a recognized predicate keyword
                                // (e.g. `u.v NOT NULL` instead of `u.v IS NOT NULL`).
                                _ => {
                                    return Err(self
                                        .lexer
                                        .format_error("Invalid usage of 'NOT' filter"));
                                }
                            }
                        }
                        _ => {
                            parse_expr_return!(stack, res);
                            continue;
                        }
                    }
                    stack.push((current, Some(res)));
                    stack.push((current + 1, None));
                }
                6 => {
                    // Add, Sub
                    parse_operators!(self, stack, res, current, Token::Plus => Add, Token::Dash => Sub);
                }
                7 => {
                    // Mul, Div, Modulo
                    parse_operators!(self, stack, res, current, Token::Star => Mul, Token::Slash => Div, Token::Modulo => Modulo);
                }
                8 => {
                    // Power
                    parse_operators!(self, stack, res, current, Token::Power => Pow);
                }
                9 => {
                    // unary add or subtract
                    if matches!(res.root().data(), ExprIR::Negate)
                        && matches!(res.root().child(0).data(), ExprIR::Integer(i64::MIN))
                    {
                        let res = tree!(ExprIR::Integer(i64::MIN));
                        parse_expr_return!(stack, res);
                        continue;
                    } else if matches!(res.root().data(), ExprIR::Integer(i64::MIN)) {
                        // This case should not happen with proper error handling
                        // i64::MIN without negation means the literal was i64::MAX + 1
                        return Err(format!(
                            "Integer overflow '{}'",
                            9_223_372_036_854_775_808_u64
                        ));
                    }
                    parse_expr_return!(stack, res);
                }
                10 => {
                    // None arithmetic operators
                    let mut res = res;
                    loop {
                        match self.lexer.current()? {
                            Token::LBrace => {
                                self.lexer.next();
                                res = self.parse_list_operator_expression(res)?;
                            }
                            Token::Dot => {
                                self.lexer.next();
                                res = self.parse_property_lookup(res)?;
                            }
                            Token::LBracket => {
                                self.lexer.next();
                                res = self.parse_map_projection(res)?;
                            }
                            _ => break,
                        }
                    }
                    if self.lexer.current()? == Token::Colon {
                        let labels = tree!(ExprIR::List; self.parse_labels()?.into_iter().map(|l| tree!(ExprIR::String(l))));
                        res = tree!(
                            ExprIR::FuncInvocation(
                                get_functions().get("hasLabels", &FnType::Function)?
                            ),
                            res,
                            labels
                        );
                    }
                    parse_expr_return!(stack, res);
                }
                11 => {
                    // primary expression
                    let mut res = res;
                    if matches!(res.root().data(), ExprIR::Paren) {
                        match_token!(self.lexer, RParen);
                        if matches!(res.root().child(0).data(), ExprIR::Paren) {
                            res.root_mut().child_mut(0).take_out();
                        }
                    } else if matches!(res.root().data(), ExprIR::List) {
                        if optional_match_token!(self.lexer, Comma) {
                            stack.push((current, Some(res)));
                            stack.push((0, None));
                            continue;
                        }
                        match_token!(self.lexer, RBrace);
                    }
                    parse_expr_return!(stack, res);
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    }

    /// Match a dot separator in index property references (e.g. `n.prop`).
    /// Handles the edge case where `.1` is lexed as a float token instead of
    /// dot + integer, producing "expected a property name" in that case.
    fn match_dot_property_separator(&mut self) -> Result<(), String> {
        match self.lexer.current()? {
            Token::Dot => {
                self.lexer.next();
                Ok(())
            }
            Token::Float(_) if self.lexer.current_str().starts_with('.') => {
                Err(self.lexer.format_error(&format!(
                    "Invalid input '{}': expected a property name",
                    &self.lexer.current_str()[1..],
                )))
            }
            _ => Err(self.lexer.format_error(&format!(
                "Invalid input '{}': expected '.'",
                self.lexer.current_str(),
            ))),
        }
    }

    fn parse_ident(&mut self) -> Result<Arc<String>, String> {
        match self.lexer.current() {
            Ok(Token::Ident(id) | Token::Keyword(_, id)) => {
                self.lexer.next();
                Ok(id)
            }
            _ => Err(self.lexer.format_error(&format!(
                "Invalid input '{}': expected an identifier",
                self.lexer.current_str(),
            ))),
        }
    }

    fn parse_property_name(&mut self) -> Result<Arc<String>, String> {
        match self.lexer.current() {
            Ok(Token::Ident(id) | Token::Keyword(_, id)) => {
                self.lexer.next();
                Ok(id)
            }
            _ => Err(self.lexer.format_error(&format!(
                "Invalid input '{}': expected a property name",
                self.lexer.current_str(),
            ))),
        }
    }

    fn parse_named_exprs(
        &mut self,
        must_alias: bool,
    ) -> Result<Vec<(Arc<String>, QueryExpr<Arc<String>>)>, String> {
        let mut named_exprs = Vec::new();
        loop {
            let pos = self.lexer.pos(false);
            let expr = Arc::new(self.parse_expr(true)?);
            if let Token::Keyword(Keyword::As, _) = self.lexer.current()? {
                self.lexer.next();
                let ident = self.parse_ident()?;
                named_exprs.push((ident, expr));
            } else if let ExprIR::Variable(id) = expr.root().data() {
                named_exprs.push((id.clone(), expr));
            } else {
                if must_alias {
                    return Err(self
                        .lexer
                        .format_error("WITH clause projections must be aliased"));
                }
                named_exprs.push((
                    Arc::new(String::from(&self.lexer.str[pos..self.lexer.pos(true)])),
                    expr,
                ));
            }
            match self.lexer.current()? {
                Token::Comma => self.lexer.next(),
                _ => return Ok(named_exprs),
            }
        }
    }

    fn parse_expression_list(
        &mut self,
        expression_list_type: ExpressionListType,
        allow_pattern_predicate: bool,
    ) -> Result<Vec<DynTree<ExprIR<Arc<String>>>>, String> {
        let mut exprs = Vec::new();
        while !expression_list_type.is_end_token(&self.lexer.current()?) {
            exprs.push(self.parse_expr(allow_pattern_predicate)?);
            match self.lexer.current()? {
                Token::Comma => self.lexer.next(),
                _ => break,
            }
        }

        if let ExpressionListType::ZeroOrMoreClosedBy(token) = expression_list_type {
            if self.lexer.current()? == token {
                self.lexer.next();
            } else {
                return Err(self.lexer.format_error(&format!("Invalid input {token:?}")));
            }
        }
        Ok(exprs)
    }

    /// Parses the contents after an opening `[` bracket.
    ///
    /// Uses backtracking to distinguish between:
    /// 1. List comprehension: `[var IN list WHERE cond | expr]`
    /// 2. Named pattern comprehension: `[var = (pattern) WHERE cond | expr]`
    /// 3. Unnamed pattern comprehension: `[(pattern) WHERE cond | expr]`
    /// 4. List literal: `[1, 2, 3]` or `[]`
    ///
    /// Returns `(tree, recurse)` where `recurse` indicates the caller must
    /// continue parsing comma-separated elements for a list literal.
    fn parse_list_literal_or_comprehension(
        &mut self,
        allow_pattern_predicate: bool,
    ) -> Result<(DynTree<ExprIR<Arc<String>>>, bool), String> {
        let saved = self.save_state();

        // 1) Try list comprehension: [var IN ...]
        if let Ok(var) = self.parse_ident()
            && optional_match_token!(self.lexer => In)
        {
            return Ok((
                self.parse_list_comprehension(var, allow_pattern_predicate)?,
                false,
            ));
        }
        self.restore_state(saved);

        // 2) Try named pattern comprehension: [var = (pattern) ... | expr]
        if let Ok(var) = self.parse_ident()
            && optional_match_token!(self.lexer, Equal)
            && self.lexer.current()? == Token::LParen
            && let Ok(result) = self.parse_pattern_comprehension(Some(var), allow_pattern_predicate)
        {
            return Ok((result, false));
        }
        self.restore_state(saved);

        // 3) Try unnamed pattern comprehension: [(pattern) ... | expr]
        if self.lexer.current()? == Token::LParen {
            if let Ok(result) = self.parse_pattern_comprehension(None, allow_pattern_predicate) {
                return Ok((result, false));
            }
            self.restore_state(saved);
        }

        // 4) Default: list literal
        Ok((
            tree!(ExprIR::List),
            !optional_match_token!(self.lexer, RBrace),
        ))
    }

    fn parse_list_comprehension(
        &mut self,
        var: Arc<String>,
        allow_pattern_predicate: bool,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        // var and 'IN' already parsed
        let list_expr = self.parse_expr(allow_pattern_predicate)?;

        let condition = if optional_match_token!(self.lexer => Where) {
            Some(self.parse_expr(allow_pattern_predicate)?)
        } else {
            None
        };

        let expression = if optional_match_token!(self.lexer, Pipe) {
            Some(self.parse_expr(allow_pattern_predicate)?)
        } else {
            None
        };

        match_token!(self.lexer, RBrace);

        Ok(tree!(
            ExprIR::ListComprehension(var.clone()),
            list_expr,
            condition.unwrap_or_else(|| tree!(ExprIR::Bool(true))),
            expression.map_or_else(|| Ok::<_, String>(tree!(ExprIR::Variable(var))), Ok)?
        ))
    }

    /// Parses a pattern comprehension after the opening `[`.
    ///
    /// Grammar: `[` (path_var `=`)? relationship_pattern (`WHERE` cond)? `|` expr `]`
    ///
    /// Collects the graph pattern into a `PatternComprehension` node
    /// for execution by the runtime.
    fn parse_pattern_comprehension(
        &mut self,
        path_var: Option<Arc<String>>,
        allow_pattern_predicate: bool,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let first_node = self.parse_node_pattern()?;

        // Must have at least one relationship
        if !matches!(self.lexer.current()?, Token::Dash | Token::LessThan) {
            return Err("Expected relationship pattern".into());
        }

        let mut graph = QueryGraph::default();
        graph.add_node(first_node.clone());

        let mut left = first_node;
        while matches!(self.lexer.current()?, Token::Dash | Token::LessThan) {
            let (rel, right) = self.parse_relationship_pattern(left, &Keyword::Match)?;
            graph.add_node(right.clone());
            graph.add_relationship(rel);
            left = right;
        }

        if let Some(pv) = path_var {
            graph.add_path(Arc::new(QueryPath::new(pv, vec![])));
        }

        // Optional WHERE
        let condition = if optional_match_token!(self.lexer => Where) {
            self.parse_expr(allow_pattern_predicate)?
        } else {
            tree!(ExprIR::Bool(true))
        };

        // Pipe + result expression
        match_token!(self.lexer, Pipe);
        let result_expr = self.parse_expr(allow_pattern_predicate)?;
        match_token!(self.lexer, RBrace);

        Ok(tree!(
            ExprIR::PatternComprehension(graph),
            condition,
            result_expr
        ))
    }

    /// Parses an inline property map, preserving "Unknown function" errors
    /// while replacing other parse errors with a generic inlined-properties message.
    fn parse_inline_properties(&mut self) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        self.parse_map().map_err(|e| {
            if e.starts_with("Unknown function") {
                e
            } else {
                String::from("Encountered unhandled type in inlined properties.")
            }
        })
    }

    fn parse_node_pattern(&mut self) -> Result<Arc<QueryNode<Arc<String>, Arc<String>>>, String> {
        match_token!(self.lexer, LParen);
        let alias = if let Ok(id) = self.parse_ident() {
            id
        } else {
            let name = Arc::new(format!("_anon_{}", self.anon_counter));
            self.anon_counter += 1;
            name
        };
        let labels = self.parse_labels()?;
        let attrs = if let Token::Parameter(param) = self.lexer.current()? {
            self.lexer.next();
            tree!(ExprIR::Parameter(param))
        } else if self.lexer.current()? == Token::LBracket {
            self.parse_inline_properties()?
        } else {
            tree!(ExprIR::Map)
        };
        match_token!(self.lexer, RParen);
        Ok(Arc::new(QueryNode::new(alias, labels, Arc::new(attrs))))
    }

    #[allow(clippy::too_many_lines)]
    fn parse_relationship_pattern(
        &mut self,
        src: Arc<QueryNode<Arc<String>, Arc<String>>>,
        clause: &Keyword,
    ) -> Result<
        (
            Arc<QueryRelationship<Arc<String>, Arc<String>, Arc<String>>>,
            Arc<QueryNode<Arc<String>, Arc<String>>>,
        ),
        String,
    > {
        let is_incoming = optional_match_token!(self.lexer, LessThan);
        match_token!(self.lexer, Dash);
        let has_details = optional_match_token!(self.lexer, LBrace);
        let (alias, types, attrs, var_len) = if has_details {
            let alias = if let Ok(id) = self.parse_ident() {
                id
            } else {
                let name = Arc::new(format!("_anon_{}", self.anon_counter));
                self.anon_counter += 1;
                name
            };
            let mut types = HashSet::new();
            if optional_match_token!(self.lexer, Colon) {
                loop {
                    types.insert(self.parse_ident()?);
                    let pipe = optional_match_token!(self.lexer, Pipe);
                    let colon = optional_match_token!(self.lexer, Colon);
                    if pipe || colon {
                        continue;
                    }
                    break;
                }
            }
            let var_len = if optional_match_token!(self.lexer, Star) {
                let start = if let Token::Integer(i) = self.lexer.current()? {
                    self.lexer.next();
                    Some(i)
                } else {
                    None
                };
                if optional_match_token!(self.lexer, DotDot) {
                    let end = if let Token::Integer(i) = self.lexer.current()? {
                        self.lexer.next();
                        Some(i)
                    } else {
                        None
                    };
                    // [*a..b] → min=a (default 1), max=b (default None=infinity)
                    Some((start.unwrap_or(1) as u32, end.map(|e| e as u32)))
                } else if let Some(exact) = start {
                    // [*N] → exact N hops
                    Some((exact as u32, Some(exact as u32)))
                } else {
                    // [*] → 1..infinity
                    Some((1, None))
                }
            } else {
                None
            };
            if var_len.is_some() && matches!(clause, Keyword::Create | Keyword::Merge) {
                let clause_name = if *clause == Keyword::Create {
                    "CREATE"
                } else {
                    "MERGE"
                };
                return Err(format!(
                    "Variable length relationships cannot be used in {clause_name} patterns."
                ));
            }
            let attrs = if let Token::Parameter(param) = self.lexer.current()? {
                self.lexer.next();
                tree!(ExprIR::Parameter(param))
            } else if self.lexer.current()? == Token::LBracket {
                self.parse_inline_properties()?
            } else {
                tree!(ExprIR::Map)
            };
            match_token!(self.lexer, RBrace);
            (alias, types.into_iter().collect(), attrs, var_len)
        } else {
            let name = Arc::new(format!("_anon_{}", self.anon_counter));
            self.anon_counter += 1;
            (name, vec![], tree!(ExprIR::Map), None)
        };
        match_token!(self.lexer, Dash);
        let is_outgoing = optional_match_token!(self.lexer, GreaterThan);
        let dst = self.parse_node_pattern()?;
        let (min_hops, max_hops) = match var_len {
            Some((min, max)) => (Some(min), max),
            None => (None, None),
        };
        let relationship = match (is_incoming, is_outgoing) {
            (true, true) | (false, false) => {
                if *clause == Keyword::Create {
                    return Err(self
                        .lexer
                        .format_error("Only directed relationships are supported in CREATE"));
                }
                QueryRelationship::new(
                    alias,
                    types,
                    Arc::new(attrs),
                    src,
                    dst.clone(),
                    true,
                    min_hops,
                    max_hops,
                )
            }
            (true, false) => QueryRelationship::new(
                alias,
                types,
                Arc::new(attrs),
                dst.clone(),
                src,
                false,
                min_hops,
                max_hops,
            ),
            (false, true) => QueryRelationship::new(
                alias,
                types,
                Arc::new(attrs),
                src,
                dst.clone(),
                false,
                min_hops,
                max_hops,
            ),
        };
        Ok((Arc::new(relationship), dst))
    }

    fn parse_labels(&mut self) -> Result<OrderSet<Arc<String>>, String> {
        let mut labels = OrderSet::default();
        while self.lexer.current()? == Token::Colon {
            self.lexer.next();
            labels.insert(self.parse_ident()?);
        }
        Ok(labels)
    }

    fn parse_map(&mut self) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let mut attrs = Vec::new();
        if self.lexer.current()? == Token::LBracket {
            self.lexer.next();
        } else {
            return Ok(tree!(ExprIR::Map));
        }

        if self.lexer.current() == Ok(Token::RBracket) {
            self.lexer.next();
            return Ok(tree!(ExprIR::Map));
        }

        loop {
            let key = self.parse_ident()?;
            match_token!(self.lexer, Colon);
            let value = self.parse_expr(false)?;
            attrs.push(tree!(ExprIR::String(key), value));

            match self.lexer.current()? {
                Token::Comma => self.lexer.next(),
                Token::RBracket => {
                    self.lexer.next();
                    return Ok(tree!(ExprIR::Map ; attrs));
                }
                token => {
                    return Err(self.lexer.format_error(&format!("Invalid input {token:?}")));
                }
            }
        }
    }

    fn parse_map_projection(
        &mut self,
        base: DynTree<ExprIR<Arc<String>>>,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
        let mut items = vec![base];

        // Empty projection: expr {}
        if self.lexer.current()? == Token::RBracket {
            self.lexer.next();
            return Ok(tree!(ExprIR::MapProjection ; items));
        }

        loop {
            if self.lexer.current()? == Token::Dot {
                self.lexer.next();
                if optional_match_token!(self.lexer, Star) {
                    // .* - all properties
                    items.push(tree!(ExprIR::MapProjection));
                } else {
                    // .property - property shorthand
                    let prop_name = self.parse_property_name()?;
                    items.push(tree!(ExprIR::Property(prop_name)));
                }
            } else {
                // key: expr  or  variable shorthand
                let ident = self.parse_ident()?;
                if optional_match_token!(self.lexer, Colon) {
                    let value = self.parse_expr(false)?;
                    items.push(tree!(ExprIR::String(ident), value));
                } else {
                    // variable shorthand: name -> name: name
                    items.push(tree!(
                        ExprIR::String(ident.clone()),
                        tree!(ExprIR::Variable(ident))
                    ));
                }
            }

            match self.lexer.current()? {
                Token::Comma => {
                    self.lexer.next();
                }
                Token::RBracket => {
                    self.lexer.next();
                    break;
                }
                _ => {
                    return Err(self.lexer.format_error(&format!(
                        "Invalid input '{}': expected ':', ',' or '}}'",
                        self.lexer.current_str()
                    )));
                }
            }
        }

        Ok(tree!(ExprIR::MapProjection ; items))
    }

    fn parse_orderby(&mut self) -> Result<Vec<(QueryExpr<Arc<String>>, bool)>, String> {
        match_token!(self.lexer => By);
        let mut orderby = vec![];
        loop {
            let expr = Arc::new(self.parse_expr(false)?);
            let is_ascending = optional_match_token!(self.lexer => Asc)
                || optional_match_token!(self.lexer => Ascending);
            let is_descending = !is_ascending
                && (optional_match_token!(self.lexer => Desc)
                    || optional_match_token!(self.lexer => Descending));
            orderby.push((expr, is_descending));
            if !optional_match_token!(self.lexer, Comma) {
                break;
            }
        }
        Ok(orderby)
    }

    fn parse_set_clause(&mut self) -> Result<QueryIR<Arc<String>>, String> {
        let mut set_items = vec![];
        self.parse_set_items(&mut set_items)?;
        Ok(QueryIR::Set(set_items))
    }

    fn parse_set_items(
        &mut self,
        set_items: &mut Vec<SetItem<Arc<String>, Arc<String>>>,
    ) -> Result<(), String> {
        loop {
            let (mut expr, recurse) = self.parse_primary_expr(false)?;
            if recurse {
                expr = self.parse_expr(false)?;
                match_token!(self.lexer, RParen);
            }
            if self.lexer.current()? == Token::Dot {
                while self.lexer.current()? == Token::Dot {
                    self.lexer.next();
                    expr = self.parse_property_lookup(expr)?;
                }
                match_token!(self.lexer, Equal);
                let value = Arc::new(self.parse_expr(false)?);
                set_items.push(SetItem::Attribute(Arc::new(expr), value, false));
            } else if self.lexer.current()? == Token::Colon {
                let ExprIR::Variable(id) = expr.root().data() else {
                    return Err(self
                        .lexer
                        .format_error("Cannot set labels on non-node expressions"));
                };
                set_items.push(SetItem::Label(id.clone(), self.parse_labels()?));
            } else {
                let equals = optional_match_token!(self.lexer, Equal);
                let plus_equals = if equals {
                    false
                } else {
                    match_token!(self.lexer, PlusEqual);
                    true
                };
                let value = Arc::new(self.parse_expr(false)?);
                set_items.push(SetItem::Attribute(Arc::new(expr), value, !plus_equals));
            }

            if !optional_match_token!(self.lexer, Comma) {
                return Ok(());
            }
        }
    }

    fn parse_remove_clause(&mut self) -> Result<QueryIR<Arc<String>>, String> {
        let mut remove_items = vec![];
        loop {
            let (mut expr, recurse) = self.parse_primary_expr(false)?;
            if recurse {
                expr = self.parse_expr(false)?;
                match_token!(self.lexer, RParen);
            }
            if self.lexer.current()? == Token::Dot {
                while self.lexer.current()? == Token::Dot {
                    self.lexer.next();
                    expr = self.parse_property_lookup(expr)?;
                }
                remove_items.push(Arc::new(expr));
            } else if self.lexer.current()? == Token::Colon {
                expr = tree!(
                    ExprIR::FuncInvocation(get_functions().get("hasLabels", &FnType::Function)?),
                    expr,
                    tree!(ExprIR::List; self.parse_labels()?.into_iter().map(|l| tree!(ExprIR::String(l))))
                );
                remove_items.push(Arc::new(expr));
            } else {
                return Err(self
                    .lexer
                    .format_error(&format!("Invalid input '{}'", self.lexer.current_str())));
            }

            if !optional_match_token!(self.lexer, Comma) {
                break;
            }
        }
        Ok(QueryIR::Remove(remove_items))
    }
}
