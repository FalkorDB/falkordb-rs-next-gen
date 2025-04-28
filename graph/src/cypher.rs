use crate::ast::{
    Alias, NodePattern, PathPattern, Pattern, QueryExprIR, QueryIR, RelationshipPattern,
};
use falkordb_macro::parse_binary_expr;
use std::collections::{BTreeMap, HashSet};
use std::iter::Peekable;
use std::str::Chars;

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
    Modulo,
    Power,
    Star,
    Slash,
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
    Starts,
    Ends,
    Contains,
    RegexMatches,
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
            match char {
                '[' => return (Token::LBrace, 1),
                ']' => return (Token::RBrace, 1),
                '{' => return (Token::LBracket, 1),
                '}' => return (Token::RBracket, 1),
                '(' => return (Token::LParen, 1),
                ')' => return (Token::RParen, 1),
                '%' => return (Token::Modulo, 1),
                '^' => return (Token::Power, 1),
                '*' => return (Token::Star, 1),
                '/' => return (Token::Slash, 1),
                '+' => return (Token::Plus, 1),
                '-' => return (Token::Dash, 1),
                '=' => {
                    return match chars.next() {
                        Some('~') => (Token::RegexMatches, 2),
                        _ => (Token::Equal, 1),
                    };
                }
                '<' => return (Token::LessThan, 1),
                '>' => return (Token::GreaterThan, 1),
                ',' => return (Token::Comma, 1),
                ':' => return (Token::Colon, 1),
                '.' => {
                    return match chars.next() {
                        Some('.') => (Token::DotDot, 2),
                        Some('0'..='9') => Self::lex_number(str, pos),
                        _ => (Token::Dot, 1),
                    };
                }
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
                '0'..='9' => return Self::lex_number(str, pos),
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
                        "starts" => Token::Starts,
                        "ends" => Token::Ends,
                        "contains" => Token::Contains,
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
                        Token::Error(format!("Unexpected token at pos: {pos} at char {char}")),
                        0,
                    );
                }
            }
        }
        (Token::EndOfFile, 0)
    }

    #[allow(clippy::too_many_lines)]
    fn lex_number(
        input: &str,
        start_pos: usize,
    ) -> (Token, usize) {
        let mut chars = input[start_pos..].chars().peekable();
        let mut len = 0;

        let mut has_digits_before_dot = false;

        // Check for radix prefix (0x, 0o, 0b)
        if let Some(&c) = chars.peek() {
            if c == '0' {
                chars.next();
                len += 1;
                has_digits_before_dot = true;
                if let Some(&c2) = chars.peek() {
                    if c2 == 'x' || c2 == 'X' {
                        chars.next();
                        len += 1;
                        return Lexer::lex_integer(input, start_pos, chars, len, 16);
                    } else if c2 == 'o' || c2 == 'O' {
                        chars.next();
                        len += 1;
                        return Lexer::lex_integer(input, start_pos, chars, len, 8);
                    } else if c2 == 'b' || c2 == 'B' {
                        chars.next();
                        len += 1;
                        return Lexer::lex_integer(input, start_pos, chars, len, 2);
                    }
                }
            }
        }

        // Integer part digits
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                chars.next();
                len += 1;
                has_digits_before_dot = true;
            } else {
                break;
            }
        }

        let mut has_dot = false;
        let mut has_digits_after_dot = false;

        // Fractional part
        if chars.peek() == Some(&'.') {
            has_dot = true;
            chars.next();
            len += 1;

            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    chars.next();
                    len += 1;
                    has_digits_after_dot = true;
                } else {
                    break;
                }
            }
        }

        // Exponent part
        let mut has_exponent = false;
        if let Some(&c) = chars.peek() {
            if c == 'e' || c == 'E' {
                has_exponent = true;
                chars.next();
                len += 1;

                // Optional exponent sign
                if let Some(&c2) = chars.peek() {
                    if c2 == '+' || c2 == '-' {
                        chars.next();
                        len += 1;
                    }
                }

                let mut exp_digits = 0;
                while let Some(&c3) = chars.peek() {
                    if c3.is_ascii_digit() {
                        chars.next();
                        len += 1;
                        exp_digits += 1;
                    } else {
                        break;
                    }
                }

                if exp_digits == 0 {
                    // Invalid exponent (no digits)
                    return (
                        Token::Error(format!(
                            "Invalid exponent (no digits): {}",
                            &input[start_pos..start_pos + len]
                        )),
                        0,
                    );
                }
            }
        }

        // Validate that we have digits somewhere
        if !(has_digits_before_dot || has_dot && has_digits_after_dot) {
            return (
                Token::Error(format!(
                    "Invalid number (no digits): {}",
                    &input[start_pos..start_pos + len]
                )),
                0,
            );
        }

        // if the last character is a dot, it is an integer and we should not eat the last dot
        if has_dot && !has_digits_after_dot {
            len -= 1;
            has_dot = false;
        }

        let number_str = &input[start_pos..start_pos + len];

        // If it has a dot or exponent, parse as float
        if has_dot || has_exponent {
            match number_str.parse::<f64>() {
                Ok(f) if f.is_finite() => (Token::Float(f), len),
                Ok(_) => (
                    Token::Error(format!("FloatingPointOverflow: {number_str}")),
                    len,
                ),
                Err(_) => (Token::Error(format!("Invalid float: {number_str}")), len),
            }
        } else {
            // Otherwise parse as integer
            let radix = if number_str.starts_with('0') { 8 } else { 10 };
            i64::from_str_radix(number_str, radix).map_or_else(
                |_| (Token::Error(format!("Invalid integer: {number_str}")), len),
                |i| (Token::Integer(i), len),
            )
        }
    }

    fn lex_integer(
        input: &str,
        start_pos: usize,
        mut chars: Peekable<Chars>,
        mut len: usize,
        radix: u32,
    ) -> (Token, usize) {
        let mut has_digits = false;
        let input = &input[start_pos..];
        while let Some(&c) = chars.peek() {
            if c.is_digit(radix) {
                chars.next();
                len += 1;
                has_digits = true;
            } else {
                break;
            }
        }

        if has_digits {
            // Handle prefixes and signs
            let number_str = if input.starts_with('-') || input.starts_with('+') {
                let sign = &input[0..1];
                let rest = &input[3..len];
                format!("{sign}{rest}")
            } else {
                input[2..len].to_string()
            };
            i64::from_str_radix(number_str.as_str(), radix).map_or_else(
                |_| (Token::Error(format!("Invalid integer: {number_str}")), len),
                |i| (Token::Integer(i), len),
            )
        } else {
            (
                Token::Error(format!("Invalid integer: {}", &input[..len])),
                len,
            )
        }
    }

    pub fn format_error(
        &self,
        err: &str,
    ) -> String {
        format!("{}\n{}^{}", self.str, " ".repeat(self.pos), err)
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
                        .format_error(&format!("Unexpected token {token:?}")));
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

    fn parse_pattern(
        &mut self,
        clause: Token,
    ) -> Result<Pattern, String> {
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
                let mut namespace_and_function = ident.clone();
                self.lexer.next();
                let pos = self.lexer.pos;
                while self.lexer.current() == Token::Dot {
                    self.lexer.next();
                    match self.lexer.current() {
                        Token::Ident(id) => {
                            self.lexer.next();
                            namespace_and_function.push('.');
                            namespace_and_function.push_str(&id);
                        }
                        _ => break,
                    }
                }
                if self.lexer.current() == Token::LParen {
                    self.lexer.next();

                    let exprs = self.parse_exprs()?;
                    match_token!(self.lexer, RParen);
                    return Ok(QueryExprIR::FuncInvocation(namespace_and_function, exprs));
                }
                self.lexer.set_pos(pos);
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

        while self.lexer.current() == Token::LBrace {
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

    fn parse_unary_add_or_subtract_expr(&mut self) -> Result<QueryExprIR, String> {
        match self.lexer.current() {
            Token::Plus => {
                self.lexer.next();
                Ok(self.parse_null_operator_expression()?)
            }
            Token::Dash => {
                self.lexer.next();
                let expr = self.parse_null_operator_expression()?;
                Ok(QueryExprIR::Negate(Box::new(expr)))
            }
            _ => self.parse_null_operator_expression(),
        }
    }

    fn parse_power_expr(&mut self) -> Result<QueryExprIR, String> {
        parse_binary_expr!(self.parse_unary_add_or_subtract_expr()?, Power => Pow);
    }

    fn parse_mul_div_modulo_expr(&mut self) -> Result<QueryExprIR, String> {
        parse_binary_expr!(self.parse_power_expr()?, Star => Mul, Slash =>  Div, Modulo => Modulo);
    }

    fn parse_add_sub_expr(&mut self) -> Result<QueryExprIR, String> {
        parse_binary_expr!(self.parse_mul_div_modulo_expr()?, Plus => Add, Dash => Sub);
    }

    fn parser_string_list_null_predicate_expr(&mut self) -> Result<QueryExprIR, String> {
        let lhs = self.parse_add_sub_expr()?;
        match self.lexer.current() {
            Token::In => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(QueryExprIR::In(Box::new((lhs, rhs))))
            }
            Token::Starts => {
                self.lexer.next();
                match_token!(self.lexer, With);
                let rhs = self.parse_add_sub_expr()?;
                Ok(QueryExprIR::StartsWith(Box::new((lhs, rhs))))
            }
            Token::Ends => {
                self.lexer.next();
                match_token!(self.lexer, With);
                let rhs = self.parse_add_sub_expr()?;
                Ok(QueryExprIR::EndsWith(Box::new((lhs, rhs))))
            }
            Token::Contains => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(QueryExprIR::Contains(Box::new((lhs, rhs))))
            }
            Token::RegexMatches => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(QueryExprIR::RegexMatches(Box::new((lhs, rhs))))
            }
            _ => Ok(lhs),
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<QueryExprIR, String> {
        parse_binary_expr!(self.parser_string_list_null_predicate_expr()?, Equal => Eq);
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
        parse_binary_expr!(self.parse_not_expr()?, And => And);
    }

    fn parse_xor_expr(&mut self) -> Result<QueryExprIR, String> {
        parse_binary_expr!(self.parse_and_expr()?, Xor => Xor);
    }

    fn parse_or_expr(&mut self) -> Result<QueryExprIR, String> {
        parse_binary_expr!(self.parse_xor_expr()?, Or => Or);
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
                            .format_error(&format!("Unexpected token {token:?}")));
                    }
                }
            } else {
                match_token!(self.lexer, RBracket);
                return Ok(attrs);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_float() {
        let inputs = [
            ("0.1", Token::Float(0.1)),
            ("5.14159", Token::Float(5.14159)),
            ("1e10", Token::Float(1e10)),
            ("1.2E-3", Token::Float(1.2E-3)),
            (".1", Token::Float(0.1)),
            ("1e0", Token::Float(1e0)),
        ];
        for (input, expected) in inputs {
            let (token, _) = Lexer::lex_number(input, 0);
            assert_eq!(token, expected);
        }

        let (token, _) = Lexer::lex_number("1.34E999", 0);
        assert_eq!(
            token,
            Token::Error("FloatingPointOverflow: 1.34E999".to_string())
        );
    }

    #[test]
    fn test_scan_int() {
        let inputs = [
            ("1", Token::Integer(1)),
            ("0", Token::Integer(0)),
            (
                "12345678901234567890",
                Token::Error("Invalid integer: 12345678901234567890".to_string()),
            ),
            ("0x1", Token::Integer(1)),
            ("0x10", Token::Integer(16)),
            ("0xFF", Token::Integer(255)),
            ("0o1", Token::Integer(1)),
            ("0o10", Token::Integer(8)),
            ("0o77", Token::Integer(63)),
            ("0b1", Token::Integer(1)),
            ("0b10", Token::Integer(2)),
            ("0b1111", Token::Integer(15)),
            ("02613152366", Token::Integer(372036854)),
        ];

        for (input, expected) in inputs {
            let (token, _) = Lexer::lex_number(input, 0);
            assert_eq!(token, expected);
        }
    }
}
