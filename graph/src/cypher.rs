use crate::ast::{
    Alias, ExprIR, NodePattern, PathPattern, Pattern, QuantifierType, QueryIR, RelationshipPattern,
};
use crate::cypher::Token::RParen;
use crate::functions::{FnType, get_functions};
use crate::tree;
use falkordb_macro::parse_binary_expr;
use orx_tree::{DynTree, NodeRef};
use std::collections::{BTreeMap, HashSet};
use std::num::IntErrorKind;
use std::rc::Rc;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
enum Keyword {
    Call,
    Optional,
    Match,
    Unwind,
    Merge,
    Create,
    Delete,
    Where,
    With,
    Return,
    As,
    Null,
    Or,
    Xor,
    And,
    Not,
    Is,
    In,
    Starts,
    Ends,
    Contains,
    True,
    False,
    Case,
    When,
    Then,
    Else,
    End,
    All,
    Any,
    None,
    Single,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Ident(Rc<String>),
    Keyword(Keyword, Rc<String>),
    Parameter(String),
    Integer(i64),
    Float(f64),
    String(Rc<String>),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Modulo,
    Power,
    Star,
    Slash,
    Plus,
    Dash,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Comma,
    Colon,
    Dot,
    DotDot,
    Pipe,
    RegexMatches,
    Error(String),
    EndOfFile,
}

const KEYWORDS: [(&str, Keyword); 32] = [
    ("CALL", Keyword::Call),
    ("OPTIONAL", Keyword::Optional),
    ("MATCH", Keyword::Match),
    ("UNWIND", Keyword::Unwind),
    ("MERGE", Keyword::Merge),
    ("CREATE", Keyword::Create),
    ("DELETE", Keyword::Delete),
    ("WHERE", Keyword::Where),
    ("WITH", Keyword::With),
    ("RETURN", Keyword::Return),
    ("AS", Keyword::As),
    ("NULL", Keyword::Null),
    ("OR", Keyword::Or),
    ("XOR", Keyword::Xor),
    ("AND", Keyword::And),
    ("NOT", Keyword::Not),
    ("IS", Keyword::Is),
    ("IN", Keyword::In),
    ("STARTS", Keyword::Starts),
    ("ENDS", Keyword::Ends),
    ("CONTAINS", Keyword::Contains),
    ("TRUE", Keyword::True),
    ("FALSE", Keyword::False),
    ("CASE", Keyword::Case),
    ("WHEN", Keyword::When),
    ("THEN", Keyword::Then),
    ("ELSE", Keyword::Else),
    ("END", Keyword::End),
    ("ALL", Keyword::All),
    ("ANY", Keyword::Any),
    ("NONE", Keyword::None),
    ("SINGLE", Keyword::Single),
];

const MIN_I64: [&str; 5] = [
    "0b1000000000000000000000000000000000000000000000000000000000000000", // binary
    "0o1000000000000000000000",                                           // octal
    "01000000000000000000000",                                            // octal
    "9223372036854775808",                                                // decimal
    "0x8000000000000000",                                                 // hex
];

struct Lexer<'a> {
    str: &'a str,
    pos: usize,
    cached_current: (Token, usize),
}

#[derive(Debug)]
enum ExpressionListType {
    OneOrMore,
    ZeroOrMoreClosedBy(Token),
}

impl ExpressionListType {
    fn is_end_token(
        &self,
        current_token: Token,
    ) -> bool {
        match self {
            Self::OneOrMore => false,
            Self::ZeroOrMoreClosedBy(token) => *token == current_token,
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            str,
            pos: Self::read_spaces(str, 0),
            cached_current: Self::get_token(str, Self::read_spaces(str, 0)),
        }
    }

    fn next(&mut self) {
        self.pos += self.cached_current.1;
        self.pos += Self::read_spaces(self.str, self.pos);
        self.cached_current = Self::get_token(self.str, self.pos);
    }

    fn read_spaces(
        str: &'a str,
        pos: usize,
    ) -> usize {
        let mut len = 0;
        let mut chars = str[pos..].chars();
        while let Some(' ' | '\t' | '\n') = chars.next() {
            len += 1;
        }
        len
    }

    fn current(&self) -> Token {
        self.cached_current.0.clone()
    }

    #[inline]
    #[allow(clippy::too_many_lines)]
    fn get_token(
        str: &'a str,
        pos: usize,
    ) -> (Token, usize) {
        let mut chars = str[pos..].chars();
        if let Some(char) = chars.next() {
            return match char {
                '[' => (Token::LBrace, 1),
                ']' => (Token::RBrace, 1),
                '{' => (Token::LBracket, 1),
                '}' => (Token::RBracket, 1),
                '(' => (Token::LParen, 1),
                ')' => (Token::RParen, 1),
                '%' => (Token::Modulo, 1),
                '^' => (Token::Power, 1),
                '*' => (Token::Star, 1),
                '/' => (Token::Slash, 1),
                '+' => (Token::Plus, 1),
                '-' => (Token::Dash, 1),
                '=' => match chars.next() {
                    Some('~') => (Token::RegexMatches, 2),
                    _ => (Token::Equal, 1),
                },
                '<' => match chars.next() {
                    Some('=') => (Token::LessThanOrEqual, 2),
                    Some('>') => (Token::NotEqual, 2),
                    _ => (Token::LessThan, 1),
                },
                '>' => match chars.next() {
                    Some('=') => (Token::GreaterThanOrEqual, 2),
                    _ => (Token::GreaterThan, 1),
                },
                ',' => (Token::Comma, 1),
                ':' => (Token::Colon, 1),
                '.' => match chars.next() {
                    Some('.') => (Token::DotDot, 2),
                    Some('0'..='9') => Self::lex_numeric(str, chars, pos, 2),
                    _ => (Token::Dot, 1),
                },
                '|' => (Token::Pipe, 1),
                '\'' => {
                    let mut len = 1;
                    let mut end = false;
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            len += chars.next().unwrap().len_utf8();
                        } else if c == '\'' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return (
                            Token::Error(String::from(&str[pos + 1..pos + len])),
                            len + 1,
                        );
                    }
                    (
                        Token::String(Rc::new(String::from(&str[pos + 1..pos + len]))),
                        len + 1,
                    )
                }
                '\"' => {
                    let mut len = 1;
                    let mut end = false;
                    for c in chars.by_ref() {
                        if c == '\"' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return (
                            Token::Error(String::from(&str[pos + 1..pos + len])),
                            len + 1,
                        );
                    }
                    (
                        Token::String(Rc::new(String::from(&str[pos + 1..pos + len]))),
                        len + 1,
                    )
                }
                '0'..='9' => Self::lex_numeric(str, chars, pos, 1),
                '$' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = chars.next() {
                        len += 1;
                    }
                    let token = Token::Parameter(String::from(&str[pos + 1..pos + len]));
                    (token, len)
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = chars.next() {
                        len += 1;
                    }

                    let token = KEYWORDS
                        .iter()
                        .find(|&other| str[pos..pos + len].eq_ignore_ascii_case(other.0))
                        .map_or_else(
                            || Token::Ident(Rc::new(String::from(&str[pos..pos + len]))),
                            |o| {
                                Token::Keyword(
                                    o.1.clone(),
                                    Rc::new(String::from(&str[pos..pos + len])),
                                )
                            },
                        );
                    (token, len)
                }
                '`' => {
                    let mut len = 1;
                    let mut end = false;
                    for c in chars.by_ref() {
                        if c == '`' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return (
                            Token::Error(String::from(&str[pos + 1..pos + len])),
                            len + 1,
                        );
                    }
                    (
                        Token::Ident(Rc::new(String::from(&str[pos + 1..pos + len]))),
                        len + 1,
                    )
                }
                _ => (
                    Token::Error(format!("Invalid input at pos: {pos} at char {char}")),
                    0,
                ),
            };
        }
        (Token::EndOfFile, 0)
    }

    fn lex_numeric(
        str: &'a str,
        mut chars: Chars,
        pos: usize,
        mut len: usize,
    ) -> (Token, usize) {
        while let Some(c) = chars.next() {
            if c == '.' && chars.clone().next() == Some('.') {
                break; // Stop when encountering `..`
            }
            if c.is_ascii_alphanumeric() || c == '_' || c == '$' || c == '+' || c == '-' || c == '.'
            {
                len += 1;
            } else {
                break;
            }
        }
        let str = String::from(&str[pos..pos + len]);
        let token = Lexer::str2number_token(&str);
        (token, len)
    }

    fn is_str_float(str: &str) -> bool {
        str.contains('.')
            || str.to_lowercase().contains('e')
                && !(str.starts_with("0x") || str.starts_with("0X"))
                && !(str.starts_with("0b") || str.starts_with("0B"))
                && !(str.starts_with("0o") || str.starts_with("0O"))
    }

    fn str2number_token(str: &str) -> Token {
        if Lexer::is_str_float(str) {
            return match str.parse::<f64>() {
                Ok(f) if f.is_finite() => Token::Float(f),
                Ok(_) => Token::Error(format!("Float overflow '{str}'")),
                Err(_) => Token::Error(format!("Invalid float: {str}")),
            };
        }

        if str.eq_ignore_ascii_case(MIN_I64[0])
            || str.eq_ignore_ascii_case(MIN_I64[1])
            || str.eq_ignore_ascii_case(MIN_I64[2])
            || str.eq_ignore_ascii_case(MIN_I64[3])
            || str.eq_ignore_ascii_case(MIN_I64[4])
        {
            return Token::Integer(i64::MIN);
        }

        let (mut offset, mut radix) = (0, 10);
        if str.starts_with("0x") || str.starts_with("0X") {
            offset = 2;
            radix = 16;
        } else if str.starts_with("0b") || str.starts_with("0B") {
            offset = 2;
            radix = 2;
        } else if str.starts_with("0o") || str.starts_with("0O") {
            offset = 2;
            radix = 8;
        } else if 1 < str.len() && str.starts_with('0') {
            offset = 1;
            radix = 8;
        }
        let number_str = &str[offset..];
        i64::from_str_radix(number_str, radix).map_or_else(
            |err| match err.kind() {
                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => {
                    Token::Error(format!("Integer overflow '{number_str}'"))
                }
                _ => Token::Error(format!("Invalid numeric value '{number_str}'")),
            },
            Token::Integer,
        )
    }

    pub fn format_error(
        &self,
        err: &str,
    ) -> String {
        format!("{}, errCtx: {}, pos {}", err, self.str, self.pos)
    }

    fn set_pos(
        &mut self,
        pos: usize,
    ) {
        self.pos = pos;
        self.cached_current = Self::get_token(self.str, pos);
    }
}

macro_rules! match_token {
    ($lexer:expr, $token:ident) => {
        match $lexer.current() {
            Token::$token => {
                $lexer.next();
            }
            token => return Err($lexer.format_error(&format!("Invalid input {token:?}"))),
        }
    };
    ($lexer:expr => $token:ident) => {
        match $lexer.current() {
            Token::Keyword(Keyword::$token, _) => {
                $lexer.next();
            }
            token => return Err($lexer.format_error(&format!("Invalid input {token:?}"))),
        }
    };
    () => {};
}

macro_rules! optional_match_token {
    ($lexer:expr, $token:ident) => {
        match $lexer.current() {
            Token::$token => {
                $lexer.next();
                true
            }
            _ => false,
        }
    };
    ($lexer:expr => $token:ident) => {
        match $lexer.current() {
            Token::Keyword(Keyword::$token, _) => {
                $lexer.next();
                true
            }
            _ => false,
        }
    };
    () => {};
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    anon_id: i32,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(str: &'a str) -> Self {
        Self {
            lexer: Lexer::new(str),
            anon_id: 0,
        }
    }

    pub fn parse_parameters(
        &mut self
    ) -> Result<(BTreeMap<String, DynTree<ExprIR>>, &'a str), String> {
        match self.lexer.current() {
            Token::Ident(id) => {
                if id.as_str() == "CYPHER" {
                    self.lexer.next();
                    let mut params = BTreeMap::new();
                    while let Token::Ident(id) = self.lexer.current() {
                        self.lexer.next();
                        match_token!(self.lexer, Equal);
                        params.insert(String::from(id.as_str()), self.parse_expr()?);
                    }
                    Ok((params, &self.lexer.str[self.lexer.pos..]))
                } else {
                    Ok((BTreeMap::new(), self.lexer.str))
                }
            }
            _ => Ok((BTreeMap::new(), self.lexer.str)),
        }
    }

    pub fn parse(&mut self) -> Result<QueryIR, String> {
        let mut ir = self.parse_query()?;
        ir.validate()?;
        Ok(ir)
    }

    fn parse_query(&mut self) -> Result<QueryIR, String> {
        let mut clauses = Vec::new();
        let mut write = false;
        loop {
            while let Token::Keyword(
                Keyword::Optional
                | Keyword::Match
                | Keyword::Unwind
                | Keyword::Call
                | Keyword::Where,
                _,
            ) = self.lexer.current()
            {
                clauses.push(self.parse_reading_clasue()?);
            }
            while let Token::Keyword(Keyword::Create | Keyword::Merge | Keyword::Delete, _) =
                self.lexer.current()
            {
                write = true;
                clauses.push(self.parse_writing_clause()?);
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
        }
        if self.lexer.current() != Token::EndOfFile {
            return Err(self
                .lexer
                .format_error(&format!("Invalid input '{:?}'", self.lexer.current())));
        }
        Ok(QueryIR::Query(clauses, write))
    }

    fn parse_reading_clasue(&mut self) -> Result<QueryIR, String> {
        if optional_match_token!(self.lexer => Optional) {
            match_token!(self.lexer => Match);
            return self.parse_match_clause(true);
        }
        match self.lexer.current() {
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
            Token::Keyword(Keyword::Where, _) => {
                self.lexer.next();
                self.parse_where_clause()
            }
            _ => unreachable!(),
        }
    }

    fn parse_writing_clause(&mut self) -> Result<QueryIR, String> {
        match self.lexer.current() {
            Token::Keyword(Keyword::Create, _) => {
                self.lexer.next();
                self.parse_create_clause()
            }
            Token::Keyword(Keyword::Merge, _) => {
                self.lexer.next();
                self.parse_merge_clause()
            }
            Token::Keyword(Keyword::Delete, _) => {
                self.lexer.next();
                self.parse_delete_clause()
            }
            token => Err(self.lexer.format_error(&format!("Invalid input {token:?}"))),
        }
    }

    fn parse_call_clause(&mut self) -> Result<QueryIR, String> {
        let ident = self.parse_dotted_ident()?;
        match_token!(self.lexer, LParen);
        Ok(QueryIR::Call(
            ident,
            self.parse_expression_list(ExpressionListType::ZeroOrMoreClosedBy(RParen))?,
        ))
    }

    fn parse_dotted_ident(&mut self) -> Result<Rc<String>, String> {
        let mut idents = vec![self.parse_ident()?];
        while self.lexer.current() == Token::Dot {
            self.lexer.next();
            idents.push(self.parse_ident()?);
        }
        Ok(Rc::new(
            idents
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join("."),
        ))
    }

    fn parse_match_clause(
        &mut self,
        optional: bool,
    ) -> Result<QueryIR, String> {
        Ok(QueryIR::Match(
            self.parse_pattern(Keyword::Match)?,
            optional,
        ))
    }

    fn parse_unwind_clause(&mut self) -> Result<QueryIR, String> {
        let list = self.parse_expr()?;
        match_token!(self.lexer => As);
        let ident = self.parse_ident()?;
        Ok(QueryIR::Unwind(list, ident))
    }

    fn parse_create_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Create(self.parse_pattern(Keyword::Create)?))
    }

    fn parse_merge_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Merge(self.parse_pattern(Keyword::Merge)?))
    }

    fn parse_delete_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Delete(
            self.parse_expression_list(ExpressionListType::OneOrMore)?,
        ))
    }

    fn parse_where_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Where(self.parse_expr()?))
    }

    fn parse_with_clause(
        &mut self,
        write: bool,
    ) -> Result<QueryIR, String> {
        if optional_match_token!(self.lexer, Star) {
            return Ok(QueryIR::With(vec![], write));
        }
        Ok(QueryIR::With(self.parse_named_exprs()?, write))
    }

    fn parse_return_clause(
        &mut self,
        write: bool,
    ) -> Result<QueryIR, String> {
        Ok(QueryIR::Return(self.parse_named_exprs()?, write))
    }

    fn parse_pattern(
        &mut self,
        clause: Keyword,
    ) -> Result<Pattern, String> {
        let mut nodes = Vec::new();
        let mut nodes_alias = HashSet::new();
        let mut relationships = Vec::new();
        let mut paths = Vec::new();
        loop {
            if let Token::Ident(p) = self.lexer.current() {
                match_token!(self.lexer, Equal);
                let mut vars = Vec::new();
                let left = self.parse_node_pattern()?;
                let mut left_alias = left.alias.clone();
                vars.push(left_alias.clone());
                if nodes_alias.insert(left.alias.to_string()) {
                    nodes.push(left);
                }
                loop {
                    if let Token::Dash | Token::LessThan = self.lexer.current() {
                        let (relationship, right) =
                            self.parse_relationship_pattern(left_alias, &clause)?;
                        vars.push(relationship.alias.clone());
                        vars.push(right.alias.clone());
                        left_alias = right.alias.clone();
                        relationships.push(relationship);
                        if nodes_alias.insert(right.alias.to_string()) {
                            nodes.push(right);
                        }
                    } else {
                        paths.push(PathPattern::new(vars, p));
                        break;
                    }
                }
            } else {
                let left = self.parse_node_pattern()?;
                let mut left_alias = left.alias.clone();

                if nodes_alias.insert(left.alias.to_string()) {
                    nodes.push(left);
                }
                while let Token::Dash | Token::LessThan = self.lexer.current() {
                    let (relationship, right) =
                        self.parse_relationship_pattern(left_alias, &clause)?;
                    left_alias = right.alias.clone();
                    relationships.push(relationship);
                    if nodes_alias.insert(right.alias.to_string()) {
                        nodes.push(right);
                    }
                }
            }

            if clause == Keyword::Merge {
                break;
            }

            match self.lexer.current() {
                Token::Comma => {
                    self.lexer.next();
                }
                Token::Keyword(token, _) => {
                    if token == clause {
                        self.lexer.next();
                        continue;
                    }
                    break;
                }
                _ => break,
            }
        }

        Ok(Pattern::new(nodes, relationships, paths))
    }

    fn parse_case_expression(&mut self) -> Result<DynTree<ExprIR>, String> {
        self.lexer.next();
        let mut children = vec![];
        if let Token::Keyword(Keyword::When, _) = self.lexer.current() {
        } else {
            children.push(self.parse_expr()?);
        }
        let mut params = Vec::new();
        while optional_match_token!(self.lexer => When) {
            let when = self.parse_expr()?;
            match_token!(self.lexer => Then);
            let then = self.parse_expr()?;
            params.push(when);
            params.push(then);
        }
        if optional_match_token!(self.lexer => Else) {
            let else_ = self.parse_expr()?;
            if children.len() == 1 {
                params.push(children[0].clone());
            } else {
                params.push(tree!(ExprIR::Bool(true)));
            }
            params.push(else_);
        }
        match_token!(self.lexer => End);
        if params.is_empty() {
            return Err(self.lexer.format_error("Invalid input"));
        }
        children.push(tree!(ExprIR::List ; params));
        Ok(tree!(
            ExprIR::FuncInvocation(String::from("case"), FnType::Internal); children
        ))
    }

    fn parse_quantifier_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        let quantifier_type = match self.lexer.current() {
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
        let expr = self.parse_expr()?;
        match_token!(self.lexer => Where);
        let condition = self.parse_expr()?;
        match_token!(self.lexer, RParen);
        Ok(tree!(
            ExprIR::Quantifier(quantifier_type, var),
            expr,
            condition
        ))
    }

    fn parse_primary_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        match self.lexer.current() {
            Token::Ident(ident) => {
                let mut namespace_and_function = ident.to_lowercase();
                self.lexer.next();
                let pos = self.lexer.pos;
                while self.lexer.current() == Token::Dot {
                    self.lexer.next();
                    match self.lexer.current() {
                        Token::Ident(id) => {
                            self.lexer.next();
                            namespace_and_function.push('.');
                            namespace_and_function.push_str(&id.to_lowercase());
                        }
                        _ => break,
                    }
                }
                if self.lexer.current() == Token::LParen {
                    self.lexer.next();

                    let is_aggregate = get_functions().is_aggregate(&namespace_and_function);
                    if is_aggregate && optional_match_token!(self.lexer, Star) {
                        match_token!(self.lexer, RParen);
                        return Ok(tree!(ExprIR::FuncInvocation(
                            namespace_and_function,
                            FnType::Aggregation
                        )));
                    }
                    return Ok(tree!(ExprIR::FuncInvocation(
                        namespace_and_function,
                        if is_aggregate { FnType::Aggregation } else { FnType::Function },
                    ); self.parse_expression_list(ExpressionListType::ZeroOrMoreClosedBy(RParen))?));
                }
                self.lexer.set_pos(pos);
                Ok(tree!(ExprIR::Var(ident)))
            }
            Token::Parameter(param) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Parameter(param)))
            }
            Token::Keyword(Keyword::Case, _) => self.parse_case_expression(),
            Token::Keyword(Keyword::All, _)
            | Token::Keyword(Keyword::Any, _)
            | Token::Keyword(Keyword::None, _)
            | Token::Keyword(Keyword::Single, _) => self.parse_quantifier_expr(),

            Token::Keyword(Keyword::Null, _) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Null))
            }
            Token::Keyword(Keyword::True, _) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Bool(true)))
            }
            Token::Keyword(Keyword::False, _) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Bool(false)))
            }
            Token::Integer(i) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Integer(i)))
            }
            Token::Float(f) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Float(f)))
            }
            Token::String(s) => {
                self.lexer.next();
                Ok(tree!(ExprIR::String(s)))
            }
            Token::LBrace => {
                self.lexer.next();
                self.parse_list_literal_or_comprehension()
            }
            Token::LBracket => self.parse_map(),
            Token::LParen => {
                self.lexer.next();
                let expr = self.parse_expr()?;
                match_token!(self.lexer, RParen);
                Ok(expr)
            }
            token => Err(self.lexer.format_error(&format!("Invalid input {token:?}"))),
        }
    }

    fn parse_property_expression(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut expr = self.parse_primary_expr()?;

        while self.lexer.current() == Token::Dot {
            self.lexer.next();
            let ident = self.parse_ident()?;
            expr = tree!(
                ExprIR::FuncInvocation(String::from("property"), FnType::Internal),
                expr,
                tree!(ExprIR::String(ident))
            );
        }

        Ok(expr)
    }

    // match one of those kind [..4], [4..], [4..5], [6]
    fn parse_list_operator_expression(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut expr = self.parse_property_expression()?;

        while self.lexer.current() == Token::LBrace {
            self.lexer.next();

            let from = self.parse_expr();
            if optional_match_token!(self.lexer, DotDot) {
                let to = self.parse_expr();
                match_token!(self.lexer, RBrace);
                expr = tree!(
                    ExprIR::GetElements,
                    expr.clone(),
                    from.unwrap_or_else(|_| tree!(ExprIR::Integer(0))),
                    to.unwrap_or_else(|_| tree!(ExprIR::Length, expr))
                );
            } else {
                match_token!(self.lexer, RBrace);
                expr = tree!(ExprIR::GetElement, expr, from?);
            }
        }

        Ok(expr)
    }

    fn parse_null_operator_expression(&mut self) -> Result<DynTree<ExprIR>, String> {
        let expr = self.parse_list_operator_expression()?;

        match self.lexer.current() {
            Token::Keyword(Keyword::Is, _) => {
                self.lexer.next();
                let is_not = optional_match_token!(self.lexer => Not);
                match self.lexer.current() {
                    Token::Keyword(Keyword::Null, _) => {
                        self.lexer.next();
                        if is_not {
                            return Ok(tree!(ExprIR::Not, tree!(ExprIR::IsNull, expr)));
                        }
                        Ok(tree!(ExprIR::IsNull, expr))
                    }
                    _ => Ok(expr),
                }
            }
            _ => Ok(expr),
        }
    }

    fn parse_unary_add_or_subtract_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        optional_match_token!(self.lexer, Plus);
        let minus = optional_match_token!(self.lexer, Dash);
        let expr = self.parse_null_operator_expression()?;
        if matches!(expr.root().data(), ExprIR::Integer(i64::MIN)) {
            if minus {
                return Ok(tree!(ExprIR::Integer(i64::MIN)));
            }
            return Err(self
                .lexer
                .format_error(format!("Integer overflow '{}'", i64::MAX).as_str()));
        }
        if minus {
            return Ok(tree!(ExprIR::Negate, expr));
        }
        Ok(expr)
    }

    fn parse_power_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_unary_add_or_subtract_expr()?, Token::Power => Pow);
    }

    fn parse_mul_div_modulo_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_power_expr()?, Token::Star => Mul, Token::Slash =>  Div, Token::Modulo => Modulo);
    }

    fn parse_add_sub_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_mul_div_modulo_expr()?, Token::Plus => Add, Token::Dash => Sub);
    }

    fn parser_string_list_null_predicate_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        let lhs = self.parse_add_sub_expr()?;
        match self.lexer.current() {
            Token::Keyword(Keyword::In, _) => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(ExprIR::In, lhs, rhs))
            }
            Token::Keyword(Keyword::Starts, _) => {
                self.lexer.next();
                match_token!(self.lexer => With);
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation(String::from("starts_with"), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            Token::Keyword(Keyword::Ends, _) => {
                self.lexer.next();
                match_token!(self.lexer => With);
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation(String::from("ends_with"), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            Token::Keyword(Keyword::Contains, _) => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation(String::from("contains"), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            Token::RegexMatches => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation(String::from("regex_matches"), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            _ => Ok(lhs),
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parser_string_list_null_predicate_expr()?, Token::Equal => Eq, Token::NotEqual => Neq, Token::LessThan => Lt, Token::LessThanOrEqual => Le, Token::GreaterThan => Gt, Token::GreaterThanOrEqual => Ge);
    }

    fn parse_not_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut not_count = 0;

        while let Token::Keyword(Keyword::Not, _) = self.lexer.current() {
            self.lexer.next();
            not_count += 1;
        }

        let expr = self.parse_comparison_expr()?;

        if not_count % 2 == 0 {
            Ok(expr)
        } else {
            Ok(tree!(ExprIR::Not, expr))
        }
    }

    fn parse_and_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_not_expr()?, Token::Keyword(Keyword::And, _) => And);
    }

    fn parse_xor_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_and_expr()?, Token::Keyword(Keyword::Xor, _) => Xor);
    }

    fn parse_or_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_xor_expr()?, Token::Keyword(Keyword::Or, _) => Or);
    }

    fn parse_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        self.parse_or_expr()
    }

    fn parse_ident(&mut self) -> Result<Rc<String>, String> {
        match self.lexer.current() {
            Token::Ident(id) | Token::Keyword(_, id) => {
                self.lexer.next();
                Ok(id)
            }
            token => Err(self.lexer.format_error(&format!("Invalid input {token:?}"))),
        }
    }

    fn parse_named_exprs(&mut self) -> Result<Vec<(Rc<String>, DynTree<ExprIR>)>, String> {
        let mut named_exprs = Vec::new();
        loop {
            let pos = self.lexer.pos;
            let expr = self.parse_expr()?;
            if let Token::Keyword(Keyword::As, _) = self.lexer.current() {
                self.lexer.next();
                named_exprs.push((self.parse_ident()?, expr));
            } else if let ExprIR::Var(id) = expr.root().data() {
                named_exprs.push((id.clone(), expr));
            } else {
                named_exprs.push((
                    Rc::new(String::from(&self.lexer.str[pos..self.lexer.pos])),
                    expr,
                ));
            }
            match self.lexer.current() {
                Token::Comma => self.lexer.next(),
                _ => return Ok(named_exprs),
            }
        }
    }

    fn parse_expression_list(
        &mut self,
        expression_list_type: ExpressionListType,
    ) -> Result<Vec<DynTree<ExprIR>>, String> {
        let mut exprs = Vec::new();
        while !expression_list_type.is_end_token(self.lexer.current()) {
            exprs.push(self.parse_expr()?);
            match self.lexer.current() {
                Token::Comma => self.lexer.next(),
                _ => break,
            }
        }

        if let ExpressionListType::ZeroOrMoreClosedBy(token) = expression_list_type {
            if self.lexer.current() == token {
                self.lexer.next();
            } else {
                return Err(self.lexer.format_error(&format!("Invalid input {token:?}")));
            }
        }
        Ok(exprs)
    }

    fn parse_list_literal_or_comprehension(&mut self) -> Result<DynTree<ExprIR>, String> {
        // Check if the second token is 'IN' for list comprehension
        if let Token::Ident(var) = self.lexer.current() {
            let pos = self.lexer.pos;
            self.lexer.next();
            if optional_match_token!(self.lexer => In) {
                return self.parse_list_comprehension(var);
            } else {
                self.lexer.set_pos(pos); // Reset lexer position
            }
        }

        let mut exprs = Vec::new();

        while self.lexer.current() != Token::RBrace {
            exprs.push(self.parse_expr()?);
            match self.lexer.current() {
                Token::Comma => self.lexer.next(),
                _ => break,
            }
        }

        if self.lexer.current() == Token::RBrace {
            self.lexer.next();
            Ok(tree!(ExprIR::List ; exprs))
        } else {
            Err(self
                .lexer
                .format_error(&format!("Invalid input {:?}", self.lexer.current())))
        }
    }

    fn parse_list_comprehension(
        &mut self,
        var: Rc<String>,
    ) -> Result<DynTree<ExprIR>, String> {
        // var and 'IN' already parsed
        let list_expr = self.parse_expr()?;

        let condition = if optional_match_token!(self.lexer => Where) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        let expression = if optional_match_token!(self.lexer, Pipe) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        match_token!(self.lexer, RBrace);

        Ok(tree!(
            ExprIR::ListComprehension(var.clone()),
            list_expr,
            condition.unwrap_or_else(|| tree!(ExprIR::Bool(true))),
            expression.unwrap_or_else(|| tree!(ExprIR::Var(var)))
        ))
    }

    fn parse_node_pattern(&mut self) -> Result<NodePattern, String> {
        match_token!(self.lexer, LParen);
        let alias = if let Token::Ident(id) = self.lexer.current() {
            self.lexer.next();
            Alias::String(id)
        } else {
            self.anon_id += 1;
            Alias::Anon(self.anon_id - 1)
        };
        let labels = self.parse_labels()?;
        let attrs = self.parse_map()?;
        match_token!(self.lexer, RParen);
        Ok(NodePattern::new(alias, labels, attrs))
    }

    fn parse_relationship_pattern(
        &mut self,
        src: Alias,
        clause: &Keyword,
    ) -> Result<(RelationshipPattern, NodePattern), String> {
        let is_incoming = optional_match_token!(self.lexer, LessThan);
        match_token!(self.lexer, Dash);
        let has_details = optional_match_token!(self.lexer, LBrace);
        let (alias, types, attrs) = if has_details {
            let alias = if let Token::Ident(id) = self.lexer.current() {
                self.lexer.next();
                Alias::String(id)
            } else {
                self.anon_id += 1;
                Alias::Anon(self.anon_id - 1)
            };
            let mut types = vec![];
            while optional_match_token!(self.lexer, Colon) {
                types.push(self.parse_ident()?);
                optional_match_token!(self.lexer, Pipe);
            }
            let attrs = self.parse_map()?;
            match_token!(self.lexer, RBrace);
            (alias, types, attrs)
        } else {
            self.anon_id += 1;
            (Alias::Anon(self.anon_id - 1), vec![], tree!(ExprIR::Map))
        };
        match_token!(self.lexer, Dash);
        let is_outgoing = optional_match_token!(self.lexer, GreaterThan);
        let dst = self.parse_node_pattern()?;
        let relationship = match (is_incoming, is_outgoing) {
            (true, true) | (false, false) => {
                if *clause == Keyword::Create {
                    return Err(self
                        .lexer
                        .format_error("Only directed relationships are supported in CREATE"));
                }
                RelationshipPattern::new(alias, types, attrs, src, dst.alias.clone(), true)
            }
            (true, false) => {
                RelationshipPattern::new(alias, types, attrs, dst.alias.clone(), src, false)
            }
            (false, true) => {
                RelationshipPattern::new(alias, types, attrs, src, dst.alias.clone(), false)
            }
        };
        Ok((relationship, dst))
    }

    fn parse_labels(&mut self) -> Result<Vec<Rc<String>>, String> {
        let mut labels = Vec::new();
        while self.lexer.current() == Token::Colon {
            self.lexer.next();
            labels.push(self.parse_ident()?);
        }
        Ok(labels)
    }

    fn parse_map(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut attrs = Vec::new();
        if self.lexer.current() == Token::LBracket {
            self.lexer.next();
        } else {
            return Ok(tree!(ExprIR::Map));
        }

        loop {
            match self.lexer.current() {
                Token::Ident(key) | Token::Keyword(_, key) => {
                    self.lexer.next();
                    match_token!(self.lexer, Colon);
                    let value = self.parse_expr()?;
                    attrs.push(tree!(ExprIR::Var(key), value));

                    match self.lexer.current() {
                        Token::Comma => self.lexer.next(),
                        Token::RBracket => {
                            self.lexer.next();
                            return Ok(tree!(ExprIR::Map ; attrs));
                        }
                        Token::Error(s) => return Err(s),
                        token => {
                            return Err(self
                                .lexer
                                .format_error(&format!("Invalid input {token:?}")));
                        }
                    }
                }
                _ => {
                    match_token!(self.lexer, RBracket);
                    return Ok(tree!(ExprIR::Map ; attrs));
                }
            }
        }
    }
}
