use std::{collections::HashSet, fmt::Display, rc::Rc};

use orx_tree::{DynNode, DynTree, NodeRef};

use crate::functions::{FnType, get_functions};

#[derive(Clone, Debug)]
pub enum ExprIR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Rc<String>),
    Var(Rc<String>),
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
    FuncInvocation(String, FnType),
    Map,
}

impl Display for ExprIR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Var(id) => write!(f, "{id}"),
            Self::Parameter(p) => write!(f, "@{p}"),
            Self::List => write!(f, "[]"),
            Self::Length => write!(f, "length()"),
            Self::GetElement => write!(f, "get_element()"),
            Self::GetElements => write!(f, "get_elements()"),
            Self::IsNull => write!(f, "is_null()"),
            Self::IsNode => write!(f, "is_node()"),
            Self::IsRelationship => write!(f, "is_relationship()"),
            Self::Or => write!(f, "or()"),
            Self::Xor => write!(f, "xor()"),
            Self::And => write!(f, "and()"),
            Self::Not => write!(f, "not()"),
            Self::Negate => write!(f, "-negate()"),
            Self::Eq => write!(f, "="),
            Self::Neq => write!(f, "<>"),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Le => write!(f, "<="),
            Self::Ge => write!(f, ">="),
            Self::In => write!(f, "in()"),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Pow => write!(f, "^"),
            Self::Modulo => write!(f, "%"),
            Self::FuncInvocation(name, _) => write!(f, "{name}()"),
            Self::Map => write!(f, "{{}}"),
        }
    }
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
                debug_assert_eq!(self.num_children(), 0);
                if env.contains(id.as_str()) {
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
            | ExprIR::GetElement => {
                for expr in self.children() {
                    expr.validate(env)?;
                }
                Ok(())
            }
            ExprIR::FuncInvocation(name, fn_type) => {
                get_functions().validate(name, fn_type, self.num_children())?;
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
        }
    }
}

pub trait SupportAggregation {
    fn is_aggregation(&self) -> bool;
}

impl SupportAggregation for DynNode<'_, ExprIR> {
    fn is_aggregation(&self) -> bool {
        match self.data() {
            ExprIR::FuncInvocation(_, FnType::Aggregation) => true,
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
    pub labels: Vec<Rc<String>>,
    pub attrs: DynTree<ExprIR>,
}

impl Display for NodePattern {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.labels.is_empty() {
            return write!(f, "({})", self.alias);
        }
        write!(
            f,
            "({}:{})",
            self.alias,
            self.labels
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join(":")
        )
    }
}

impl NodePattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        labels: Vec<Rc<String>>,
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
    pub types: Vec<Rc<String>>,
    pub attrs: DynTree<ExprIR>,
    pub from: Alias,
    pub to: Alias,
    pub bidirectional: bool,
}

impl Display for RelationshipPattern {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let direction = if self.bidirectional { "" } else { ">" };
        if self.types.is_empty() {
            return write!(
                f,
                "({})-[{}]-{}({})",
                self.from, self.alias, direction, self.to
            );
        }
        write!(
            f,
            "({})-[{}:{}]-{}({})",
            self.from,
            self.alias,
            self.types
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join("|"),
            direction,
            self.to
        )
    }
}

impl RelationshipPattern {
    #[must_use]
    pub const fn new(
        alias: Alias,
        types: Vec<Rc<String>>,
        attrs: DynTree<ExprIR>,
        from: Alias,
        to: Alias,
        bidirectional: bool,
    ) -> Self {
        Self {
            alias,
            types,
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

impl Display for Pattern {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for node in &self.nodes {
            write!(f, "{node}, ")?;
        }
        for relationship in &self.relationships {
            write!(f, "{relationship}, ")?;
        }
        for path in &self.paths {
            write!(f, "{path:?}, ")?;
        }
        Ok(())
    }
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
    Match(Pattern, bool),
    Unwind(DynTree<ExprIR>, String),
    Merge(Pattern),
    Where(DynTree<ExprIR>),
    Create(Pattern),
    Delete(Vec<DynTree<ExprIR>>),
    With(Vec<(String, DynTree<ExprIR>)>, bool),
    Return(Vec<(String, DynTree<ExprIR>)>, bool),
    Query(Vec<QueryIR>, bool),
}

impl Display for QueryIR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Call(name, args) => {
                writeln!(f, "{name}():")?;
                for arg in args {
                    write!(f, "{arg}")?;
                }
                Ok(())
            }
            Self::Match(p, _) => writeln!(f, "MATCH {p}"),
            Self::Unwind(l, v) => {
                writeln!(f, "UNWIND {v}:")?;
                write!(f, "{l}")
            }
            Self::Merge(p) => writeln!(f, "MERGE {p}"),
            Self::Where(expr) => {
                writeln!(f, "WHERE:")?;
                write!(f, "{expr}")
            }
            Self::Create(p) => write!(f, "CREATE {p}"),
            Self::Delete(exprs) => {
                writeln!(f, "DELETE:")?;
                for expr in exprs {
                    write!(f, "{expr}")?;
                }
                Ok(())
            }
            Self::With(exprs, _) => {
                writeln!(f, "WITH:")?;
                for (name, _) in exprs {
                    write!(f, "{name}")?;
                }
                Ok(())
            }
            Self::Return(exprs, _) => {
                writeln!(f, "RETURN:")?;
                for (name, _) in exprs {
                    write!(f, "{name}")?;
                }
                Ok(())
            }
            Self::Query(qs, _) => {
                for q in qs {
                    write!(f, "{q}")?;
                }
                Ok(())
            }
        }
    }
}

impl QueryIR {
    pub fn validate(&mut self) -> Result<(), String> {
        let mut env = HashSet::new();
        self.inner_validate(std::iter::empty(), &mut env)
    }

    #[allow(clippy::too_many_lines)]
    fn inner_validate<'a, T>(
        &mut self,
        mut iter: T,
        env: &mut HashSet<String>,
    ) -> Result<(), String>
    where
        T: Iterator<Item = &'a mut Self>,
    {
        match self {
            Self::Call(_, args) => {
                for arg in args {
                    arg.root().validate(env)?;
                }
                Ok(())
            }
            Self::Match(p, _) => {
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
            Self::Merge(p) => {
                let mut remove = Vec::new();
                for (i, node) in p.nodes.iter().enumerate() {
                    if env.contains(&node.alias.to_string()) {
                        if p.relationships.is_empty() {
                            return Err(format!(
                                "The bound variable {} can't be redeclared in a create clause",
                                node.alias.to_string().as_str()
                            ));
                        }
                        remove.push(i);
                    }
                    node.attrs.root().validate(env)?;
                }
                remove.reverse();
                for i in remove {
                    p.nodes.remove(i);
                }
                for node in &p.nodes {
                    env.insert(node.alias.to_string());
                }
                for relationship in &p.relationships {
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.to_string());
                }
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
                let mut remove = Vec::new();
                for (i, node) in p.nodes.iter().enumerate() {
                    if env.contains(&node.alias.to_string()) {
                        if p.relationships.is_empty() {
                            return Err(format!(
                                "The bound variable {} can't be redeclared in a create clause",
                                node.alias.to_string().as_str()
                            ));
                        }
                        remove.push(i);
                    }
                    node.attrs.root().validate(env)?;
                }
                remove.reverse();
                for i in remove {
                    p.nodes.remove(i);
                }
                for node in &p.nodes {
                    env.insert(node.alias.to_string());
                }
                for relationship in &p.relationships {
                    if env.contains(&relationship.alias.to_string()) {
                        return Err(format!(
                            "The bound variable '{}' can't be redeclared in a CREATE clause",
                            relationship.alias.to_string().as_str()
                        ));
                    }
                    if relationship.types.len() != 1 {
                        return Err(String::from(
                            "Exactly one relationship type must be specified for each relation in a CREATE pattern.",
                        ));
                    }
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.to_string());
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Delete(exprs) => {
                for expr in exprs {
                    expr.root().validate(env)?;
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::With(exprs, _) | Self::Return(exprs, _) => {
                for (_, expr) in exprs.iter() {
                    expr.root().validate(env)?;
                }
                for (name, _) in exprs {
                    env.insert(name.clone());
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Query(q, _) => {
                let mut iter = q.iter_mut();
                let first = iter.next().ok_or("Empty query")?;
                first.inner_validate(iter, env)
            }
        }
    }
}
