use std::collections::{BTreeMap, HashSet};

use crate::ast::{
    Alias, NodePattern, PathPattern, Pattern, QueryExprIR, QueryIR, RelationshipPattern,
};

#[derive(Debug, PartialEq)]
enum Token {
    Call,
    Match,
    Unwind,
    Create,
    Delete,
    Where,
    With,
    Return,
    As,
    Ident(String),
    Parameter(String),
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Or,
    Xor,
    And,
    Not,
    Star,
    Plus,
    Dash,
    Is,
    Equal,
    LessThan,
    GreaterThan,
    Comma,
    Colon,
    Dot,
    DotDot,
    Error(String),
    EndOfFile,
}

struct Lexer<'a> {
    str: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    const fn new(str: &'a str) -> Self {
        Self { str, pos: 0 }
    }

    fn next(&mut self, len: usize) {
        self.pos += len;
    }

    #[allow(clippy::too_many_lines)]
    fn current(&mut self) -> (Token, usize) {
        let mut chars = self.str[self.pos..].chars();
        while let Some(char) = chars.next() {
            match char {
                ' ' | '\t' | '\n' => {
                    self.pos += 1;
                    continue;
                }
                '[' => return (Token::LBrace, 1),
                ']' => return (Token::RBrace, 1),
                '{' => return (Token::LBracket, 1),
                '}' => return (Token::RBracket, 1),
                '(' => return (Token::LParen, 1),
                ')' => return (Token::RParen, 1),
                '*' => return (Token::Star, 1),
                '+' => return (Token::Plus, 1),
                '-' => {
                    if let Some('0'..='9') = chars.next() {
                        return self.lex_number(2, &mut chars);
                    }
                    return (Token::Dash, 1);
                }
                '=' => return (Token::Equal, 1),
                '<' => return (Token::LessThan, 1),
                '>' => return (Token::GreaterThan, 1),
                ',' => return (Token::Comma, 1),
                ':' => return (Token::Colon, 1),
                '.' => {
                    if let Some('.') = chars.next() {
                        return (Token::DotDot, 2);
                    }
                    return (Token::Dot, 1);
                }
                '\'' => {
                    let mut pos = self.pos + 1;
                    let mut end = false;
                    for c in chars.by_ref() {
                        if c == '\'' {
                            end = true;
                            break;
                        }
                        pos += c.len_utf8();
                    }
                    if !end {
                        return (
                            Token::Error(self.str[self.pos + 1..pos].to_string()),
                            pos - self.pos + 1,
                        );
                    }
                    return (
                        Token::String(self.str[self.pos + 1..pos].to_string()),
                        pos - self.pos + 1,
                    );
                }
                '\"' => {
                    let mut pos = self.pos + 1;
                    let mut end = false;
                    for c in chars.by_ref() {
                        if c == '\"' {
                            end = true;
                            break;
                        }
                        pos += c.len_utf8();
                    }
                    if !end {
                        return (
                            Token::Error(self.str[self.pos + 1..pos].to_string()),
                            pos - self.pos + 1,
                        );
                    }
                    return (
                        Token::String(self.str[self.pos + 1..pos].to_string()),
                        pos - self.pos + 1,
                    );
                }
                '0'..='9' => return self.lex_number(1, &mut chars),
                '$' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = chars.next() {
                        len += 1;
                    }
                    let token =
                        Token::Parameter(self.str[self.pos + 1..self.pos + len].to_string());
                    return (token, len);
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = chars.next() {
                        len += 1;
                    }
                    let token = match self.str[self.pos..self.pos + len].to_lowercase().as_str() {
                        "call" => Token::Call,
                        "match" => Token::Match,
                        "unwind" => Token::Unwind,
                        "create" => Token::Create,
                        "delete" => Token::Delete,
                        "where" => Token::Where,
                        "with" => Token::With,
                        "return" => Token::Return,
                        "null" => Token::Null,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        "as" => Token::As,
                        "or" => Token::Or,
                        "xor" => Token::Xor,
                        "and" => Token::And,
                        "not" => Token::Not,
                        "is" => Token::Is,
                        _ => Token::Ident(self.str[self.pos..self.pos + len].to_string()),
                    };
                    return (token, len);
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
                            Token::Error(self.str[self.pos + 1..self.pos + len].to_string()),
                            len + 1,
                        );
                    }
                    return (
                        Token::Ident(self.str[self.pos + 1..self.pos + len].to_string()),
                        len + 1,
                    );
                }
                _ => return (Token::Error("Unexpected token".to_string()), 0),
            }
        }
        (Token::EndOfFile, 0)
    }

    fn lex_number(&self, init_len: usize, chars: &mut std::str::Chars<'_>) -> (Token, usize) {
        let mut len = init_len;
		let mut chars = chars.peekable();
        while let Some(c @ ('0'..='9' | '.')) = chars.next() {
            if c == '.' && (Some(&'.') == chars.peek()) {
                break;
            }
            len += 1;
        }
        if self.str[self.pos..self.pos + len].contains('.') {
            return self.str[self.pos..self.pos + len]
                .parse::<f64>()
                .map_or((Token::Error("Float overflow".to_string()), 0), |num| {
                    (Token::Float(num), len)
                });
        }
        self.str[self.pos..self.pos + len]
            .parse::<i64>()
            .map_or((Token::Error("Integer overflow".to_string()), 0), |num| {
                (Token::Integer(num), len)
            })
    }

    pub fn format_error(&self, err: &str) -> String {
        format!("{}\n{}^{}", self.str, " ".repeat(self.pos), err)
    }
}

macro_rules! match_token {
    ($lexer:expr, $token:ident) => {
        match $lexer.current() {
            (Token::$token, len) => {
                $lexer.next(len);
            }
            (token, _) => return Err($lexer.format_error(&format!("Unexpected token: {token:?}"))),
        }
    };
    () => {};
}

macro_rules! optional_match_token {
    ($lexer:expr, $token:ident) => {
        match $lexer.current() {
            (Token::$token, len) => {
                $lexer.next(len);
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
    pub const fn new(str: &'a str) -> Self {
        Self {
            lexer: Lexer::new(str),
            anon_id: 0,
        }
    }

    pub fn parse_parameters(&mut self) -> Result<(BTreeMap<String, QueryExprIR>, &'a str), String> {
        match self.lexer.current() {
            (Token::Ident(id), _) => {
                if id == "CYPHER" {
                    self.lexer.next(id.len());
                    let mut params = BTreeMap::new();
                    while let (Token::Ident(id), len) = self.lexer.current() {
                        self.lexer.next(len);
                        match_token!(self.lexer, Equal);
                        params.insert(id, self.parse_expr()?);
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
        let ir = self.parse_query()?;
        ir.validate()?;
        Ok(ir)
    }

    fn parse_query(&mut self) -> Result<QueryIR, String> {
        let mut clauses = Vec::new();

        loop {
            match self.lexer.current() {
                (Token::Call, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_call_clause()?);
                }
                (Token::Match, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_match_clause()?);
                }
                (Token::Unwind, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_unwind_clause()?);
                }
                (Token::Create, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_create_clause()?);
                }
                (Token::Delete, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_delete_clause()?);
                }
                (Token::Where, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_where_clause()?);
                }
                (Token::With, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_with_clause()?);
                }
                (Token::Return, len) => {
                    self.lexer.next(len);
                    clauses.push(self.parse_return_clause()?);
                }
                (Token::EndOfFile, _) => {
                    return Ok(QueryIR::Query(clauses));
                }
                (token, _) => {
                    return Err(self
                        .lexer
                        .format_error(&format!("Unexpected token {token:?}")))
                }
            }
        }
    }

    fn parse_call_clause(&mut self) -> Result<QueryIR, String> {
        let ident = self.parse_dotted_ident()?;
        match_token!(self.lexer, LParen);
        if let (Token::RParen, len) = self.lexer.current() {
            self.lexer.next(len);
            return Ok(QueryIR::Call(ident, vec![]));
        }
        let exprs = self.parse_exprs()?;
        match_token!(self.lexer, RParen);
        Ok(QueryIR::Call(ident, exprs))
    }

    fn parse_dotted_ident(&mut self) -> Result<String, String> {
        let mut ident = self.parse_ident()?;
        while let (Token::Dot, len) = self.lexer.current() {
            self.lexer.next(len);
            ident.push('.');
            ident.push_str(&self.parse_ident()?);
        }
        Ok(ident)
    }

    fn parse_match_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Match(self.parse_pattern(Token::Match)?))
    }

    fn parse_unwind_clause(&mut self) -> Result<QueryIR, String> {
        let list = self.parse_expr()?;
        match_token!(self.lexer, As);
        let ident = self.parse_ident()?;

        Ok(QueryIR::Unwind(list, ident))
    }

    fn parse_create_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Create(self.parse_pattern(Token::Create)?))
    }

    fn parse_delete_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Delete(self.parse_exprs()?))
    }

    fn parse_where_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Where(self.parse_expr()?))
    }

    fn parse_with_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::With(self.parse_named_exprs()?))
    }

    fn parse_return_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Return(self.parse_named_exprs()?))
    }

    fn parse_pattern(&mut self, clause: Token) -> Result<Pattern, String> {
        let mut nodes = Vec::new();
        let mut nodes_alias = HashSet::new();
        let mut relationships = Vec::new();
        let mut paths = Vec::new();
        loop {
            if let Ok(p) = self.parse_ident() {
                match_token!(self.lexer, Equal);
                let mut vars = Vec::new();
                let left = self.parse_node_pattern()?;
                let mut left_alias = left.alias.clone();
                vars.push(left_alias.clone());
                if nodes_alias.insert(left.alias.to_string()) {
                    nodes.push(left);
                }
                loop {
                    if let (Token::Dash | Token::LessThan, _) = self.lexer.current() {
                        let (relationship, right) = self.parse_relationship_pattern(left_alias)?;
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
                while let (Token::Dash | Token::LessThan, _) = self.lexer.current() {
                    let (relationship, right) = self.parse_relationship_pattern(left_alias)?;
                    left_alias = right.alias.clone();
                    relationships.push(relationship);
                    if nodes_alias.insert(right.alias.to_string()) {
                        nodes.push(right);
                    }
                }
            }

            match self.lexer.current() {
                (Token::Comma, len) => {
                    self.lexer.next(len);
                    continue;
                }
                (token, len) => {
                    if token == clause {
                        self.lexer.next(len);
                        continue;
                    }
                    break;
                }
            }
        }

        Ok(Pattern::new(nodes, relationships, paths))
    }

    fn parse_primary_expr(&mut self) -> Result<QueryExprIR, String> {
        match self.lexer.current() {
            (Token::Ident(ident), len) => {
                self.lexer.next(len);
                if let (Token::LParen, len) = self.lexer.current() {
                    self.lexer.next(len);

                    let exprs = self.parse_exprs()?;
                    match_token!(self.lexer, RParen);
                    return Ok(QueryExprIR::FuncInvocation(ident, exprs));
                }

                Ok(QueryExprIR::Ident(ident))
            }
            (Token::Parameter(param), len) => {
                self.lexer.next(len);
                Ok(QueryExprIR::Parameter(param))
            }
            (Token::Null, len) => {
                self.lexer.next(len);
                Ok(QueryExprIR::Null)
            }
            (Token::Bool(b), len) => {
                self.lexer.next(len);
                Ok(QueryExprIR::Bool(b))
            }
            (Token::Integer(i), len) => {
                self.lexer.next(len);
                Ok(QueryExprIR::Integer(i))
            }
            (Token::Float(f), len) => {
                self.lexer.next(len);
                Ok(QueryExprIR::Float(f))
            }
            (Token::String(s), len) => {
                self.lexer.next(len);
                Ok(QueryExprIR::String(s))
            }
            (Token::LBrace, len) => {
                self.lexer.next(len);
                if let (Token::RBrace, len) = self.lexer.current() {
                    self.lexer.next(len);
                    return Ok(QueryExprIR::List(vec![]));
                }
                let exprs = self.parse_exprs()?;
                match_token!(self.lexer, RBrace);
                Ok(QueryExprIR::List(exprs))
            }
            (Token::LBracket, _) => {
                let attrs = self.parse_map()?;
                Ok(QueryExprIR::Map(attrs))
            }
            (Token::LParen, len) => {
                self.lexer.next(len);
                let expr = self.parse_expr()?;
                match_token!(self.lexer, RParen);
                Ok(expr)
            }
            (token, _) => Err(self
                .lexer
                .format_error(&format!("Unexpected token {token:?}"))),
        }
    }

    fn parse_property_expression(&mut self) -> Result<QueryExprIR, String> {
        let mut expr = self.parse_primary_expr()?;

        while let (Token::Dot, len) = self.lexer.current() {
            self.lexer.next(len);
            let ident = self.parse_ident()?;
            expr = QueryExprIR::Property(Box::new(expr), ident);
        }

        Ok(expr)
    }

    // match one of those kind [..4], [4..], [4..5], [6]
    fn parse_list_operator_expression(&mut self) -> Result<QueryExprIR, String> {
        let mut expr = self.parse_property_expression()?;
        while let (Token::LBrace, len) = self.lexer.current() {
            self.lexer.next(len);

            if optional_match_token!(self.lexer, DotDot) {
                let start = None;
                expr = self.match_get_elements(expr, start)?
            }else {
                let index = self.parse_expr()?;

                if optional_match_token!(self.lexer, DotDot) {
                    expr = self.match_get_elements(expr, Some(index))?;
                } else {
                    match_token!(self.lexer, RBrace);
                    expr = QueryExprIR::GetElement(Box::new((expr, index)));
                }
            }
        }

        Ok(expr)
    }

    fn match_get_elements(&mut self, expr: QueryExprIR, start: Option<QueryExprIR>) -> Result<QueryExprIR, String> {
        let end = self.parse_expr().ok();
        match_token!(self.lexer, RBrace);
        Ok(QueryExprIR::GetElements(Box::new((expr, start, end))))
    }

    fn parse_null_operator_expression(&mut self) -> Result<QueryExprIR, String> {
        let expr = self.parse_list_operator_expression()?;

        match self.lexer.current() {
            (Token::Is, len) => {
                self.lexer.next(len);
                match self.lexer.current() {
                    (Token::Null, len) => {
                        self.lexer.next(len);
                        Ok(QueryExprIR::IsNull(Box::new(expr)))
                    }
                    _ => Ok(expr),
                }
            }
            _ => Ok(expr),
        }
    }

    fn parse_mul_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();
        loop {
            vec.push(self.parse_null_operator_expression()?);

            match self.lexer.current() {
                (Token::Star, len) => {
                    self.lexer.next(len);
                }
                _ => {
                    if vec.len() == 1 {
                        return Ok(vec.pop().unwrap());
                    }
                    return Ok(QueryExprIR::Mul(vec));
                }
            }
        }
    }

    fn parse_add_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();

        loop {
            vec.push(self.parse_mul_expr()?);

            match self.lexer.current() {
                (Token::Plus, len) => {
                    self.lexer.next(len);
                }
                _ => {
                    if vec.len() == 1 {
                        return Ok(vec.pop().unwrap());
                    }
                    return Ok(QueryExprIR::Add(vec));
                }
            }
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();

        loop {
            vec.push(self.parse_add_expr()?);

            match self.lexer.current() {
                (Token::Equal, len) => {
                    self.lexer.next(len);
                }
                _ => {
                    if vec.len() == 1 {
                        return Ok(vec.pop().unwrap());
                    }
                    return Ok(QueryExprIR::Eq(vec));
                }
            }
        }
    }

    fn parse_not_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut not_count = 0;

        while let (Token::Not, len) = self.lexer.current() {
            self.lexer.next(len);
            not_count += 1;
        }

        let expr = self.parse_comparison_expr()?;

        if not_count % 2 == 0 {
            Ok(expr)
        } else {
            Ok(QueryExprIR::Not(Box::new(expr)))
        }
    }

    fn parse_and_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();

        loop {
            vec.push(self.parse_not_expr()?);

            if let (Token::And, len) = self.lexer.current() {
                self.lexer.next(len);
            } else {
                if vec.len() == 1 {
                    return Ok(vec.pop().unwrap());
                }
                return Ok(QueryExprIR::And(vec));
            }
        }
    }

    fn parse_xor_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();

        loop {
            vec.push(self.parse_and_expr()?);

            if let (Token::Xor, len) = self.lexer.current() {
                self.lexer.next(len);
            } else {
                if vec.len() == 1 {
                    return Ok(vec.pop().unwrap());
                }
                return Ok(QueryExprIR::Xor(vec));
            }
        }
    }

    fn parse_or_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();

        loop {
            vec.push(self.parse_xor_expr()?);

            if let (Token::Or, len) = self.lexer.current() {
                self.lexer.next(len);
            } else {
                if vec.len() == 1 {
                    return Ok(vec.pop().unwrap());
                }
                return Ok(QueryExprIR::Or(vec));
            }
        }
    }

    fn parse_expr(&mut self) -> Result<QueryExprIR, String> {
        self.parse_or_expr()
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        match self.lexer.current() {
            (Token::Ident(v), len) => {
                self.lexer.next(len);
                Ok(v)
            }
            (token, _) => Err(self
                .lexer
                .format_error(&format!("Unexpected token {token:?}"))),
        }
    }

    fn parse_named_exprs(&mut self) -> Result<Vec<QueryExprIR>, String> {
        let mut exprs = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            if let (Token::As, len) = self.lexer.current() {
                self.lexer.next(len);
                let ident = self.parse_ident()?;
                exprs.push(QueryExprIR::Named(ident, Box::new(expr)));
            } else {
                exprs.push(expr);
            }
            match self.lexer.current() {
                (Token::Comma, len) => self.lexer.next(len),
                _ => return Ok(exprs),
            }
        }
    }

    fn parse_exprs(&mut self) -> Result<Vec<QueryExprIR>, String> {
        let mut exprs = Vec::new();
        loop {
            exprs.push(self.parse_expr()?);
            match self.lexer.current() {
                (Token::Comma, len) => self.lexer.next(len),
                _ => return Ok(exprs),
            }
        }
    }

    fn parse_node_pattern(&mut self) -> Result<NodePattern, String> {
        match_token!(self.lexer, LParen);
        let alias = if let (Token::Ident(id), len) = self.lexer.current() {
            self.lexer.next(len);
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
    ) -> Result<(RelationshipPattern, NodePattern), String> {
        let is_incomming = optional_match_token!(self.lexer, LessThan);
        match_token!(self.lexer, Dash);
        match_token!(self.lexer, LBrace);
        let alias = if let (Token::Ident(id), len) = self.lexer.current() {
            self.lexer.next(len);
            Alias::String(id)
        } else {
            self.anon_id += 1;
            Alias::Anon(self.anon_id - 1)
        };
        match_token!(self.lexer, Colon);
        let relationship_type = self.parse_ident()?;
        let attrs = self.parse_map()?;
        match_token!(self.lexer, RBrace);
        match_token!(self.lexer, Dash);
        let is_outgoing = optional_match_token!(self.lexer, GreaterThan);
        let dst = self.parse_node_pattern()?;
        let relationship = match (is_incomming, is_outgoing) {
            (true, true) | (false, false) => RelationshipPattern::new(
                alias,
                relationship_type,
                attrs,
                src,
                dst.alias.clone(),
                true,
            ),
            (true, false) => RelationshipPattern::new(
                alias,
                relationship_type,
                attrs,
                dst.alias.clone(),
                src,
                false,
            ),
            (false, true) => RelationshipPattern::new(
                alias,
                relationship_type,
                attrs,
                src,
                dst.alias.clone(),
                false,
            ),
        };
        Ok((relationship, dst))
    }

    fn parse_labels(&mut self) -> Result<Vec<String>, String> {
        let mut labels = Vec::new();
        while let (Token::Colon, len) = self.lexer.current() {
            self.lexer.next(len);
            labels.push(self.parse_ident()?);
        }
        Ok(labels)
    }

    fn parse_map(&mut self) -> Result<BTreeMap<String, QueryExprIR>, String> {
        let mut attrs = BTreeMap::new();
        if let (Token::LBracket, len) = self.lexer.current() {
            self.lexer.next(len);
        } else {
            return Ok(attrs);
        }

        loop {
            if let (Token::Ident(key), len) = self.lexer.current() {
                self.lexer.next(len);
                match_token!(self.lexer, Colon);
                let value = self.parse_expr()?;
                attrs.insert(key, value);

                match self.lexer.current() {
                    (Token::Comma, len) => self.lexer.next(len),
                    (Token::RBracket, len) => {
                        self.lexer.next(len);
                        return Ok(attrs);
                    }
                    (token, _) => {
                        return Err(self
                            .lexer
                            .format_error(&format!("Unexpected token {token:?}")))
                    }
                }
            } else {
                match_token!(self.lexer, RBracket);
                return Ok(attrs);
            }
        }
    }
}
