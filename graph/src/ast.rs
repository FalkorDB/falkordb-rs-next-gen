use std::{collections::HashSet, fmt::Display};

use orx_tree::{DynNode, DynTree, NodeRef};

#[derive(Clone, Debug)]
pub enum ExprIR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Var(String),
    Parameter(String),
    List,
    Length,
    GetElement,
    GetElements,
    IsNull,
    IsNode,
    IsRelationship,
    Or,
    Xor,
    And,
    Not,
    Negate,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    In,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Modulo,
    FuncInvocation(String),
    Map,
    Set(String),
}

pub trait Validate {
    fn validate(
        self,
        env: &mut HashSet<String>,
    ) -> Result<(), String>;
}

impl Validate for DynNode<'_, ExprIR> {
    fn validate(
        self,
        env: &mut HashSet<String>,
    ) -> Result<(), String> {
        match self.data() {
            ExprIR::Null
            | ExprIR::Bool(_)
            | ExprIR::Integer(_)
            | ExprIR::Float(_)
            | ExprIR::String(_)
            | ExprIR::Parameter(_) => {
                debug_assert_eq!(self.num_children(), 0);
                Ok(())
            }
            ExprIR::Var(id) => {
                debug_assert!(self.num_children() <= 1);
                if env.contains(id) {
                    Ok(())
                } else {
                    Err(format!("'{id}' not defined"))
                }
            }
            ExprIR::And | ExprIR::Or | ExprIR::Xor => {
                debug_assert!(self.num_children() >= 2);
                for expr in self.children() {
                    if let _e @ (ExprIR::Integer(_)
                    | ExprIR::Float(_)
                    | ExprIR::String(_)
                    | ExprIR::List
                    | ExprIR::Map) = expr.data()
                    {
                        return Err("Type mismatch: expected bool".to_string());
                    }
                    expr.validate(env)?;
                }
                Ok(())
            }
            ExprIR::List
            | ExprIR::Eq
            | ExprIR::Neq
            | ExprIR::Lt
            | ExprIR::Gt
            | ExprIR::Le
            | ExprIR::Ge
            | ExprIR::Add
            | ExprIR::Sub
            | ExprIR::Mul
            | ExprIR::Div
            | ExprIR::Pow
            | ExprIR::Modulo
            | ExprIR::GetElement
            | ExprIR::FuncInvocation(_) => {
                for expr in self.children() {
                    expr.validate(env)?;
                }
                Ok(())
            }
            ExprIR::Map => {
                for expr in self.children() {
                    debug_assert!(matches!(expr.data(), ExprIR::Var(_)));
                    debug_assert_eq!(expr.num_children(), 1);
                    expr.child(0).validate(env)?;
                }
                Ok(())
            }
            ExprIR::In => {
                debug_assert_eq!(self.num_children(), 2);
                for expr in self.children() {
                    expr.validate(env)?;
                }
                Ok(())
            }
            ExprIR::Not
            | ExprIR::IsNull
            | ExprIR::Negate
            | ExprIR::Length
            | ExprIR::IsNode
            | ExprIR::IsRelationship => {
                debug_assert_eq!(self.num_children(), 1);
                self.child(0).validate(env)
            }
            ExprIR::GetElements => {
                debug_assert_eq!(self.num_children(), 3);
                for expr in self.children() {
                    expr.validate(env)?;
                }
                Ok(())
            }
            ExprIR::Set(x) => {
                debug_assert_eq!(self.num_children(), 1);
                self.child(0).validate(env)?;
                env.insert(x.to_string());
                Ok(())
            }
        }
    }
}

pub trait SupportAggregation {
    fn is_aggregation(&self) -> bool;
}

impl SupportAggregation for DynNode<'_, ExprIR> {
    fn is_aggregation(&self) -> bool {
        match self.data() {
            ExprIR::FuncInvocation(name) => {
                matches!(
                    name.as_str(),
                    "count" | "sum" | "avg" | "min" | "max" | "collect"
                )
            }
            ExprIR::Set(_) => self.child(0).is_aggregation(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Alias {
    String(String),
    Anon(i32),
}

impl Display for Alias {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Anon(id) => write!(f, "@anon{id}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodePattern {
    pub alias: Alias,
    pub labels: Vec<String>,
    pub attrs: DynTree<ExprIR>,
}

impl NodePattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        labels: Vec<String>,
        attrs: DynTree<ExprIR>,
    ) -> Self {
        Self {
            alias,
            labels,
            attrs,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RelationshipPattern {
    pub alias: Alias,
    pub relationship_type: String,
    pub attrs: DynTree<ExprIR>,
    pub from: Alias,
    pub to: Alias,
    pub bidirectional: bool,
}

impl RelationshipPattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        relationship_type: String,
        attrs: DynTree<ExprIR>,
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

#[derive(Clone, Debug)]
pub struct PathPattern {
    pub vars: Vec<Alias>,
    pub name: String,
}

impl PathPattern {
    #[must_use]
    pub const fn new(
        vars: Vec<Alias>,
        name: String,
    ) -> Self {
        Self { vars, name }
    }
}

#[derive(Clone, Debug)]
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
    Call(String, Vec<DynTree<ExprIR>>),
    Match(Pattern),
    Unwind(DynTree<ExprIR>, String),
    Where(DynTree<ExprIR>),
    Create(Pattern),
    Delete(Vec<DynTree<ExprIR>>),
    With(Vec<DynTree<ExprIR>>, bool),
    Return(Vec<DynTree<ExprIR>>, bool),
    Query(Vec<QueryIR>, bool),
}

impl QueryIR {
    pub fn validate(&self) -> Result<(), String> {
        let mut env = HashSet::new();
        self.inner_validate(std::iter::empty(), &mut env)
    }

    fn inner_validate<'a, T>(
        &self,
        mut iter: T,
        env: &mut HashSet<String>,
    ) -> Result<(), String>
    where
        T: Iterator<Item = &'a Self>,
    {
        match self {
            Self::Call(_, args) => {
                for arg in args {
                    arg.root().validate(env)?;
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
                    node.attrs.root().validate(env)?;
                    env.insert(node.alias.to_string());
                }
                for relationship in &p.relationships {
                    if env.contains(&relationship.alias.to_string()) {
                        return Err(format!(
                            "Duplicate alias {}",
                            relationship.alias.to_string().as_str()
                        ));
                    }
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.to_string());
                }
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Unwind(l, v) => {
                l.root().validate(env)?;
                if env.contains(v) {
                    return Err(format!("Duplicate alias {v}"));
                }
                env.insert((*v).to_string());
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Where(expr) => expr.root().validate(env),
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
                    node.attrs.root().validate(env)?;
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
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.to_string());
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Delete(exprs) | Self::With(exprs, _) | Self::Return(exprs, _) => {
                for expr in exprs {
                    expr.root().validate(env)?;
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Query(q, _) => {
                let mut iter = q.iter();
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
        }
    }
}
