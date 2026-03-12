//! Cypher lexer and token definitions.
//!
//! This module contains the lexical analysis layer used by the Cypher parser.
//! It converts an input query string into a token stream and provides:
//! - [`Keyword`]: recognized Cypher keywords
//! - [`Token`]: lexical token variants
//! - [`Lexer`]: cursor-based tokenizer with lookahead via `current()`
//!
//! The lexer also handles:
//! - comments and whitespace skipping
//! - string unescaping
//! - numeric literal parsing and validation

use crate::parser::string_escape::cypher_unescape;
use std::sync::Arc;
use std::{num::IntErrorKind, str::Chars};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
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
    Union,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Semicolon,
    EndOfFile,
}

impl std::fmt::Display for Token {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self {
            Self::Ident(s) | Self::Keyword(_, s) => write!(f, "'{s}'"),
            Self::Parameter(s) => write!(f, "${s}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::String(s) => write!(f, "\"{s}\""),
            Self::LBrace => write!(f, "'{{'"),
            Self::RBrace => write!(f, "'}}'"),
            Self::LBracket => write!(f, "'['"),
            Self::RBracket => write!(f, "']'"),
            Self::LParen => write!(f, "'('"),
            Self::RParen => write!(f, "')'"),
            Self::Modulo => write!(f, "'%'"),
            Self::Power => write!(f, "'^'"),
            Self::Star => write!(f, "'*'"),
            Self::Slash => write!(f, "'/'"),
            Self::Plus => write!(f, "'+'"),
            Self::Dash => write!(f, "'-'"),
            Self::Equal => write!(f, "'='"),
            Self::PlusEqual => write!(f, "'+='"),
            Self::NotEqual => write!(f, "'<>'"),
            Self::LessThan => write!(f, "'<'"),
            Self::LessThanOrEqual => write!(f, "'<='"),
            Self::GreaterThan => write!(f, "'>'"),
            Self::GreaterThanOrEqual => write!(f, "'>='"),
            Self::Comma => write!(f, "','"),
            Self::Colon => write!(f, "':'"),
            Self::Dot => write!(f, "'.'"),
            Self::DotDot => write!(f, "'..'"),
            Self::Pipe => write!(f, "'|'"),
            Self::RegexMatches => write!(f, "'=~'"),
            Self::Semicolon => write!(f, "';'"),
            Self::EndOfFile => write!(f, "end of input"),
        }
    }
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
    ("UNION", Keyword::Union),
];

const MIN_I64: [&str; 5] = [
    "0b1000000000000000000000000000000000000000000000000000000000000000", // binary
    "0o1000000000000000000000",                                           // octal
    "01000000000000000000000",                                            // octal
    "9223372036854775808",                                                // decimal
    "0x8000000000000000",                                                 // hex
];

pub struct Lexer<'a> {
    pub str: &'a str,
    pos: usize,
    cached_current: Result<(Token, usize), (String, usize)>,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            pos: 0,
            cached_current: Self::get_token(str, Self::read_spaces(str, 0)),
        }
    }

    pub fn next(&mut self) {
        self.pos += Self::read_spaces(self.str, self.pos);
        self.pos += self.cached_current.as_ref().map_or(0, |t| t.1);
        let pos = self.pos + Self::read_spaces(self.str, self.pos);
        self.cached_current = Self::get_token(self.str, pos);
    }

    #[must_use]
    pub fn pos(
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

    pub fn current(&self) -> Result<Token, String> {
        self.cached_current
            .as_ref()
            .map(|t| t.0.clone())
            .map_err(|e| e.0.clone())
    }

    #[must_use]
    pub fn current_str(&self) -> &str {
        let pos = self.pos(false);
        &self.str[pos..pos + self.cached_current.as_ref().map_or(0, |t| t.1)]
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
                        let mut end = false;
                        for ch in chars {
                            len += 1;
                            if ch == '`' {
                                end = true;
                                break;
                            }
                        }
                        if !end {
                            return Err((
                                format!("Unterminated backtick-quoted parameter at pos: {pos}"),
                                len,
                            ));
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
                ';' => Ok((Token::Semicolon, 1)),
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

    #[must_use]
    pub fn format_error(
        &self,
        err: &str,
    ) -> String {
        format!("{}, errCtx: {}, pos {}", err, self.str, self.pos)
    }

    pub fn set_pos(
        &mut self,
        pos: usize,
    ) {
        self.pos = pos;
        let pos = pos + Self::read_spaces(self.str, pos);
        self.cached_current = Self::get_token(self.str, pos);
    }
}
