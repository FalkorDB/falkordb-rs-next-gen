use std::{collections::HashSet, fmt::Display, rc::Rc};

use orx_tree::{Dfs, DynNode, DynTree, NodeRef};

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
    Quantifier(QuantifierType, Rc<String>),
    ListComprehension(Rc<String>),
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
            Self::Quantifier(quantifier_type, var_name) => {
                write!(f, "{quantifier_type} {var_name}")
            }
            Self::ListComprehension(var) => {
                write!(f, "list comp({var})")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum QuantifierType {
    All,
    Any,
    None,
    Single,
}

impl Display for QuantifierType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Any => write!(f, "any"),
            Self::None => write!(f, "none"),
            Self::Single => write!(f, "single"),
        }
    }
}

pub trait Validate {
    fn validate(
        self,
        env: &mut HashSet<Rc<String>>,
    ) -> Result<(), String>;
}

impl Validate for DynNode<'_, ExprIR> {
    #[allow(clippy::too_many_lines)]
    fn validate(
        self,
        env: &mut HashSet<Rc<String>>,
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
            ExprIR::Quantifier(_quantifier_type, var_name) => {
                debug_assert_eq!(self.num_children(), 2);
                self.child(0).validate(env)?;
                env.insert(var_name.clone());
                self.child(1).validate(env)?;
                env.remove(var_name);
                Ok(())
            }
            ExprIR::ListComprehension(var) => {
                debug_assert!(0 < self.num_children() && self.num_children() <= 3);
                self.child(0).validate(env)?;
                env.insert(var.clone());
                for expr in self.children().skip(1) {
                    expr.validate(env)?;
                }
                env.remove(var);
                Ok(())
            }
        }
    }
}

pub trait SupportAggregation {
    fn is_aggregation(&self) -> bool;
}

impl SupportAggregation for DynTree<ExprIR> {
    fn is_aggregation(&self) -> bool {
        self.root().indices::<Dfs>().any(|idx| {
            matches!(
                self.node(&idx).data(),
                ExprIR::FuncInvocation(_, FnType::Aggregation)
            )
        })
    }
}

#[derive(Debug, Clone)]
pub enum Alias {
    String(Rc<String>),
    Anon(i32),
}

impl Alias {
    #[must_use]
    pub fn to_string(&self) -> Rc<String> {
        match self {
            Self::String(s) => s.clone(),
            Self::Anon(i) => Rc::new(format!("${i}")),
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
            return write!(f, "({})", self.alias.to_string());
        }
        write!(
            f,
            "({}:{})",
            self.alias.to_string(),
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
                self.from.to_string(),
                self.alias.to_string(),
                direction,
                self.to.to_string()
            );
        }
        write!(
            f,
            "({})-[{}:{}]-{}({})",
            self.from.to_string(),
            self.alias.to_string(),
            self.types
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join("|"),
            direction,
            self.to.to_string()
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
    pub name: Rc<String>,
}

impl PathPattern {
    #[must_use]
    pub const fn new(
        vars: Vec<Alias>,
        name: Rc<String>,
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
    Call(Rc<String>, Vec<DynTree<ExprIR>>),
    Match(Pattern, bool),
    Unwind(DynTree<ExprIR>, Rc<String>),
    Merge(Pattern),
    Where(DynTree<ExprIR>),
    Create(Pattern),
    Delete(Vec<DynTree<ExprIR>>, bool),
    With(Vec<(Rc<String>, DynTree<ExprIR>)>, bool),
    Return(Vec<(Rc<String>, DynTree<ExprIR>)>, bool),
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
            Self::Delete(exprs, _) => {
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
        env: &mut HashSet<Rc<String>>,
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
                let mut remove = Vec::new();
                for (i, node) in p.nodes.iter().enumerate() {
                    if env.contains(&node.alias.to_string()) {
                        if p.relationships.is_empty() {
                            return Err(format!(
                                "Duplicate alias {}",
                                node.alias.to_string().as_str()
                            ));
                        }
                        remove.push(i);
                    }
                    node.attrs.root().validate(env)?;
                    env.insert(node.alias.to_string());
                }
                remove.reverse();
                for i in remove {
                    p.nodes.remove(i);
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
                for path in &p.paths {
                    if env.contains(&path.name) {
                        return Err(format!("Duplicate alias {}", path.name.as_str()));
                    }
                    env.insert(path.name.clone());
                }
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Unwind(l, v) => {
                l.root().validate(env)?;
                if env.contains(v) {
                    return Err(format!("Duplicate alias {v}"));
                }
                env.insert(v.clone());
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
                    env.insert(path.name.clone());
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
            Self::Delete(exprs, _) => {
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
                if !exprs.is_empty() {
                    env.clear();
                    for (name, _) in exprs {
                        env.insert(name.clone());
                    }
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
