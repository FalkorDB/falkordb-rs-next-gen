//! Cypher query parser for FalkorDB.
//!
//! This module implements a hand-written recursive descent parser for the Cypher
//! query language. It converts Cypher query strings into an Abstract Syntax Tree
//! (AST) defined in [`crate::ast`].
//!
//! ## Architecture
//!
//! The parser consists of two main components:
//!
//! 1. **Lexer** (`Lexer`): Tokenizes the input string into a stream of tokens
//!    (keywords, identifiers, literals, operators, punctuation).
//!
//! 2. **Parser** (`Parser`): Consumes tokens and builds the AST using recursive
//!    descent with operator precedence parsing for expressions.
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

use crate::ast::{
    ExprIR, QuantifierType, QueryExpr, QueryGraph, QueryIR, QueryNode, QueryPath,
    QueryRelationship, RawQueryIR, SetItem,
};
use crate::indexer::{EntityType, IndexType};
use crate::runtime::orderset::OrderSet;
use crate::string_escape::cypher_unescape;
use crate::{
    cypher::Token::RParen,
    runtime::{
        functions::{FnType, get_functions},
        value::Value,
    },
    tree,
};
use itertools::Itertools;
use orx_tree::{DynTree, NodeRef};
use std::sync::Arc;
use std::{
    collections::{HashMap, HashSet},
    num::IntErrorKind,
    str::Chars,
};

#[derive(Debug, PartialEq, Clone)]
enum Keyword {
    Call,
    Yield,
    Optional,
    Match,
    Unwind,
    Merge,
    Create,
    Detach,
    Delete,
    Set,
    Remove,
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
    Distinct,
    Order,
    By,
    Asc,
    Ascending,
    Desc,
    Descending,
    Skip,
    Limit,
    Load,
    Csv,
    Headers,
    From,
    Delimiter,
    Drop,
    Index,
    Fulltext,
    Vector,
    Options,
    For,
    On,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Ident(Arc<String>),
    Keyword(Keyword, Arc<String>),
    Parameter(String),
    Integer(i64),
    Float(f64),
    String(Arc<String>),
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
    PlusEqual,
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
    EndOfFile,
}

const KEYWORDS: &[(&str, Keyword)] = &[
    ("CALL", Keyword::Call),
    ("YIELD", Keyword::Yield),
    ("OPTIONAL", Keyword::Optional),
    ("MATCH", Keyword::Match),
    ("UNWIND", Keyword::Unwind),
    ("MERGE", Keyword::Merge),
    ("CREATE", Keyword::Create),
    ("DETACH", Keyword::Detach),
    ("DELETE", Keyword::Delete),
    ("SET", Keyword::Set),
    ("REMOVE", Keyword::Remove),
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
    ("DISTINCT", Keyword::Distinct),
    ("ORDER", Keyword::Order),
    ("BY", Keyword::By),
    ("ASC", Keyword::Asc),
    ("ASCENDING", Keyword::Ascending),
    ("DESC", Keyword::Desc),
    ("DESCENDING", Keyword::Descending),
    ("SKIP", Keyword::Skip),
    ("LIMIT", Keyword::Limit),
    ("LOAD", Keyword::Load),
    ("CSV", Keyword::Csv),
    ("HEADERS", Keyword::Headers),
    ("FROM", Keyword::From),
    ("DELIMITER", Keyword::Delimiter),
    ("DROP", Keyword::Drop),
    ("INDEX", Keyword::Index),
    ("FULLTEXT", Keyword::Fulltext),
    ("VECTOR", Keyword::Vector),
    ("OPTIONS", Keyword::Options),
    ("FOR", Keyword::For),
    ("ON", Keyword::On),
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
    cached_current: Result<(Token, usize), (String, usize)>,
}

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

impl<'a> Lexer<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            str,
            pos: 0,
            cached_current: Self::get_token(str, Self::read_spaces(str, 0)),
        }
    }

    fn next(&mut self) {
        self.pos += Self::read_spaces(self.str, self.pos);
        self.pos += self.cached_current.as_ref().map(|t| t.1).unwrap_or(0);
        let pos = self.pos + Self::read_spaces(self.str, self.pos);
        self.cached_current = Self::get_token(self.str, pos);
    }

    fn pos(
        &self,
        before_whitespaces: bool,
    ) -> usize {
        if before_whitespaces {
            self.pos
        } else {
            self.pos + Self::read_spaces(self.str, self.pos)
        }
    }

    fn read_spaces(
        str: &'a str,
        pos: usize,
    ) -> usize {
        let mut len = 0;
        let mut chars = str[pos..].chars();
        let mut next = chars.next();

        while let Some(' ' | '\t' | '\n' | '/') = next {
            if next == Some('/') {
                len += 1;
                next = chars.next();
                if next.is_none() {
                    break;
                }
                len += 1;
                if next == Some('/') {
                    next = chars.next();
                    loop {
                        if next.is_none() {
                            break;
                        }
                        len += 1;
                        if next == Some('\n') {
                            next = chars.next();
                            break;
                        }
                        next = chars.next();
                    }
                } else if next == Some('*') {
                    next = chars.next();
                    loop {
                        if next.is_none() {
                            break;
                        }
                        while next == Some('*') {
                            len += 1;
                            next = chars.next();
                        }
                        if next.is_none() {
                            break;
                        }
                        len += 1;
                        if next == Some('/') {
                            next = chars.next();
                            break;
                        }
                        next = chars.next();
                    }
                } else {
                    len -= 2;
                    break;
                }
                continue;
            }
            len += 1;
            next = chars.next();
        }
        len
    }

    fn current(&self) -> Result<Token, String> {
        self.cached_current
            .as_ref()
            .map(|t| t.0.clone())
            .map_err(|e| e.0.clone())
    }

    pub fn current_str(&self) -> &str {
        let pos = self.pos(false);
        &self.str[pos..pos + self.cached_current.as_ref().map(|t| t.1).unwrap_or(0)]
    }

    #[inline]
    #[allow(clippy::too_many_lines)]
    fn get_token(
        str: &'a str,
        pos: usize,
    ) -> Result<(Token, usize), (String, usize)> {
        let mut chars = str[pos..].chars();
        if let Some(char) = chars.next() {
            return match char {
                '[' => Ok((Token::LBrace, 1)),
                ']' => Ok((Token::RBrace, 1)),
                '{' => Ok((Token::LBracket, 1)),
                '}' => Ok((Token::RBracket, 1)),
                '(' => Ok((Token::LParen, 1)),
                ')' => Ok((Token::RParen, 1)),
                '%' => Ok((Token::Modulo, 1)),
                '^' => Ok((Token::Power, 1)),
                '*' => Ok((Token::Star, 1)),
                '/' => Ok((Token::Slash, 1)),
                '+' => match chars.next() {
                    Some('=') => Ok((Token::PlusEqual, 2)),
                    _ => Ok((Token::Plus, 1)),
                },
                '-' => Ok((Token::Dash, 1)),
                '=' => match chars.next() {
                    Some('~') => Ok((Token::RegexMatches, 2)),
                    _ => Ok((Token::Equal, 1)),
                },
                '<' => match chars.next() {
                    Some('=') => Ok((Token::LessThanOrEqual, 2)),
                    Some('>') => Ok((Token::NotEqual, 2)),
                    _ => Ok((Token::LessThan, 1)),
                },
                '>' => match chars.next() {
                    Some('=') => Ok((Token::GreaterThanOrEqual, 2)),
                    _ => Ok((Token::GreaterThan, 1)),
                },
                ',' => Ok((Token::Comma, 1)),
                ':' => Ok((Token::Colon, 1)),
                '.' => match chars.next() {
                    Some('.') => Ok((Token::DotDot, 2)),
                    Some('0'..='9') => Self::lex_numeric(str, chars, pos, '.', 2),
                    _ => Ok((Token::Dot, 1)),
                },
                '|' => Ok((Token::Pipe, 1)),
                '\'' => {
                    let mut len = 1;
                    let mut end = false;
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            match chars.next() {
                                Some(c) => {
                                    len += c.len_utf8();
                                }
                                None => {
                                    return Err((
                                        format!(
                                            "Invalid escape sequence in string at pos: {}",
                                            pos + len
                                        ),
                                        len + 1,
                                    ));
                                }
                            }
                        } else if c == '\'' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return Err((format!("Unterminated string starting at pos: {pos}"), len));
                    }
                    match cypher_unescape(&str[pos + 1..pos + len]) {
                        Ok(unescaped) => Ok((Token::String(Arc::new(unescaped)), len + 1)),
                        Err(e) => Err((format!("{e} at pos: {pos}"), len + 1)),
                    }
                }
                '\"' => {
                    let mut len = 1;
                    let mut end = false;
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            match chars.next() {
                                Some(c) => {
                                    len += c.len_utf8();
                                }
                                None => {
                                    return Err((
                                        format!(
                                            "Invalid escape sequence in string at pos: {}",
                                            pos + len
                                        ),
                                        len + 1,
                                    ));
                                }
                            }
                        } else if c == '\"' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return Err((format!("Unterminated string starting at pos: {pos}"), len));
                    }
                    match cypher_unescape(&str[pos + 1..pos + len]) {
                        Ok(unescaped) => Ok((Token::String(Arc::new(unescaped)), len + 1)),
                        Err(e) => Err((format!("{e} at pos: {pos}"), len + 1)),
                    }
                }
                d @ '0'..='9' => Self::lex_numeric(str, chars, pos, d, 1),
                '$' => {
                    let mut len = 1;
                    let Some(first) = chars.next() else {
                        return Err((String::from("Invalid parameter at end of input"), len));
                    };
                    let id = if first == '`' {
                        len += 1;
                        for ch in chars {
                            len += 1;
                            if ch == '`' {
                                break;
                            }
                        }
                        &str[pos + 2..pos + len - 1]
                    } else if let 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' = first {
                        len += 1;
                        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = chars.next() {
                            len += 1;
                        }
                        &str[pos + 1..pos + len]
                    } else {
                        return Err((format!("Invalid parameter at pos: {pos}"), len));
                    };
                    let token = Token::Parameter(String::from(id));
                    Ok((token, len))
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
                            || Token::Ident(Arc::new(String::from(&str[pos..pos + len]))),
                            |o| {
                                Token::Keyword(
                                    o.1.clone(),
                                    Arc::new(String::from(&str[pos..pos + len])),
                                )
                            },
                        );
                    Ok((token, len))
                }
                '`' => {
                    let mut len = 1;
                    let mut end = false;
                    for c in chars {
                        if c == '`' {
                            end = true;
                            break;
                        }
                        len += c.len_utf8();
                    }
                    if !end {
                        return Err((String::from(&str[pos..pos + len]), len));
                    }
                    let id = &str[pos + 1..pos + len];
                    Ok((Token::Ident(Arc::new(String::from(id))), len + 1))
                }
                _ => Err((format!("Invalid input at pos: {pos} at char {char}"), 0)),
            };
        }
        Ok((Token::EndOfFile, 0))
    }

    #[allow(clippy::too_many_lines)]
    fn lex_numeric(
        str: &'a str,
        mut chars: Chars,
        pos: usize,
        current: char,
        mut len: usize,
    ) -> Result<(Token, usize), (String, usize)> {
        let mut radix = 10;
        let mut is_float = false;
        let mut is_e = false;
        if current == '0' {
            let next_char = chars.next();
            match next_char {
                Some('x' | 'X') => {
                    radix = 16;
                    len += 1;
                    // Validate that at least one hex digit follows
                    if let Some(c) = str[pos + len..].chars().next() {
                        if !c.is_ascii_hexdigit() {
                            let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                            return Err((
                                format!("Invalid numeric value '{invalid_literal}'"),
                                len,
                            ));
                        }
                    } else {
                        let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                        return Err((format!("Invalid numeric value '{invalid_literal}'"), len));
                    }
                }
                Some('o' | 'O') => {
                    radix = 8;
                    len += 1;
                    // Validate that at least one octal digit follows
                    if let Some(c) = str[pos + len..].chars().next() {
                        if !c.is_digit(8) {
                            let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                            return Err((
                                format!("Invalid numeric value '{invalid_literal}'"),
                                len,
                            ));
                        }
                    } else {
                        let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                        return Err((format!("Invalid numeric value '{invalid_literal}'"), len));
                    }
                }
                Some('0'..='9') => {
                    radix = 8;
                    len += 1;
                }
                Some('b' | 'B') => {
                    radix = 2;
                    len += 1;
                    // Validate that at least one binary digit follows
                    if let Some(c) = str[pos + len..].chars().next() {
                        if !matches!(c, '0' | '1') {
                            let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                            return Err((
                                format!("Invalid numeric value '{invalid_literal}'"),
                                len,
                            ));
                        }
                    } else {
                        let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                        return Err((format!("Invalid numeric value '{invalid_literal}'"), len));
                    }
                }
                Some('.') => match chars.next() {
                    Some(c) if c.is_digit(radix) => {
                        is_float = true;
                        len += 2;
                    }
                    _ => {
                        return Ok((Token::Integer(0), len));
                    }
                },
                Some(_) | None => {
                    return Ok((Token::Integer(0), len));
                }
            }
        } else if current == '.' {
            is_float = true;
        }
        if !is_float {
            while let Some(c) = chars.next() {
                // Check for scientific notation first (only for decimal numbers)
                if (c == 'e' || c == 'E') && radix == 10 {
                    is_float = true;
                    is_e = true;
                    len += 1;
                    if let Some(next_char) = str.get(pos + len..).and_then(|s| s.chars().next())
                        && (next_char == '-' || next_char == '+')
                    {
                        chars.next();
                        len += next_char.len_utf8();
                    }
                    break;
                } else if c.is_digit(radix) {
                    // Only accept valid digits for the current radix
                    len += 1;
                } else if c == '.' && radix == 10 {
                    if is_float {
                        return Err((format!("Invalid numeric value at pos: {pos} in {str}"), len));
                    }
                    if let Some(ch) = str[pos + len + 1..].chars().next()
                        && (ch == '.' || !ch.is_ascii_digit())
                    {
                        break;
                    }
                    is_float = true;
                    len += 1;
                    break;
                } else if c.is_alphanumeric() {
                    // Invalid character in number literal - consume the rest to create a complete error token
                    len += 1;
                    for ch in chars.by_ref() {
                        if ch.is_alphanumeric() {
                            len += 1;
                        } else {
                            break;
                        }
                    }
                    let invalid_literal = str[pos..].chars().take(len).collect::<String>();
                    return Err((
                        format!("Invalid numeric value '{invalid_literal}'"),
                        invalid_literal.len(),
                    ));
                } else {
                    break;
                }
            }
        }
        if is_float {
            while let Some(c) = chars.next() {
                if c.is_digit(radix) {
                    len += 1;
                } else if c == 'e' || c == 'E' {
                    if is_e {
                        return Err((format!("Invalid numeric value at pos: {pos} in {str}"), len));
                    }
                    len += 1;
                    if let Some(next_char) = str.get(pos + len..).and_then(|s| s.chars().next())
                        && (next_char == '-' || next_char == '+')
                    {
                        chars.next();
                        len += next_char.len_utf8();
                    }
                    for c in chars.by_ref() {
                        if c.is_digit(radix) {
                            len += 1;
                        } else {
                            break;
                        }
                    }
                    break;
                } else {
                    break;
                }
            }
        }
        let str = str[pos..].chars().take(len).collect::<String>();
        let token = Lexer::str2number_token(&str, radix, is_float);
        token.map(|t| (t, str.len())).map_err(|e| (e, str.len()))
    }

    fn str2number_token(
        str: &str,
        radix: u32,
        is_float: bool,
    ) -> Result<Token, String> {
        if is_float {
            return match str.parse::<f64>() {
                Ok(f) if f.is_finite() && !f.is_subnormal() => Ok(Token::Float(f)),
                Ok(_) => Err(format!("Float overflow '{str}'")),
                Err(_) => Err(format!("Invalid input '{str}'")),
            };
        }

        if str.eq_ignore_ascii_case(MIN_I64[0])
            || str.eq_ignore_ascii_case(MIN_I64[1])
            || str.eq_ignore_ascii_case(MIN_I64[2])
            || str.eq_ignore_ascii_case(MIN_I64[3])
            || str.eq_ignore_ascii_case(MIN_I64[4])
        {
            return Ok(Token::Integer(i64::MIN));
        }

        let mut offset = 0;
        if radix == 8 {
            if str.starts_with("0o") || str.starts_with("0O") {
                offset = 2;
            } else if 1 < str.len() && str.starts_with('0') {
                offset = 1;
            }
        } else if radix != 10 {
            offset = 2;
        }
        let number_str = &str[offset..];
        i64::from_str_radix(number_str, radix).map_or_else(
            |err| match err.kind() {
                IntErrorKind::PosOverflow => Err(format!("Integer overflow '{number_str}'")),
                IntErrorKind::NegOverflow => Err(format!("Integer overflow '-{number_str}'")),
                _ => Err(format!("Invalid input '{str}'")),
            },
            |i| Ok(Token::Integer(i)),
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
        let pos = pos + Self::read_spaces(self.str, pos);
        self.cached_current = Self::get_token(self.str, pos);
    }
}

macro_rules! match_token {
    ($lexer:expr, $token:ident) => {
        match $lexer.current()? {
            Token::$token => {
                $lexer.next();
            }
            _ => {
                return Err($lexer.format_error(&format!(
                    "Invalid input '{}': expected {:?}",
                    $lexer.current_str(),
                    Token::$token
                )));
            }
        }
    };
    ($lexer:expr => $token:ident) => {
        match $lexer.current()? {
            Token::Keyword(Keyword::$token, _) => {
                $lexer.next();
            }
            _ => {
                return Err($lexer.format_error(&format!(
                    "Invalid input '{}': expected {:?}",
                    $lexer.current_str(),
                    Keyword::$token
                )));
            }
        }
    };
    () => {};
}

macro_rules! optional_match_token {
    ($lexer:expr, $token:ident) => {
        match $lexer.current()? {
            Token::$token => {
                $lexer.next();
                true
            }
            _ => false,
        }
    };
    ($lexer:expr => $token:ident) => {
        match $lexer.current()? {
            Token::Keyword(Keyword::$token, _) => {
                $lexer.next();
                true
            }
            _ => false,
        }
    };
    () => {};
}

#[macro_export]
macro_rules! parse_expr_return {
    ($stack:ident, $res:ident) => {
        match &mut $stack.last_mut() {
            Some((_, Some(expr))) => {
                expr.root_mut().push_child_tree($res);
            }
            Some((_, expr)) => {
                *expr = Some($res);
            }
            _ => return Ok($res),
        }
    };
}

#[macro_export]
macro_rules! parse_operators {
    ($self:ident, $stack:ident, $res:ident, $current:ident, $token:pat => $expr:ident) => {
        if let $token = $self.lexer.current()? {
            $self.lexer.next();
            let res = if matches!($res.root().data(), ExprIR::$expr) {
                $res
            } else {
                tree!(ExprIR::$expr, $res)
            };
            $stack.push(($current, Some(res)));
            $stack.push(($current + 1, None));
        } else {
            match &mut $stack.last_mut() {
                Some((_, Some(expr))) => {
                    expr.root_mut().push_child_tree($res);
                }
                Some((_, expr)) => {
                    *expr = Some($res);
                }
                _ => return Ok($res),
            }
        }
    };
    ($self:ident, $stack:ident, $res:ident, $current:ident, $($token:pat => $expr:ident),*) => {
        let mut res = $res;
        $(if let $token = $self.lexer.current()? {
            $self.lexer.next();
            if matches!(res.root().data(), ExprIR::$expr) {
            } else {
                res = tree!(ExprIR::$expr, res);
            };
            $stack.push(($current, Some(res)));
            $stack.push(($current + 1, None));
            continue;
        })*

        match &mut $stack.last_mut() {
            Some((_, Some(expr))) => {
                expr.root_mut().push_child_tree(res);
            }
            Some((_, expr)) => {
                *expr = Some(res);
            }
            _ => return Ok(res),
        }
    };
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
                let mut pos = self.lexer.pos;
                while let Ok(id) = self.parse_ident() {
                    if !optional_match_token!(self.lexer, Equal) {
                        self.lexer.set_pos(pos);
                        break;
                    }
                    params.insert(String::from(id.as_str()), self.parse_expr()?);
                    pos = self.lexer.pos;
                }
            } else {
                break;
            }
        }
        Ok((params, &self.lexer.str[self.lexer.pos..]))
    }

    /// Parses a complete Cypher query.
    ///
    /// This is the main entry point for parsing. It first tries to parse index
    /// operations (CREATE INDEX, DROP INDEX), then falls back to regular queries.
    ///
    /// # Errors
    /// Returns an error string if the query has syntax errors.
    pub fn parse(&mut self) -> Result<RawQueryIR, String> {
        let pos = self.lexer.pos;
        if let Some(ir) = self.parse_index_ops()? {
            Ok(ir)
        } else {
            self.lexer.set_pos(pos);
            self.parse_query()
        }
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
                match_token!(self.lexer, EndOfFile);
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
                match_token!(self.lexer, Colon);
                let label = self.parse_ident()?;
                match_token!(self.lexer, RParen);
                (nkey, label, EntityType::Node)
            };
            match_token!(self.lexer => On);
            match_token!(self.lexer, LParen);
            let key = self.parse_ident()?;
            if nkey.as_str() != key.as_str() {
                return Err(self.lexer.format_error(&format!("'{key}' not defined")));
            }
            match_token!(self.lexer, Dot);
            let mut attrs = vec![self.parse_ident()?];
            while optional_match_token!(self.lexer, Comma) {
                let key = self.parse_ident()?;
                if nkey.as_str() != key.as_str() {
                    return Err(self.lexer.format_error(&format!("'{key}' not defined")));
                }
                match_token!(self.lexer, Dot);
                attrs.push(self.parse_ident()?);
            }
            match_token!(self.lexer, RParen);
            let index_type = if fulltext {
                IndexType::Fulltext
            } else if vector {
                IndexType::Vector
            } else {
                IndexType::Range
            };
            let options = if vector && optional_match_token!(self.lexer => Options) {
                Some(Arc::new(self.parse_map()?))
            } else {
                None
            };
            match_token!(self.lexer, EndOfFile);
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
                match_token!(self.lexer, EndOfFile);
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
                match_token!(self.lexer, Colon);
                let label = self.parse_ident()?;
                match_token!(self.lexer, RParen);
                (nkey, label, EntityType::Node)
            };
            match_token!(self.lexer => On);
            match_token!(self.lexer, LParen);
            let key = self.parse_ident()?;
            if nkey.as_str() != key.as_str() {
                return Err(self.lexer.format_error(&format!("'{key}' not defined")));
            }
            match_token!(self.lexer, Dot);
            let mut attrs = vec![self.parse_ident()?];
            while optional_match_token!(self.lexer, Comma) {
                let key = self.parse_ident()?;
                if nkey.as_str() != key.as_str() {
                    return Err(self.lexer.format_error(&format!("'{key}' not defined")));
                }
                match_token!(self.lexer, Dot);
                attrs.push(self.parse_ident()?);
            }
            match_token!(self.lexer, RParen);
            match_token!(self.lexer, EndOfFile);
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

    fn parse_query(&mut self) -> Result<RawQueryIR, String> {
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
                clauses.push(self.parse_reading_clasue()?);
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
        match_token!(self.lexer, EndOfFile);
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
                    Arc::new(self.parse_expr()?)
                } else {
                    Arc::new(tree!(ExprIR::String(Arc::new(String::from(',')))))
                };
                match_token!(self.lexer => From);
                let file_path = Arc::new(self.parse_expr()?);
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
            .parse_expression_list(ExpressionListType::ZeroOrMoreClosedBy(RParen))?
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
        let list = Arc::new(self.parse_expr()?);
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
            self.parse_expression_list(ExpressionListType::OneOrMore)?
                .into_iter()
                .map(Arc::new)
                .collect(),
            is_detach,
        ))
    }

    fn parse_where(&mut self) -> Result<Option<QueryExpr<Arc<String>>>, String> {
        if let Token::Keyword(Keyword::Where, _) = self.lexer.current()? {
            self.lexer.next();
            return Ok(Some(Arc::new(self.parse_expr()?)));
        }
        Ok(None)
    }

    fn parse_with_clause(
        &mut self,
        write: bool,
    ) -> Result<RawQueryIR, String> {
        let distinct = optional_match_token!(self.lexer => Distinct);
        let (all, exprs) = if optional_match_token!(self.lexer, Star) {
            (true, vec![])
        } else {
            (false, self.parse_named_exprs()?)
        };
        let orderby = if optional_match_token!(self.lexer => Order) {
            self.parse_orderby()?
        } else {
            vec![]
        };
        let skip = if optional_match_token!(self.lexer => Skip) {
            let skip = Arc::new(self.parse_expr()?);
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
            let limit = Arc::new(self.parse_expr()?);
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
            (false, self.parse_named_exprs()?)
        };
        let orderby = if optional_match_token!(self.lexer => Order) {
            self.parse_orderby()?
        } else {
            vec![]
        };
        let skip = if optional_match_token!(self.lexer => Skip) {
            let skip = Arc::new(self.parse_expr()?);
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
            let limit = Arc::new(self.parse_expr()?);
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
            children.push(self.parse_expr()?);
        }
        let mut conditions = vec![];
        while optional_match_token!(self.lexer => When) {
            conditions.push(self.parse_expr()?);
            match_token!(self.lexer => Then);
            conditions.push(self.parse_expr()?);
        }
        if conditions.is_empty() {
            return Err(self.lexer.format_error("Invalid input"));
        }
        children.push(tree!(ExprIR::List ; conditions));
        if optional_match_token!(self.lexer => Else) {
            children.push(self.parse_expr()?);
        } else {
            children.push(tree!(ExprIR::Null));
        }
        match_token!(self.lexer => End);
        Ok(tree!(
            ExprIR::FuncInvocation(get_functions().get("case", &FnType::Internal)?); children
        ))
    }

    fn parse_quantifier_expr(&mut self) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
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

    #[allow(clippy::too_many_lines)]
    fn parse_primary_expr(&mut self) -> Result<(DynTree<ExprIR<Arc<String>>>, bool), String> {
        match self.lexer.current()? {
            Token::Ident(_) => {
                let pos = self.lexer.pos;
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

                    let args =
                        self.parse_expression_list(ExpressionListType::ZeroOrMoreClosedBy(RParen))?;
                    func.validate(args.len())?;
                    if distinct && args.is_empty() {
                        return Err(self.lexer.format_error(
                            "DISTINCT can only be used with function calls that have arguments",
                        ));
                    }
                    return Ok((tree!(ExprIR::FuncInvocation(func); args), false));
                }
                self.lexer.set_pos(pos);
                let ident = self.parse_ident()?;
                Ok((tree!(ExprIR::Variable(ident)), false))
            }
            Token::Parameter(param) => {
                self.lexer.next();
                Ok((tree!(ExprIR::Parameter(param)), false))
            }
            Token::Keyword(Keyword::Case, _) => Ok((self.parse_case_expression()?, false)),
            Token::Keyword(Keyword::All | Keyword::Any | Keyword::None | Keyword::Single, _) => {
                Ok((self.parse_quantifier_expr()?, false))
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
                self.parse_list_literal_or_comprehension()
            }
            Token::LBracket => Ok((self.parse_map()?, false)),
            Token::LParen => {
                self.lexer.next();
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
        let from = self.parse_expr();
        if optional_match_token!(self.lexer, DotDot) {
            let to = self.parse_expr();
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
    fn parse_expr(&mut self) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
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
                    let (res, recurse) = self.parse_primary_expr()?;
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

    fn parse_ident(&mut self) -> Result<Arc<String>, String> {
        match self.lexer.current() {
            Ok(Token::Ident(id) | Token::Keyword(_, id)) => {
                self.lexer.next();
                Ok(id)
            }
            _ => Err(self.lexer.format_error(&format!(
                "Invalid input '{}': expected an identifier",
                self.lexer.current_str()
            ))),
        }
    }

    fn parse_named_exprs(&mut self) -> Result<Vec<(Arc<String>, QueryExpr<Arc<String>>)>, String> {
        let mut named_exprs = Vec::new();
        loop {
            let pos = self.lexer.pos(false);
            let expr = Arc::new(self.parse_expr()?);
            if let Token::Keyword(Keyword::As, _) = self.lexer.current()? {
                self.lexer.next();
                let ident = self.parse_ident()?;
                named_exprs.push((ident, expr));
            } else if let ExprIR::Variable(id) = expr.root().data() {
                named_exprs.push((id.clone(), expr));
            } else {
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
    ) -> Result<Vec<DynTree<ExprIR<Arc<String>>>>, String> {
        let mut exprs = Vec::new();
        while !expression_list_type.is_end_token(&self.lexer.current()?) {
            exprs.push(self.parse_expr()?);
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

    fn parse_list_literal_or_comprehension(
        &mut self
    ) -> Result<(DynTree<ExprIR<Arc<String>>>, bool), String> {
        // Check if the second token is 'IN' for list comprehension
        let pos = self.lexer.pos;
        if let Ok(var) = self.parse_ident()
            && optional_match_token!(self.lexer => In)
        {
            return Ok((self.parse_list_comprehension(var)?, false));
        }
        self.lexer.set_pos(pos); // Reset lexer position

        Ok((
            tree!(ExprIR::List),
            !optional_match_token!(self.lexer, RBrace),
        ))
    }

    fn parse_list_comprehension(
        &mut self,
        var: Arc<String>,
    ) -> Result<DynTree<ExprIR<Arc<String>>>, String> {
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
            expression.map_or_else(|| Ok::<_, String>(tree!(ExprIR::Variable(var))), Ok)?
        ))
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
        } else {
            self.parse_map()?
        };
        match_token!(self.lexer, RParen);
        Ok(Arc::new(QueryNode::new(alias, labels, Arc::new(attrs))))
    }

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
        let (alias, types, attrs) = if has_details {
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
            let _ = if optional_match_token!(self.lexer, Star) {
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
                    Some((start, end))
                } else {
                    Some((start, None))
                }
            } else {
                None
            };
            let attrs = if let Token::Parameter(param) = self.lexer.current()? {
                self.lexer.next();
                tree!(ExprIR::Parameter(param))
            } else {
                self.parse_map()?
            };
            match_token!(self.lexer, RBrace);
            (alias, types.into_iter().collect(), attrs)
        } else {
            let name = Arc::new(format!("_anon_{}", self.anon_counter));
            self.anon_counter += 1;
            (name, vec![], tree!(ExprIR::Map))
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
                QueryRelationship::new(alias, types, Arc::new(attrs), src, dst.clone(), true)
            }
            (true, false) => {
                QueryRelationship::new(alias, types, Arc::new(attrs), dst.clone(), src, false)
            }
            (false, true) => {
                QueryRelationship::new(alias, types, Arc::new(attrs), src, dst.clone(), false)
            }
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
            let value = self.parse_expr()?;
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

    fn parse_orderby(&mut self) -> Result<Vec<(QueryExpr<Arc<String>>, bool)>, String> {
        match_token!(self.lexer => By);
        let mut orderby = vec![];
        loop {
            let expr = Arc::new(self.parse_expr()?);
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
            let (mut expr, recurse) = self.parse_primary_expr()?;
            if recurse {
                expr = self.parse_expr()?;
                match_token!(self.lexer, RParen);
            }
            if self.lexer.current()? == Token::Dot {
                while self.lexer.current()? == Token::Dot {
                    self.lexer.next();
                    expr = self.parse_property_lookup(expr)?;
                }
                match_token!(self.lexer, Equal);
                let value = Arc::new(self.parse_expr()?);
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
                let value = Arc::new(self.parse_expr()?);
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
            let (mut expr, recurse) = self.parse_primary_expr()?;
            if recurse {
                expr = self.parse_expr()?;
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
