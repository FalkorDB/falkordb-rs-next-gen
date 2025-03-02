use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
pub enum QueryExprIR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Ident(String),
    Param(String),
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
    Add(Vec<QueryExprIR>),
    Sub(Vec<QueryExprIR>),
    Mul(Vec<QueryExprIR>),
    Div(Vec<QueryExprIR>),
    Pow(Vec<QueryExprIR>),
    IsNull(Box<QueryExprIR>),
    GetElement(Box<(QueryExprIR, QueryExprIR)>),
    Property(Box<QueryExprIR>, String),
    FuncInvocation(String, Vec<QueryExprIR>),
    Map(Vec<(String, QueryExprIR)>),
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
                    Err(format!("Unknown identifier {id}"))
                }
            }
            Self::Param(_) => todo!(),
            Self::List(exprs)
            | Self::Or(exprs)
            | Self::Xor(exprs)
            | Self::And(exprs)
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
            Self::Map(exprs) => {
                for (_, expr) in exprs {
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
            Self::FuncInvocation(name, exprs) => {
                if name != "range" {
                    return Err(format!("Unknown function {name}"));
                }
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
    pub attrs: Vec<(String, QueryExprIR)>,
}

impl NodePattern {
    #[must_use]
    pub const fn new(alias: Alias, labels: Vec<String>, attrs: Vec<(String, QueryExprIR)>) -> Self {
        Self {
            alias,
            labels,
            attrs,
        }
    }
}

#[derive(Debug)]
pub struct LinkPattern {
    pub alias: Alias,
    pub link_type: String,
    pub attrs: Vec<(String, QueryExprIR)>,
    pub from: Alias,
    pub to: Alias,
}

impl LinkPattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        link_type: String,
        attrs: Vec<(String, QueryExprIR)>,
        from: Alias,
        to: Alias,
    ) -> Self {
        Self {
            alias,
            link_type,
            attrs,
            from,
            to,
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
    pub links: Vec<LinkPattern>,
    pub paths: Vec<PathPattern>,
}

impl Pattern {
    #[must_use]
    pub const fn new(
        nodes: Vec<NodePattern>,
        links: Vec<LinkPattern>,
        paths: Vec<PathPattern>,
    ) -> Self {
        Self {
            nodes,
            links,
            paths,
        }
    }
}

#[derive(Debug)]
pub enum QueryIR {
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
        self.inner_validate([].iter(), &mut env)
    }

    fn inner_validate(
        &self,
        mut iter: std::slice::Iter<Self>,
        env: &mut HashSet<String>,
    ) -> Result<(), String> {
        match self {
            Self::Match(p) => {
                for node in &p.nodes {
                    if env.contains(&node.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            node.alias.to_string().as_str()
                        ));
                    }
                    for (_, v) in &node.attrs {
                        v.inner_validate(env)?;
                    }
                    env.insert(node.alias.to_string());
                }
                for link in &p.links {
                    if env.contains(&link.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            link.alias.to_string().as_str()
                        ));
                    }
                    for (_, v) in &link.attrs {
                        v.inner_validate(env)?;
                    }
                    env.insert(link.alias.to_string());
                }
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Unwind(l, v) => {
                l.inner_validate(env)?;
                if matches!(l, QueryExprIR::List(_))
                    || (matches!(l, QueryExprIR::FuncInvocation(name, _) if name == "range"))
                {
                } else {
                    return Err("Expected list or range in UNWIND".to_string());
                }
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
                        return Err(format!("Duplicate alias {}", path.name));
                    }
                    env.insert(path.name.to_string());
                }
                for node in &p.nodes {
                    if env.contains(&node.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            node.alias.to_string().as_str()
                        ));
                    }
                    for (_, v) in &node.attrs {
                        v.inner_validate(env)?;
                    }
                }
                for node in &p.nodes {
                    env.insert(node.alias.to_string());
                }
                for link in &p.links {
                    if env.contains(&link.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            link.alias.to_string().as_str()
                        ));
                    }
                    for (_, v) in &link.attrs {
                        v.inner_validate(env)?;
                    }
                    env.insert(link.alias.to_string());
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
