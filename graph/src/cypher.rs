use crate::ast::{
    ExprIR, NodePattern, PathPattern, Pattern, QuantifierType, QueryIR, RelationshipPattern, VarId,
};
use crate::cypher::Token::RParen;
use crate::functions::{FnType, Type, get_functions};
use crate::tree;
use crate::value::RcValue;
use falkordb_macro::parse_binary_expr;
use hashbrown::{HashMap, HashSet};
use ordermap::OrderSet;
use orx_tree::{DynTree, NodeRef};
use std::num::IntErrorKind;
use std::rc::Rc;
use std::str::Chars;
use unescaper::unescape;

#[derive(Debug, PartialEq, Clone)]
enum Keyword {
    Call,
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
    Error(String),
    EndOfFile,
}

const KEYWORDS: &[(&str, Keyword)] = &[
    ("CALL", Keyword::Call),
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
                '+' => match chars.next() {
                    Some('=') => (Token::PlusEqual, 2),
                    _ => (Token::Plus, 1),
                },
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
                            match chars.next() {
                                Some(c) => {
                                    len += c.len_utf8();
                                }
                                None => {
                                    return (
                                        Token::Error(String::from(&str[pos + 1..pos + len])),
                                        len + 1,
                                    );
                                }
                            }
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
                    unescape(&str[pos + 1..pos + len]).map_or_else(
                        |e| match e {
                            unescaper::Error::InvalidChar { .. } => (
                                Token::String(Rc::new(String::from(&str[pos + 1..pos + len]))),
                                len + 1,
                            ),
                            _ => (
                                Token::Error(String::from(&str[pos + 1..pos + len])),
                                len + 1,
                            ),
                        },
                        |unescaped| (Token::String(Rc::new(unescaped)), len + 1),
                    )
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
                                    return (
                                        Token::Error(String::from(&str[pos + 1..pos + len])),
                                        len + 1,
                                    );
                                }
                            }
                        } else if c == '\"' {
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
                    unescape(&str[pos + 1..pos + len]).map_or_else(
                        |e| match e {
                            unescaper::Error::InvalidChar { .. } => (
                                Token::String(Rc::new(String::from(&str[pos + 1..pos + len]))),
                                len + 1,
                            ),
                            _ => (
                                Token::Error(String::from(&str[pos + 1..pos + len])),
                                len + 1,
                            ),
                        },
                        |unescaped| (Token::String(Rc::new(unescaped)), len + 1),
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
                Ok(f) if f.is_finite() && !f.is_subnormal() => Token::Float(f),
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
    var_id: u32,
    vars: HashMap<Rc<String>, VarId>,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(str: &'a str) -> Self {
        Self {
            lexer: Lexer::new(str),
            var_id: 0,
            vars: HashMap::new(),
        }
    }

    fn create_var(
        &mut self,
        name: Option<Rc<String>>,
        ty: Type,
    ) -> Result<VarId, String> {
        if let Some(name) = &name {
            if let Some(id) = self.vars.get(name) {
                if (ty == Type::Relationship && (id.ty == Type::Node || id.ty == Type::Path))
                    || (ty == Type::Node && (id.ty == Type::Relationship || id.ty == Type::Path))
                {
                    return Err(format!(
                        "The alias '{}' was specified for both a node and a relationship.",
                        name.as_str()
                    ));
                }
                // debug_assert!(id.ty == ty, "Variable type mismatch");
                return Ok(VarId {
                    name: id.name.clone(),
                    id: id.id,
                    ty,
                });
            }
            self.vars.insert(
                name.clone(),
                VarId {
                    name: Some(name.clone()),
                    id: self.var_id,
                    ty: ty.clone(),
                },
            );
        }
        self.var_id += 1;
        Ok(VarId {
            name,
            id: self.var_id - 1,
            ty,
        })
    }

    pub fn parse_parameters(
        &mut self
    ) -> Result<(HashMap<String, DynTree<ExprIR>>, &'a str), String> {
        match self.lexer.current() {
            Token::Ident(id) => {
                if id.as_str() == "CYPHER" {
                    self.lexer.next();
                    let mut params = HashMap::new();
                    while let Token::Ident(id) = self.lexer.current() {
                        self.lexer.next();
                        match_token!(self.lexer, Equal);
                        params.insert(String::from(id.as_str()), self.parse_expr()?);
                    }
                    Ok((params, &self.lexer.str[self.lexer.pos..]))
                } else {
                    Ok((HashMap::new(), self.lexer.str))
                }
            }
            _ => Ok((HashMap::new(), self.lexer.str)),
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
            while let Token::Keyword(
                Keyword::Create
                | Keyword::Merge
                | Keyword::Delete
                | Keyword::Detach
                | Keyword::Set
                | Keyword::Remove,
                _,
            ) = self.lexer.current()
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
            self.parse_pattern(&Keyword::Match)?,
            optional,
        ))
    }

    fn parse_unwind_clause(&mut self) -> Result<QueryIR, String> {
        let list = self.parse_expr()?;
        match_token!(self.lexer => As);
        let ident = self.parse_ident()?;
        Ok(QueryIR::Unwind(
            list,
            self.create_var(Some(ident), Type::Any)?,
        ))
    }

    fn parse_create_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Create(self.parse_pattern(&Keyword::Create)?))
    }

    fn parse_merge_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Merge(self.parse_pattern(&Keyword::Merge)?))
    }

    fn parse_delete_clause(
        &mut self,
        is_detach: bool,
    ) -> Result<QueryIR, String> {
        Ok(QueryIR::Delete(
            self.parse_expression_list(ExpressionListType::OneOrMore)?,
            is_detach,
        ))
    }

    fn parse_where_clause(&mut self) -> Result<QueryIR, String> {
        Ok(QueryIR::Where(self.parse_expr()?))
    }

    fn parse_with_clause(
        &mut self,
        write: bool,
    ) -> Result<QueryIR, String> {
        let exprs = if optional_match_token!(self.lexer, Star) {
            vec![]
        } else {
            self.parse_named_exprs()?
        };
        let orderby = if optional_match_token!(self.lexer => Order) {
            self.parse_orderby()?
        } else {
            vec![]
        };
        let skip = if optional_match_token!(self.lexer => Skip) {
            let skip = self.parse_expr()?;
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
            let limit = self.parse_expr()?;
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
        Ok(QueryIR::With {
            exprs,
            orderby,
            skip,
            limit,
            write,
        })
    }

    fn parse_return_clause(
        &mut self,
        write: bool,
    ) -> Result<QueryIR, String> {
        let exprs = if optional_match_token!(self.lexer, Star) {
            vec![]
        } else {
            self.parse_named_exprs()?
        };
        let orderby = if optional_match_token!(self.lexer => Order) {
            self.parse_orderby()?
        } else {
            vec![]
        };
        let skip = if optional_match_token!(self.lexer => Skip) {
            let skip = self.parse_expr()?;
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
            let limit = self.parse_expr()?;
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
            exprs,
            orderby,
            skip,
            limit,
            write,
        })
    }

    fn parse_pattern(
        &mut self,
        clause: &Keyword,
    ) -> Result<Pattern, String> {
        let mut nodes = Vec::new();
        let mut nodes_alias = HashSet::new();
        let mut relationships = Vec::new();
        let mut paths = Vec::new();
        loop {
            if let Token::Ident(ident) = self.lexer.current() {
                self.lexer.next();
                match_token!(self.lexer, Equal);
                let mut vars = vec![];
                let mut left = self.parse_node_pattern(clause)?;
                vars.push(left.alias.clone());
                if nodes_alias.insert(left.alias.clone()) {
                    nodes.push(left.clone());
                }
                loop {
                    if let Token::Dash | Token::LessThan = self.lexer.current() {
                        let (relationship, right) =
                            self.parse_relationship_pattern(left, clause)?;
                        vars.push(relationship.alias.clone());
                        vars.push(right.alias.clone());
                        left = right.clone();
                        relationships.push(relationship);
                        if nodes_alias.insert(right.alias.clone()) {
                            nodes.push(right);
                        }
                    } else {
                        paths.push(Rc::new(PathPattern::new(
                            self.create_var(Some(ident), Type::Path)?,
                            vars,
                        )));
                        break;
                    }
                }
            } else {
                let mut left = self.parse_node_pattern(clause)?;

                if nodes_alias.insert(left.alias.clone()) {
                    nodes.push(left.clone());
                }
                while let Token::Dash | Token::LessThan = self.lexer.current() {
                    let (relationship, right) = self.parse_relationship_pattern(left, clause)?;
                    left = right.clone();
                    relationships.push(relationship);
                    if nodes_alias.insert(right.alias.clone()) {
                        nodes.push(right);
                    }
                }
            }

            if *clause == Keyword::Merge {
                break;
            }

            match self.lexer.current() {
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

        Ok(Pattern::new(nodes, relationships, paths))
    }

    fn parse_case_expression(&mut self) -> Result<DynTree<ExprIR>, String> {
        self.lexer.next();
        let mut children = vec![];
        if let Token::Keyword(Keyword::When, _) = self.lexer.current() {
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
            ExprIR::Quantifier(quantifier_type, self.create_var(Some(var), Type::Any)?),
            expr,
            condition
        ))
    }

    #[allow(clippy::too_many_lines)]
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

                    let func = get_functions()
                        .get(&namespace_and_function, &FnType::Function)
                        .or_else(|_| {
                            get_functions().get(
                                &namespace_and_function,
                                &FnType::Aggregation(RcValue::null(), None),
                            )
                        })?;

                    let distinct = optional_match_token!(self.lexer => Distinct);

                    if func.is_aggregate() {
                        if optional_match_token!(self.lexer, Star) {
                            let mut arg = tree!(ExprIR::Var(self.create_var(None, Type::Any)?));
                            if distinct {
                                arg = tree!(ExprIR::Distinct, arg);
                            }
                            match_token!(self.lexer, RParen);
                            return Ok(tree!(ExprIR::FuncInvocation(func), arg));
                        }

                        let mut args = self.parse_expression_list(
                            ExpressionListType::ZeroOrMoreClosedBy(RParen),
                        )?;
                        if distinct {
                            args = vec![tree!(ExprIR::Distinct; args)];
                        }
                        args.push(tree!(ExprIR::Var(self.create_var(None, Type::Any)?)));
                        return Ok(tree!(ExprIR::FuncInvocation(func); args));
                    }

                    let args =
                        self.parse_expression_list(ExpressionListType::ZeroOrMoreClosedBy(RParen))?;
                    return Ok(tree!(ExprIR::FuncInvocation(func); args));
                }
                self.lexer.set_pos(pos);
                Ok(tree!(ExprIR::Var(self.create_var(Some(ident), Type::Any)?)))
            }
            Token::Parameter(param) => {
                self.lexer.next();
                Ok(tree!(ExprIR::Parameter(param)))
            }
            Token::Keyword(Keyword::Case, _) => self.parse_case_expression(),
            Token::Keyword(Keyword::All | Keyword::Any | Keyword::None | Keyword::Single, _) => {
                self.parse_quantifier_expr()
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

    // match one of those kind [..4], [4..], [4..5], [6]
    fn parse_list_operator_expression(
        &mut self,
        mut lhs: DynTree<ExprIR>,
    ) -> Result<DynTree<ExprIR>, String> {
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

    fn parse_unary_add_or_subtract_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        optional_match_token!(self.lexer, Plus);
        let minus = optional_match_token!(self.lexer, Dash);
        let expr = self.parse_non_arithmetic_operator_expr()?;
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

    fn parse_non_arithmetic_operator_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut lhs = self.parse_primary_expr()?;
        loop {
            match self.lexer.current() {
                Token::LBrace => {
                    self.lexer.next();
                    lhs = self.parse_list_operator_expression(lhs)?;
                }
                Token::Dot => {
                    self.lexer.next();
                    lhs = self.parse_property_lookup(lhs)?;
                }
                _ => break,
            }
        }
        if self.lexer.current() == Token::Colon {
            let labels = tree!(ExprIR::List; self
                .parse_labels()?
                .into_iter()
                .map(|l| tree!(ExprIR::String(l))));
            return Ok(tree!(
                ExprIR::FuncInvocation(get_functions().get("node_has_labels", &FnType::Internal)?),
                lhs,
                labels
            ));
        }
        Ok(lhs)
    }

    fn parse_property_lookup(
        &mut self,
        expr: DynTree<ExprIR>,
    ) -> Result<DynTree<ExprIR>, String> {
        let ident = self.parse_ident()?;
        Ok(tree!(
            ExprIR::FuncInvocation(get_functions().get("property", &FnType::Internal)?),
            expr,
            tree!(ExprIR::String(ident))
        ))
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

    fn parse_string_list_null_predicate_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        let mut lhs = self.parse_add_sub_expr()?;
        loop {
            match self.lexer.current() {
                Token::Keyword(Keyword::In, _) => {
                    self.lexer.next();
                    let rhs = self.parse_add_sub_expr()?;
                    lhs = tree!(ExprIR::In, lhs, rhs);
                }
                Token::Keyword(Keyword::Starts, _) => {
                    self.lexer.next();
                    match_token!(self.lexer => With);
                    let rhs = self.parse_add_sub_expr()?;
                    lhs = tree!(
                        ExprIR::FuncInvocation(
                            get_functions().get("starts_with", &FnType::Internal)?
                        ),
                        lhs,
                        rhs
                    );
                }
                Token::Keyword(Keyword::Ends, _) => {
                    self.lexer.next();
                    match_token!(self.lexer => With);
                    let rhs = self.parse_add_sub_expr()?;
                    lhs = tree!(
                        ExprIR::FuncInvocation(
                            get_functions().get("ends_with", &FnType::Internal)?,
                        ),
                        lhs,
                        rhs
                    );
                }
                Token::Keyword(Keyword::Contains, _) => {
                    self.lexer.next();
                    let rhs = self.parse_add_sub_expr()?;
                    lhs = tree!(
                        ExprIR::FuncInvocation(get_functions().get("contains", &FnType::Internal)?,),
                        lhs,
                        rhs
                    );
                }
                Token::RegexMatches => {
                    self.lexer.next();
                    let rhs = self.parse_add_sub_expr()?;
                    lhs = tree!(
                        ExprIR::FuncInvocation(
                            get_functions().get("regex_matches", &FnType::Internal)?,
                        ),
                        lhs,
                        rhs
                    );
                }
                Token::Keyword(Keyword::Is, _) => {
                    self.lexer.next();
                    let is_not = tree!(ExprIR::Bool(optional_match_token!(self.lexer => Not)));
                    match_token!(self.lexer => Null);
                    lhs = tree!(
                        ExprIR::FuncInvocation(get_functions().get("is_null", &FnType::Internal)?),
                        is_not,
                        lhs
                    );
                }

                _ => return Ok(lhs),
            }
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<DynTree<ExprIR>, String> {
        parse_binary_expr!(self.parse_string_list_null_predicate_expr()?, Token::Equal => Eq, Token::NotEqual => Neq, Token::LessThan => Lt, Token::LessThanOrEqual => Le, Token::GreaterThan => Gt, Token::GreaterThanOrEqual => Ge);
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

    fn parse_named_exprs(&mut self) -> Result<Vec<(VarId, DynTree<ExprIR>)>, String> {
        let mut named_exprs = Vec::new();
        loop {
            let pos = self.lexer.pos;
            let expr = self.parse_expr()?;
            if let Token::Keyword(Keyword::As, _) = self.lexer.current() {
                self.lexer.next();
                let ident = self.parse_ident()?;
                named_exprs.push((self.create_var(Some(ident), Type::Any)?, expr));
            } else if let ExprIR::Var(id) = expr.root().data() {
                named_exprs.push((id.clone(), expr));
            } else {
                named_exprs.push((
                    self.create_var(
                        Some(Rc::new(String::from(&self.lexer.str[pos..self.lexer.pos]))),
                        Type::Any,
                    )?,
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
        while !expression_list_type.is_end_token(&self.lexer.current()) {
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
            }
            self.lexer.set_pos(pos); // Reset lexer position
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
            ExprIR::ListComprehension(self.create_var(Some(var.clone()), Type::Any)?),
            list_expr,
            condition.unwrap_or_else(|| tree!(ExprIR::Bool(true))),
            expression.map_or_else(
                || Ok::<_, String>(tree!(ExprIR::Var(self.create_var(Some(var), Type::Any)?))),
                Ok
            )?
        ))
    }

    fn parse_node_pattern(
        &mut self,
        clause: &Keyword,
    ) -> Result<Rc<NodePattern>, String> {
        match_token!(self.lexer, LParen);
        let alias = if let Token::Ident(id) = self.lexer.current() {
            self.lexer.next();
            self.create_var(Some(id), Type::Node)?
        } else {
            self.create_var(None, Type::Node)?
        };
        let labels = self.parse_labels()?;
        let attrs = if let Token::Parameter(param) = self.lexer.current() {
            self.lexer.next();
            if clause == &Keyword::Match {
                return Err(self
                    .lexer
                    .format_error("Encountered unhandled type in inlined properties."));
            }
            tree!(ExprIR::Parameter(param))
        } else {
            self.parse_map()?
        };
        match_token!(self.lexer, RParen);
        Ok(Rc::new(NodePattern::new(alias, labels, Rc::new(attrs))))
    }

    fn parse_relationship_pattern(
        &mut self,
        src: Rc<NodePattern>,
        clause: &Keyword,
    ) -> Result<(Rc<RelationshipPattern>, Rc<NodePattern>), String> {
        let is_incoming = optional_match_token!(self.lexer, LessThan);
        match_token!(self.lexer, Dash);
        let has_details = optional_match_token!(self.lexer, LBrace);
        let (alias, types, attrs) = if has_details {
            let alias = if let Token::Ident(id) = self.lexer.current() {
                self.lexer.next();
                self.create_var(Some(id), Type::Relationship)?
            } else {
                self.create_var(None, Type::Relationship)?
            };
            let mut types = vec![];
            if optional_match_token!(self.lexer, Colon) {
                loop {
                    types.push(self.parse_ident()?);
                    let pipe = optional_match_token!(self.lexer, Pipe);
                    let colon = optional_match_token!(self.lexer, Colon);
                    if pipe || colon {
                        continue;
                    }
                    break;
                }
            }
            let _ = if optional_match_token!(self.lexer, Star) {
                let start = if let Token::Integer(i) = self.lexer.current() {
                    self.lexer.next();
                    Some(i)
                } else {
                    None
                };
                if optional_match_token!(self.lexer, DotDot) {
                    let end = if let Token::Integer(i) = self.lexer.current() {
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
            let attrs = if let Token::Parameter(param) = self.lexer.current() {
                self.lexer.next();
                if clause == &Keyword::Match {
                    return Err(self
                        .lexer
                        .format_error("Encountered unhandled type in inlined properties."));
                }
                tree!(ExprIR::Parameter(param))
            } else {
                self.parse_map()?
            };
            match_token!(self.lexer, RBrace);
            (alias, types, attrs)
        } else {
            (
                self.create_var(None, Type::Relationship)?,
                vec![],
                tree!(ExprIR::Map),
            )
        };
        match_token!(self.lexer, Dash);
        let is_outgoing = optional_match_token!(self.lexer, GreaterThan);
        let dst = self.parse_node_pattern(clause)?;
        let relationship = match (is_incoming, is_outgoing) {
            (true, true) | (false, false) => {
                if *clause == Keyword::Create {
                    return Err(self
                        .lexer
                        .format_error("Only directed relationships are supported in CREATE"));
                }
                RelationshipPattern::new(alias, types, Rc::new(attrs), src, dst.clone(), true)
            }
            (true, false) => {
                RelationshipPattern::new(alias, types, Rc::new(attrs), dst.clone(), src, false)
            }
            (false, true) => {
                RelationshipPattern::new(alias, types, Rc::new(attrs), src, dst.clone(), false)
            }
        };
        Ok((Rc::new(relationship), dst))
    }

    fn parse_labels(&mut self) -> Result<OrderSet<Rc<String>>, String> {
        let mut labels = OrderSet::new();
        while self.lexer.current() == Token::Colon {
            self.lexer.next();
            labels.insert(self.parse_ident()?);
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
                    attrs.push(tree!(ExprIR::String(key), value));

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

    fn parse_orderby(&mut self) -> Result<Vec<(DynTree<ExprIR>, bool)>, String> {
        match_token!(self.lexer => By);
        let mut orderby = vec![];
        loop {
            let expr = self.parse_expr()?;
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

    fn parse_set_clause(&mut self) -> Result<QueryIR, String> {
        let mut set_items = vec![];
        loop {
            let mut expr = self.parse_primary_expr()?;
            if self.lexer.current() == Token::Dot {
                while self.lexer.current() == Token::Dot {
                    self.lexer.next();
                    expr = self.parse_property_lookup(expr)?;
                }
                match_token!(self.lexer, Equal);
                let value = self.parse_expr()?;
                set_items.push((expr, value, false));
            } else if self.lexer.current() == Token::Colon {
                expr = tree!(
                    ExprIR::FuncInvocation(
                        get_functions().get("node_set_labels", &FnType::Internal)?
                    ),
                    expr,
                    tree!(ExprIR::List; self.parse_labels()?.into_iter().map(|l| tree!(ExprIR::String(l))))
                );
                set_items.push((expr, tree!(ExprIR::Null), false));
            } else {
                let equals = optional_match_token!(self.lexer, Equal);
                let plus_equals = if equals {
                    false
                } else {
                    match_token!(self.lexer, PlusEqual);
                    true
                };
                let value = self.parse_expr()?;
                set_items.push((expr, value, !plus_equals));
            }

            if !optional_match_token!(self.lexer, Comma) {
                break;
            }
        }
        Ok(QueryIR::Set(set_items))
    }

    fn parse_remove_clause(&mut self) -> Result<QueryIR, String> {
        let mut remove_items = vec![];
        loop {
            let mut expr = self.parse_primary_expr()?;
            if self.lexer.current() == Token::Dot {
                while self.lexer.current() == Token::Dot {
                    self.lexer.next();
                    expr = self.parse_property_lookup(expr)?;
                }
                remove_items.push(expr);
            } else if self.lexer.current() == Token::Colon {
                expr = tree!(
                    ExprIR::FuncInvocation(
                        get_functions().get("node_set_labels", &FnType::Internal)?
                    ),
                    expr,
                    tree!(ExprIR::List; self.parse_labels()?.into_iter().map(|l| tree!(ExprIR::String(l))))
                );
                remove_items.push(expr);
            } else {
                return Err(self
                    .lexer
                    .format_error(format!("Invalid input {:?}", self.lexer.current()).as_str()));
            }

            if !optional_match_token!(self.lexer, Comma) {
                break;
            }
        }
        Ok(QueryIR::Remove(remove_items))
    }
}
