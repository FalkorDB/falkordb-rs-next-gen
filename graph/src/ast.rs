use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
};

#[derive(Debug)]
pub enum QueryExprIR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Ident(String),
    Parameter(String),
    Named(String, Box<QueryExprIR>),
    List(Vec<QueryExprIR>),
    Or(Vec<QueryExprIR>),
    Xor(Vec<QueryExprIR>),
    And(Vec<QueryExprIR>),
    Not(Box<QueryExprIR>),
    Eq(Vec<QueryExprIR>),
    Neq(Vec<QueryExprIR>),
    Lt(Vec<QueryExprIR>),
    Gt(Vec<QueryExprIR>),
    Le(Vec<QueryExprIR>),
    Ge(Vec<QueryExprIR>),
    In(Box<(QueryExprIR, QueryExprIR)>),
    Add(Vec<QueryExprIR>),
    Sub(Vec<QueryExprIR>),
    Mul(Vec<QueryExprIR>),
    Div(Vec<QueryExprIR>),
    Pow(Vec<QueryExprIR>),
    IsNull(Box<QueryExprIR>),
    GetElement(Box<(QueryExprIR, QueryExprIR)>),
    GetElements(Box<(QueryExprIR, Option<QueryExprIR>, Option<QueryExprIR>)>),
    Property(Box<QueryExprIR>, String),
    FuncInvocation(String, Vec<QueryExprIR>),
    Map(BTreeMap<String, QueryExprIR>),
    StartsWith(Box<(QueryExprIR, QueryExprIR)>),
    EndsWith(Box<(QueryExprIR, QueryExprIR)>),
    Contains(Box<(QueryExprIR, QueryExprIR)>),
    RegexMatches(Box<(QueryExprIR, QueryExprIR)>),
}

impl QueryExprIR {
    fn inner_validate(&self, env: &mut HashSet<String>) -> Result<(), String> {
        match self {
            Self::Null | Self::Bool(_) | Self::Integer(_) | Self::Float(_) | Self::String(_) => {
                Ok(())
            }
            Self::Ident(id) => {
                if env.contains(id) {
                    Ok(())
                } else {
                    Err(format!("'{id}' not defined"))
                }
            }
            Self::Parameter(_) => Ok(()),
            Self::And(exprs) | Self::Or(exprs) | Self::Xor(exprs) => {
                debug_assert!(exprs.len() > 1);
                for expr in exprs {
                    if let _e @ (Self::Integer(_)
                    | Self::Float(_)
                    | Self::String(_)
                    | Self::List(_)
                    | Self::Map(_)) = expr
                    {
                        return Err("Type mismatch: expected bool".to_string());
                    }
                    expr.inner_validate(env)?;
                }
                Ok(())
            }
            Self::List(exprs)
            | Self::Eq(exprs)
            | Self::Neq(exprs)
            | Self::Lt(exprs)
            | Self::Gt(exprs)
            | Self::Le(exprs)
            | Self::Ge(exprs)
            | Self::Add(exprs)
            | Self::Sub(exprs)
            | Self::Mul(exprs)
            | Self::Div(exprs)
            | Self::Pow(exprs) => {
                for expr in exprs {
                    expr.inner_validate(env)?;
                }
                Ok(())
            }
            Self::In(op) => {
                op.0.inner_validate(env)?;
                op.1.inner_validate(env)
            }
            Self::StartsWith(op) => {
                op.0.inner_validate(env)?;
                op.1.inner_validate(env)
            }
            Self::EndsWith(op) => {
                op.0.inner_validate(env)?;
                op.1.inner_validate(env)
            }
            Self::Contains(op) => {
                op.0.inner_validate(env)?;
                op.1.inner_validate(env)
            }
            Self::RegexMatches(op) => {
                op.0.inner_validate(env)?;
                op.1.inner_validate(env)
            }
            Self::Map(exprs) => {
                for expr in exprs.values() {
                    expr.inner_validate(env)?;
                }
                Ok(())
            }
            Self::Named(name, expr) => {
                expr.inner_validate(env)?;
                env.insert(name.to_string());
                Ok(())
            }
            Self::Not(expr) | Self::IsNull(expr) | Self::Property(expr, _) => {
                expr.inner_validate(env)
            }
            Self::GetElement(op) => {
                op.0.inner_validate(env)?;
                op.1.inner_validate(env)
            }
            Self::GetElements(op) => {
                op.0.inner_validate(env)?;
                match (&op.1, &op.2) {
                    (Some(a), Some(b)) => {
                        a.inner_validate(env)?;
                        b.inner_validate(env)
                    }
                    (Some(a), None) => a.inner_validate(env),
                    (None, Some(b)) => b.inner_validate(env),
                    (None, None) => Ok(()),
                }
            }
            Self::FuncInvocation(_name, exprs) => {
                for arg in exprs {
                    arg.inner_validate(env)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Alias {
    String(String),
    Anon(i32),
}

impl Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Anon(id) => write!(f, "@anon{id}"),
        }
    }
}

#[derive(Debug)]
pub struct NodePattern {
    pub alias: Alias,
    pub labels: Vec<String>,
    pub attrs: BTreeMap<String, QueryExprIR>,
}

impl NodePattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        labels: Vec<String>,
        attrs: BTreeMap<String, QueryExprIR>,
    ) -> Self {
        Self {
            alias,
            labels,
            attrs,
        }
    }
}

#[derive(Debug)]
pub struct RelationshipPattern {
    pub alias: Alias,
    pub relationship_type: String,
    pub attrs: BTreeMap<String, QueryExprIR>,
    pub from: Alias,
    pub to: Alias,
    pub bidirectional: bool,
}

impl RelationshipPattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        relationship_type: String,
        attrs: BTreeMap<String, QueryExprIR>,
        from: Alias,
        to: Alias,
        bidirectional: bool,
    ) -> Self {
        Self {
            alias,
            relationship_type,
            attrs,
            from,
            to,
            bidirectional,
        }
    }
}

#[derive(Debug)]
pub struct PathPattern {
    pub vars: Vec<Alias>,
    pub name: String,
}

impl PathPattern {
    #[must_use]
    pub const fn new(vars: Vec<Alias>, name: String) -> Self {
        Self { vars, name }
    }
}

#[derive(Debug)]
pub struct Pattern {
    pub nodes: Vec<NodePattern>,
    pub relationships: Vec<RelationshipPattern>,
    pub paths: Vec<PathPattern>,
}

impl Pattern {
    #[must_use]
    pub const fn new(
        nodes: Vec<NodePattern>,
        relationships: Vec<RelationshipPattern>,
        paths: Vec<PathPattern>,
    ) -> Self {
        Self {
            nodes,
            relationships,
            paths,
        }
    }
}

#[derive(Debug)]
pub enum QueryIR {
    Call(String, Vec<QueryExprIR>),
    Match(Pattern),
    Unwind(QueryExprIR, String),
    Where(QueryExprIR),
    Create(Pattern),
    Delete(Vec<QueryExprIR>),
    With(Vec<QueryExprIR>),
    Return(Vec<QueryExprIR>),
    Query(Vec<QueryIR>),
}

impl QueryIR {
    pub fn validate(&self) -> Result<(), String> {
        let mut env = HashSet::new();
        self.inner_validate(std::iter::empty(), &mut env)
    }

    fn inner_validate<'a, T>(&self, mut iter: T, env: &mut HashSet<String>) -> Result<(), String>
    where
        T: Iterator<Item = &'a Self>,
    {
        match self {
            Self::Call(_, args) => {
                for arg in args {
                    arg.inner_validate(env)?;
                }
                Ok(())
            }
            Self::Match(p) => {
                for node in &p.nodes {
                    if env.contains(&node.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            node.alias.to_string().as_str()
                        ));
                    }
                    for v in node.attrs.values() {
                        v.inner_validate(env)?;
                    }
                    env.insert(node.alias.to_string());
                }
                for relationship in &p.relationships {
                    if env.contains(&relationship.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            relationship.alias.to_string().as_str()
                        ));
                    }
                    for v in relationship.attrs.values() {
                        v.inner_validate(env)?;
                    }
                    env.insert(relationship.alias.to_string());
                }
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Unwind(l, v) => {
                l.inner_validate(env)?;
                if env.contains(v) {
                    return Err(format!("Duplicate alias {v}"));
                }
                env.insert((*v).to_string());
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Where(expr) => expr.inner_validate(env),
            Self::Create(p) => {
                for path in &p.paths {
                    if env.contains(&path.name) {
                        return Err(format!(
                            "The bound variable {} can't be redeclared in a create clause",
                            path.name
                        ));
                    }
                    env.insert(path.name.to_string());
                }
                for node in &p.nodes {
                    if env.contains(&node.alias.to_string()) {
                        return Err(format!(
                            "The bound variable {} can't be redeclared in a create clause",
                            node.alias.to_string().as_str()
                        ));
                    }
                    for v in node.attrs.values() {
                        v.inner_validate(env)?;
                    }
                }
                for node in &p.nodes {
                    env.insert(node.alias.to_string());
                }
                for relationship in &p.relationships {
                    if env.contains(&relationship.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            relationship.alias.to_string().as_str()
                        ));
                    }
                    for v in relationship.attrs.values() {
                        v.inner_validate(env)?;
                    }
                    env.insert(relationship.alias.to_string());
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Delete(exprs) | Self::With(exprs) | Self::Return(exprs) => {
                for expr in exprs {
                    expr.inner_validate(env)?;
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Query(q) => {
                let mut iter = q.iter();
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
        }
    }
}
