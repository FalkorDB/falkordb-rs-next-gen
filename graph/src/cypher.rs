use crate::ast::{
    Alias, NodePattern, PathPattern, Pattern, QueryExprIR, QueryIR, RelationshipPattern,
};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
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
    In,
    Error(String),
    EndOfFile,
}

struct Lexer<'a> {
    str: &'a str,
    pos: usize,
    cached_current: (Token, usize),
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

    fn read_spaces(str: &'a str, pos: usize) -> usize {
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
    fn get_token(str: &'a str, pos: usize) -> (Token, usize) {
        let mut chars = str[pos..].chars();
        if let Some(char) = chars.next() {
            match char {
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
                        return Self::lex_number(str, pos, 2, &mut chars);
                    }
                    return (Token::Dash, 1);
                }
                '=' => return (Token::Equal, 1),
                '<' => return (Token::LessThan, 1),
                '>' => return (Token::GreaterThan, 1),
                ',' => return (Token::Comma, 1),
                ':' => return (Token::Colon, 1),
                '.' => match chars.next() {
                    Some('.') => return (Token::DotDot, 2),
                    _ => return (Token::Dot, 1),
                },
                '\'' => {
                    let mut len = 1;
                    let mut end = false;
                    for c in chars.by_ref() {
                        if c == '\'' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return (Token::Error(str[pos + 1..pos + len].to_string()), len + 1);
                    }
                    return (Token::String(str[pos + 1..pos + len].to_string()), len + 1);
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
                        return (Token::Error(str[pos + 1..pos + len].to_string()), len + 1);
                    }
                    return (Token::String(str[pos + 1..pos + len].to_string()), len + 1);
                }
                '0'..='9' => return Self::lex_number(str, pos, 1, &mut chars),
                '$' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = chars.next() {
                        len += 1;
                    }
                    let token = Token::Parameter(str[pos + 1..pos + len].to_string());
                    return (token, len);
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = chars.next() {
                        len += 1;
                    }
                    let token = match str[pos..pos + len].to_lowercase().as_str() {
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
                        "in" => Token::In,
                        "nan" => Token::Float(f64::NAN),
                        _ => Token::Ident(str[pos..pos + len].to_string()),
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
                        return (Token::Error(str[pos + 1..pos + len].to_string()), len + 1);
                    }
                    return (Token::Ident(str[pos + 1..pos + len].to_string()), len + 1);
                }
                _ => {
                    return (
                        Token::Error(format!("Unexpected token at pos: {} at char {}", pos, char)),
                        0,
                    )
                }
            }
        }
        (Token::EndOfFile, 0)
    }

    fn lex_number(
        str: &'a str,
        pos: usize,
        init_len: usize,
        chars: &mut std::str::Chars<'_>,
    ) -> (Token, usize) {
        let mut len = init_len;
        let mut current = chars.next();
        while let Some('0'..='9') = current {
            len += 1;
            current = chars.next();
        }
        if current == Some('.') {
            let mut is_float = false;
            while let Some('0'..='9') = chars.next() {
                len += 1;
                is_float = true;
            }
            if is_float {
                len += 1;
                return str[pos..pos + len]
                    .parse::<f64>()
                    .map_or((Token::Error("Float overflow".to_string()), 0), |num| {
                        (Token::Float(num), len)
                    });
            }
        }
        str[pos..pos + len]
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
            Token::$token => {
                $lexer.next();
            }
            token => return Err($lexer.format_error(&format!("Unexpected token: {token:?}"))),
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

    pub fn parse_parameters(&mut self) -> Result<(BTreeMap<String, QueryExprIR>, &'a str), String> {
        match self.lexer.current() {
            Token::Ident(id) => {
                if id == "CYPHER" {
                    self.lexer.next();
                    let mut params = BTreeMap::new();
                    while let Token::Ident(id) = self.lexer.current() {
                        self.lexer.next();
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
                Token::Call => {
                    self.lexer.next();
                    clauses.push(self.parse_call_clause()?);
                }
                Token::Match => {
                    self.lexer.next();
                    clauses.push(self.parse_match_clause()?);
                }
                Token::Unwind => {
                    self.lexer.next();
                    clauses.push(self.parse_unwind_clause()?);
                }
                Token::Create => {
                    self.lexer.next();
                    clauses.push(self.parse_create_clause()?);
                }
                Token::Delete => {
                    self.lexer.next();
                    clauses.push(self.parse_delete_clause()?);
                }
                Token::Where => {
                    self.lexer.next();
                    clauses.push(self.parse_where_clause()?);
                }
                Token::With => {
                    self.lexer.next();
                    clauses.push(self.parse_with_clause()?);
                }
                Token::Return => {
                    self.lexer.next();
                    clauses.push(self.parse_return_clause()?);
                }
                Token::EndOfFile => {
                    return Ok(QueryIR::Query(clauses));
                }
                token => {
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
        if self.lexer.current() == Token::RParen {
            self.lexer.next();
            return Ok(QueryIR::Call(ident, vec![]));
        }
        let exprs = self.parse_exprs()?;
        match_token!(self.lexer, RParen);
        Ok(QueryIR::Call(ident, exprs))
    }

    fn parse_dotted_ident(&mut self) -> Result<String, String> {
        let mut ident = self.parse_ident()?;
        while self.lexer.current() == Token::Dot {
            self.lexer.next();
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
                    if let Token::Dash | Token::LessThan = self.lexer.current() {
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
                while let Token::Dash | Token::LessThan = self.lexer.current() {
                    let (relationship, right) = self.parse_relationship_pattern(left_alias)?;
                    left_alias = right.alias.clone();
                    relationships.push(relationship);
                    if nodes_alias.insert(right.alias.to_string()) {
                        nodes.push(right);
                    }
                }
            }

            match self.lexer.current() {
                Token::Comma => {
                    self.lexer.next();
                    continue;
                }
                token => {
                    if token == clause {
                        self.lexer.next();
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
            Token::Ident(ident) => {
                self.lexer.next();
                if self.lexer.current() == Token::LParen {
                    self.lexer.next();

                    let exprs = self.parse_exprs()?;
                    match_token!(self.lexer, RParen);
                    return Ok(QueryExprIR::FuncInvocation(ident, exprs));
                }

                Ok(QueryExprIR::Ident(ident))
            }
            Token::Parameter(param) => {
                self.lexer.next();
                Ok(QueryExprIR::Parameter(param))
            }
            Token::Null => {
                self.lexer.next();
                Ok(QueryExprIR::Null)
            }
            Token::Bool(b) => {
                self.lexer.next();
                Ok(QueryExprIR::Bool(b))
            }
            Token::Integer(i) => {
                self.lexer.next();
                Ok(QueryExprIR::Integer(i))
            }
            Token::Float(f) => {
                self.lexer.next();
                Ok(QueryExprIR::Float(f))
            }
            Token::String(s) => {
                self.lexer.next();
                Ok(QueryExprIR::String(s))
            }
            Token::LBrace => {
                self.lexer.next();
                if self.lexer.current() == Token::RBrace {
                    self.lexer.next();
                    return Ok(QueryExprIR::List(vec![]));
                }
                let exprs = self.parse_exprs()?;
                match_token!(self.lexer, RBrace);
                Ok(QueryExprIR::List(exprs))
            }
            Token::LBracket => {
                let attrs = self.parse_map()?;
                Ok(QueryExprIR::Map(attrs))
            }
            Token::LParen => {
                self.lexer.next();
                let expr = self.parse_expr()?;
                match_token!(self.lexer, RParen);
                Ok(expr)
            }
            token => Err(self
                .lexer
                .format_error(&format!("Unexpected token {token:?}"))),
        }
    }

    fn parse_property_expression(&mut self) -> Result<QueryExprIR, String> {
        let mut expr = self.parse_primary_expr()?;

        while self.lexer.current() == Token::Dot {
            self.lexer.next();
            let ident = self.parse_ident()?;
            expr = QueryExprIR::Property(Box::new(expr), ident);
        }

        Ok(expr)
    }

    // match one of those kind [..4], [4..], [4..5], [6]
    fn parse_list_operator_expression(&mut self) -> Result<QueryExprIR, String> {
        let mut expr = self.parse_property_expression()?;

        while let Token::LBrace = self.lexer.current() {
            self.lexer.next();

            let from = self.parse_expr();
            if optional_match_token!(self.lexer, DotDot) {
                let to = self.parse_expr().ok();
                match_token!(self.lexer, RBrace);
                expr = QueryExprIR::GetElements(Box::new((expr, from.ok(), to)));
            } else {
                match_token!(self.lexer, RBrace);
                expr = QueryExprIR::GetElement(Box::new((expr, from?)));
            }
        }

        Ok(expr)
    }

    fn parse_null_operator_expression(&mut self) -> Result<QueryExprIR, String> {
        let expr = self.parse_list_operator_expression()?;

        match self.lexer.current() {
            Token::Is => {
                self.lexer.next();
                match self.lexer.current() {
                    Token::Null => {
                        self.lexer.next();
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
                Token::Star => {
                    self.lexer.next();
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
                Token::Plus => {
                    self.lexer.next();
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
    fn parser_string_list_null_predicate_expr(&mut self) -> Result<QueryExprIR, String> {
        let lhs = self.parse_add_expr()?;
        match self.lexer.current() {
            Token::In => {
                self.lexer.next();
                let rhs = self.parse_add_expr()?;
                Ok(QueryExprIR::In(Box::new((lhs, rhs))))
            }
            _ => Ok(lhs),
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<QueryExprIR, String> {
        let mut vec = Vec::new();

        loop {
            vec.push(self.parser_string_list_null_predicate_expr()?);

            match self.lexer.current() {
                Token::Equal => {
                    self.lexer.next();
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

        while self.lexer.current() == Token::Not {
            self.lexer.next();
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

            if let Token::And = self.lexer.current() {
                self.lexer.next();
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

            if self.lexer.current() == Token::Xor {
                self.lexer.next();
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

            if self.lexer.current() == Token::Or {
                self.lexer.next();
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
            Token::Ident(v) => {
                self.lexer.next();
                Ok(v)
            }
            token => Err(self
                .lexer
                .format_error(&format!("Unexpected token {token:?}"))),
        }
    }

    fn parse_named_exprs(&mut self) -> Result<Vec<QueryExprIR>, String> {
        let mut exprs = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            if self.lexer.current() == Token::As {
                self.lexer.next();
                let ident = self.parse_ident()?;
                exprs.push(QueryExprIR::Named(ident, Box::new(expr)));
            } else {
                exprs.push(expr);
            }
            match self.lexer.current() {
                Token::Comma => self.lexer.next(),
                _ => return Ok(exprs),
            }
        }
    }

    fn parse_exprs(&mut self) -> Result<Vec<QueryExprIR>, String> {
        let mut exprs = Vec::new();
        loop {
            exprs.push(self.parse_expr()?);
            match self.lexer.current() {
                Token::Comma => self.lexer.next(),
                _ => return Ok(exprs),
            }
        }
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
    ) -> Result<(RelationshipPattern, NodePattern), String> {
        let is_incomming = optional_match_token!(self.lexer, LessThan);
        match_token!(self.lexer, Dash);
        match_token!(self.lexer, LBrace);
        let alias = if let Token::Ident(id) = self.lexer.current() {
            self.lexer.next();
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
        while self.lexer.current() == Token::Colon {
            self.lexer.next();
            labels.push(self.parse_ident()?);
        }
        Ok(labels)
    }

    fn parse_map(&mut self) -> Result<BTreeMap<String, QueryExprIR>, String> {
        let mut attrs = BTreeMap::new();
        if self.lexer.current() == Token::LBracket {
            self.lexer.next();
        } else {
            return Ok(attrs);
        }

        loop {
            if let Token::Ident(key) = self.lexer.current() {
                self.lexer.next();
                match_token!(self.lexer, Colon);
                let value = self.parse_expr()?;
                attrs.insert(key, value);

                match self.lexer.current() {
                    Token::Comma => self.lexer.next(),
                    Token::RBracket => {
                        self.lexer.next();
                        return Ok(attrs);
                    }
                    token => {
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
