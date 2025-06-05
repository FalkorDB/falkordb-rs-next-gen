use std::{collections::HashSet, fmt::Display, hash::Hash, rc::Rc};

use orx_tree::{Dfs, DynNode, DynTree, NodeRef};

use crate::functions::{FnType, get_functions};

#[derive(Clone, Debug)]
pub struct VarId {
    pub name: Option<Rc<String>>,
    pub id: u32,
}

impl PartialEq for VarId {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.id == other.id
    }
}

impl Eq for VarId {}

impl Hash for VarId {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.id.hash(state);
    }
}

impl VarId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.name.as_ref().map_or("?", |n| n.as_str())
    }
}

#[derive(Clone, Debug)]
pub enum ExprIR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Rc<String>),
    List,
    Map,
    Var(VarId),
    Parameter(String),
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
    Quantifier(QuantifierType, VarId),
    ListComprehension(VarId),
    ExistsPattern(Pattern),
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
            Self::List => write!(f, "[]"),
            Self::Map => write!(f, "{{}}"),
            Self::Var(id) => write!(f, "{}", id.as_str()),
            Self::Parameter(p) => write!(f, "@{p}"),
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
            Self::Quantifier(quantifier_type, var) => {
                write!(f, "{quantifier_type} {}", var.as_str())
            }
            Self::ListComprehension(var) => {
                write!(f, "list comp({})", var.as_str())
            }
            Self::ExistsPattern(pattern) => {
                write!(f, "exists({})", pattern)
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
        env: &mut HashSet<u32>,
    ) -> Result<(), String>;
}

impl Validate for DynNode<'_, ExprIR> {
    #[allow(clippy::too_many_lines)]
    fn validate(
        self,
        env: &mut HashSet<u32>,
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
            ExprIR::Var(var) => {
                debug_assert_eq!(self.num_children(), 0);
                if env.contains(&var.id) {
                    Ok(())
                } else {
                    Err(format!("'{}' not defined", var.as_str()))
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
            ExprIR::FuncInvocation(name, FnType::Aggregation) => {
                get_functions().validate(name, &FnType::Aggregation, self.num_children())?;
                for i in 0..self.num_children() - 1 {
                    self.child(i).validate(env)?;
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
                    debug_assert!(matches!(expr.data(), ExprIR::String(_)));
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
            ExprIR::Quantifier(_quantifier_type, var) => {
                debug_assert_eq!(self.num_children(), 2);
                self.child(0).validate(env)?;
                env.insert(var.id);
                self.child(1).validate(env)?;
                env.remove(&var.id);
                Ok(())
            }
            ExprIR::ListComprehension(var) => {
                debug_assert!(0 < self.num_children() && self.num_children() <= 3);
                self.child(0).validate(env)?;
                env.insert(var.id);
                for expr in self.children().skip(1) {
                    expr.validate(env)?;
                }
                env.remove(&var.id);
                Ok(())
            }
            ExprIR::ExistsPattern(_pattern) => {
                debug_assert_eq!(self.num_children(), 0);
                // ExistsPattern validation would typically involve checking
                // that the pattern itself is valid, but for now we accept it
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

#[derive(Clone, Debug)]
pub struct NodePattern {
    pub alias: VarId,
    pub labels: Vec<Rc<String>>,
    pub attrs: DynTree<ExprIR>,
}

impl Display for NodePattern {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.labels.is_empty() {
            return write!(f, "({})", self.alias.as_str());
        }
        write!(
            f,
            "({}:{})",
            self.alias.as_str(),
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
        alias: VarId,
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
    pub alias: VarId,
    pub types: Vec<Rc<String>>,
    pub attrs: DynTree<ExprIR>,
    pub from: VarId,
    pub to: VarId,
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
                self.from.as_str(),
                self.alias.as_str(),
                direction,
                self.to.as_str()
            );
        }
        write!(
            f,
            "({})-[{}:{}]-{}({})",
            self.from.as_str(),
            self.alias.as_str(),
            self.types
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join("|"),
            direction,
            self.to.as_str()
        )
    }
}

impl RelationshipPattern {
    #[must_use]
    pub const fn new(
        alias: VarId,
        types: Vec<Rc<String>>,
        attrs: DynTree<ExprIR>,
        from: VarId,
        to: VarId,
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
    pub var: VarId,
    pub vars: Vec<VarId>,
}

impl PathPattern {
    #[must_use]
    pub const fn new(
        var: VarId,
        vars: Vec<VarId>,
    ) -> Self {
        Self { var, vars }
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
    Unwind(DynTree<ExprIR>, VarId),
    Merge(Pattern),
    Where(DynTree<ExprIR>),
    Create(Pattern),
    Delete(Vec<DynTree<ExprIR>>, bool),
    With(Vec<(VarId, DynTree<ExprIR>)>, bool),
    Return(Vec<(VarId, DynTree<ExprIR>)>, bool),
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
                writeln!(f, "UNWIND {}:", v.as_str())?;
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
                    write!(f, "{}", name.as_str())?;
                }
                Ok(())
            }
            Self::Return(exprs, _) => {
                writeln!(f, "RETURN:")?;
                for (name, _) in exprs {
                    write!(f, "{}", name.as_str())?;
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
        env: &mut HashSet<u32>,
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
                    if env.contains(&node.alias.id) {
                        if p.relationships.is_empty() {
                            return Err(format!("Duplicate alias {}", node.alias.as_str()));
                        }
                        remove.push(i);
                    }
                    node.attrs.root().validate(env)?;
                    env.insert(node.alias.id);
                }
                remove.reverse();
                for i in remove {
                    p.nodes.remove(i);
                }
                for relationship in &p.relationships {
                    if env.contains(&relationship.alias.id) {
                        return Err(format!("Duplicate alias {}", relationship.alias.as_str()));
                    }
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.id);
                }
                for path in &p.paths {
                    if env.contains(&path.var.id) {
                        return Err(format!("Duplicate alias {}", path.var.as_str()));
                    }
                    env.insert(path.var.id);
                }
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Unwind(l, v) => {
                l.root().validate(env)?;
                if env.contains(&v.id) {
                    return Err(format!("Duplicate alias {}", v.as_str()));
                }
                env.insert(v.id);
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Merge(p) => {
                let mut remove = Vec::new();
                for (i, node) in p.nodes.iter().enumerate() {
                    if env.contains(&node.alias.id) {
                        if p.relationships.is_empty() {
                            return Err(format!(
                                "The bound variable {} can't be redeclared in a create clause",
                                node.alias.as_str()
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
                    env.insert(node.alias.id);
                }
                for relationship in &p.relationships {
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.id);
                }
                let first = iter.next().unwrap();
                first.inner_validate(iter, env)
            }
            Self::Where(expr) => expr.root().validate(env),
            Self::Create(p) => {
                for path in &p.paths {
                    if env.contains(&path.var.id) {
                        return Err(format!(
                            "The bound variable {} can't be redeclared in a create clause",
                            path.var.as_str()
                        ));
                    }
                    env.insert(path.var.id);
                }
                let mut remove = Vec::new();
                for (i, node) in p.nodes.iter().enumerate() {
                    if env.contains(&node.alias.id) {
                        if p.relationships.is_empty() {
                            return Err(format!(
                                "The bound variable {} can't be redeclared in a create clause",
                                node.alias.as_str()
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
                    env.insert(node.alias.id);
                }
                for relationship in &p.relationships {
                    if env.contains(&relationship.alias.id) {
                        return Err(format!(
                            "The bound variable '{}' can't be redeclared in a CREATE clause",
                            relationship.alias.as_str()
                        ));
                    }
                    if relationship.types.len() != 1 {
                        return Err(String::from(
                            "Exactly one relationship type must be specified for each relation in a CREATE pattern.",
                        ));
                    }
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.id);
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
                        env.insert(name.id);
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
