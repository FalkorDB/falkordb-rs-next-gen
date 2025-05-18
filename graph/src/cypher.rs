use crate::ast::{Alias, ExprIR, NodePattern, PathPattern, Pattern, QueryIR, RelationshipPattern};
use crate::cypher::Token::{RBrace, RParen};
use crate::functions::{FnType, get_functions};
use crate::tree;
use falkordb_macro::parse_binary_expr;
use orx_tree::DynTree;
use std::collections::{BTreeMap, HashSet};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
enum Keyword {
    Call,
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
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Ident(String),
    Keyword(Keyword, String),
    Parameter(String),
    Integer(i64),
    Float(f64),
    String(String),
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
    LessThan,
    GreaterThan,
    Comma,
    Colon,
    Dot,
    DotDot,
    Pipe,
    RegexMatches,
    Error(String),
    EndOfFile,
}

const KEYWORDS: [(&str, Keyword); 22] = [
    ("CALL", Keyword::Call),
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
            Self::ZeroOrMoreClosedBy(token) => token == &current_token,
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
                '<' => (Token::LessThan, 1),
                '>' => (Token::GreaterThan, 1),
                ',' => (Token::Comma, 1),
                ':' => (Token::Colon, 1),
                '.' => match chars.next() {
                    Some('.') => (Token::DotDot, 2),
                    Some('0'..='9') => Self::lex_number(str, pos),
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
                        return (Token::Error(str[pos + 1..pos + len].to_string()), len + 1);
                    }
                    (Token::String(str[pos + 1..pos + len].to_string()), len + 1)
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
                    (Token::String(str[pos + 1..pos + len].to_string()), len + 1)
                }
                '0'..='9' => Self::lex_number(str, pos),
                '$' => {
                    let mut len = 1;
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9') = chars.next() {
                        len += 1;
                    }
                    let token = Token::Parameter(str[pos + 1..pos + len].to_string());
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
                            || Token::Ident(str[pos..pos + len].to_string()),
                            |o| Token::Keyword(o.1.clone(), str[pos..pos + len].to_string()),
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
                        return (Token::Error(str[pos + 1..pos + len].to_string()), len + 1);
                    }
                    (Token::Ident(str[pos + 1..pos + len].to_string()), len + 1)
                }
                _ => (
                    Token::Error(format!("Unexpected token at pos: {pos} at char {char}")),
                    0,
                ),
            };
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
        Lexer::consume_digits(&mut chars, &mut len, &mut has_digits_before_dot);

        let mut has_dot = false;
        let mut has_digits_after_dot = false;

        // Fractional part
        if chars.peek() == Some(&'.') {
            has_dot = true;
            chars.next();
            len += 1;
            Lexer::consume_digits(&mut chars, &mut len, &mut has_digits_after_dot);
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

        // if the last character is a dot, it is an integer, and we should not eat the last dot
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

    fn consume_digits(
        chars: &mut Peekable<Chars>,
        len: &mut usize,
        consume_chars: &mut bool,
    ) {
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                chars.next();
                *len += 1;
                *consume_chars = true;
            } else {
                break;
            }
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
        format!(
            "{}\n{}^{} pos {}",
            self.str,
            " ".repeat(self.pos),
            err,
            self.pos
        )
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
    ($lexer:expr => $token:ident) => {
        match $lexer.current() {
            Token::Keyword(Keyword::$token, _) => {
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
        let mut ir = self.parse_query()?;
        ir.validate()?;
        Ok(ir)
    }

    fn parse_query(&mut self) -> Result<QueryIR, String> {
        let mut clauses = Vec::new();
        let mut write = false;
        loop {
            while let Token::Keyword(
                Keyword::Match | Keyword::Unwind | Keyword::Call | Keyword::Where,
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
                .format_error(&format!("Unexpected token: {:?}", self.lexer.current())));
        }
        Ok(QueryIR::Query(clauses, write))
    }

    fn parse_reading_clasue(&mut self) -> Result<QueryIR, String> {
        match self.lexer.current() {
            Token::Keyword(Keyword::Match, _) => {
                self.lexer.next();
                self.parse_match_clause()
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
            token => Err(self
                .lexer
                .format_error(&format!("Unexpected token {token:?}"))),
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
            token => Err(self
                .lexer
                .format_error(&format!("Unexpected token {token:?}"))),
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
        Ok(QueryIR::Match(self.parse_pattern(Keyword::Match)?))
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
                Ok(tree!(ExprIR::List; self.parse_expression_list(
                    ExpressionListType::ZeroOrMoreClosedBy(RBrace),
                )?))
            }
            Token::LBracket => self.parse_map(),
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

    fn parse_property_expression(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut expr = self.parse_primary_expr()?;

        while self.lexer.current() == Token::Dot {
            self.lexer.next();
            let ident = self.parse_ident()?;
            expr = tree!(
                ExprIR::FuncInvocation("property".to_string(), FnType::Internal),
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
                match self.lexer.current() {
                    Token::Keyword(Keyword::Null, _) => {
                        self.lexer.next();
                        Ok(tree!(ExprIR::IsNull, expr))
                    }
                    _ => Ok(expr),
                }
            }
            _ => Ok(expr),
        }
    }

    fn parse_unary_add_or_subtract_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        match self.lexer.current() {
            Token::Plus => {
                self.lexer.next();
                Ok(self.parse_null_operator_expression()?)
            }
            Token::Dash => {
                self.lexer.next();
                let expr = self.parse_null_operator_expression()?;
                Ok(tree!(ExprIR::Negate, expr))
            }
            _ => self.parse_null_operator_expression(),
        }
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
                    ExprIR::FuncInvocation("starts_with".to_string(), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            Token::Keyword(Keyword::Ends, _) => {
                self.lexer.next();
                match_token!(self.lexer => With);
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation("ends_with".to_string(), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            Token::Keyword(Keyword::Contains, _) => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation("contains".to_string(), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            Token::RegexMatches => {
                self.lexer.next();
                let rhs = self.parse_add_sub_expr()?;
                Ok(tree!(
                    ExprIR::FuncInvocation("regex_matches".to_string(), FnType::Internal),
                    lhs,
                    rhs
                ))
            }
            _ => Ok(lhs),
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parser_string_list_null_predicate_expr()?, Token::Equal => Eq);
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

    fn parse_named_exprs(&mut self) -> Result<Vec<DynTree<ExprIR>>, String> {
        let mut exprs = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            if let Token::Keyword(Keyword::As, _) = self.lexer.current() {
                self.lexer.next();
                let ident = self.parse_ident()?;
                exprs.push(tree!(ExprIR::Set(ident), expr));
            } else {
                exprs.push(expr);
            }
            match self.lexer.current() {
                Token::Comma => self.lexer.next(),
                _ => return Ok(exprs),
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
                return Err(self
                    .lexer
                    .format_error(&format!("Unexpected token {token:?}")));
            }
        }
        Ok(exprs)
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
                if clause == &Keyword::Create {
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

    fn parse_labels(&mut self) -> Result<Vec<String>, String> {
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
                        token => {
                            return Err(self
                                .lexer
                                .format_error(&format!("Unexpected token {token:?}")));
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
            ("02613152366", Token::Integer(372_036_854)),
        ];

        for (input, expected) in inputs {
            let (token, _) = Lexer::lex_number(input, 0);
            assert_eq!(token, expected);
        }
    }
}
