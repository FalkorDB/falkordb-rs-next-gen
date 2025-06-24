use std::{collections::HashSet, fmt::Display, hash::Hash, rc::Rc};

use ordermap::{OrderMap, OrderSet};
use orx_tree::{Dfs, DynNode, DynTree, NodeRef};

use crate::functions::{GraphFn, Type};

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: Option<Rc<String>>,
    pub id: u32,
    pub ty: Type,
}

impl PartialEq for Variable {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.id == other.id
    }
}

impl Eq for Variable {}

impl Hash for Variable {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.id.hash(state);
    }
}

impl Variable {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.name.as_ref().map_or("?", |n| n.as_str())
    }
}

#[derive(Debug)]
pub enum ExprIR {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Rc<String>),
    List,
    Map,
    Variable(Variable),
    Parameter(String),
    Length,
    GetElement,
    GetElements,
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
    Distinct,
    FuncInvocation(Rc<GraphFn>),
    Quantifier(QuantifierType, Variable),
    ListComprehension(Variable),
}

#[cfg_attr(tarpaulin, skip)]
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
            Self::Variable(id) => write!(f, "{}", id.as_str()),
            Self::Parameter(p) => write!(f, "@{p}"),
            Self::Length => write!(f, "length()"),
            Self::GetElement => write!(f, "get_element()"),
            Self::GetElements => write!(f, "get_elements()"),
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
            Self::Distinct => write!(f, "distinct"),
            Self::FuncInvocation(func) => write!(f, "{}()", func.name),
            Self::Quantifier(quantifier_type, var) => {
                write!(f, "{quantifier_type} {}", var.as_str())
            }
            Self::ListComprehension(var) => {
                write!(f, "list comp({})", var.as_str())
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

#[cfg_attr(tarpaulin, skip)]
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
    #[allow(clippy::cognitive_complexity)]
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
            ExprIR::Variable(var) => {
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
            ExprIR::FuncInvocation(func) => {
                if func.is_aggregate() {
                    for i in 0..self.num_children() - 1 {
                        self.child(i).validate(env)?;
                    }
                } else {
                    for expr in self.children() {
                        expr.validate(env)?;
                    }
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
            | ExprIR::Negate
            | ExprIR::Length
            | ExprIR::IsNode
            | ExprIR::IsRelationship
            | ExprIR::Distinct => {
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
                ExprIR::FuncInvocation(func) if func.is_aggregate()
            )
        })
    }
}

#[derive(Debug)]
pub struct QueryNode {
    pub alias: Variable,
    pub labels: OrderSet<Rc<String>>,
    pub attrs: Rc<DynTree<ExprIR>>,
}

#[cfg_attr(tarpaulin, skip)]
impl Display for QueryNode {
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

impl QueryNode {
    #[must_use]
    pub const fn new(
        alias: Variable,
        labels: OrderSet<Rc<String>>,
        attrs: Rc<DynTree<ExprIR>>,
    ) -> Self {
        Self {
            alias,
            labels,
            attrs,
        }
    }
}

#[derive(Debug)]
pub struct QueryRelationship {
    pub alias: Variable,
    pub types: Vec<Rc<String>>,
    pub attrs: Rc<DynTree<ExprIR>>,
    pub from: Rc<QueryNode>,
    pub to: Rc<QueryNode>,
    pub bidirectional: bool,
}

#[cfg_attr(tarpaulin, skip)]
impl Display for QueryRelationship {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let direction = if self.bidirectional { "" } else { ">" };
        if self.types.is_empty() {
            return write!(
                f,
                "({})-[{}]-{}({})",
                self.from.alias.as_str(),
                self.alias.as_str(),
                direction,
                self.to.alias.as_str()
            );
        }
        write!(
            f,
            "({})-[{}:{}]-{}({})",
            self.from.alias.as_str(),
            self.alias.as_str(),
            self.types
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join("|"),
            direction,
            self.to.alias.as_str()
        )
    }
}

impl QueryRelationship {
    #[must_use]
    pub const fn new(
        alias: Variable,
        types: Vec<Rc<String>>,
        attrs: Rc<DynTree<ExprIR>>,
        from: Rc<QueryNode>,
        to: Rc<QueryNode>,
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

#[derive(Debug)]
pub struct QueryPath {
    pub var: Variable,
    pub vars: Vec<Variable>,
}

impl QueryPath {
    #[must_use]
    pub const fn new(
        var: Variable,
        vars: Vec<Variable>,
    ) -> Self {
        Self { var, vars }
    }
}

#[derive(Clone, Debug, Default)]
pub struct QueryGraph {
    nodes: OrderMap<Variable, Rc<QueryNode>>,
    relationships: OrderMap<Variable, Rc<QueryRelationship>>,
    paths: OrderMap<Variable, Rc<QueryPath>>,
}

#[cfg_attr(tarpaulin, skip)]
impl Display for QueryGraph {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for node in self.nodes.values() {
            write!(f, "{node}, ")?;
        }
        for relationship in self.relationships.values() {
            write!(f, "{relationship}, ")?;
        }
        for path in self.paths.values() {
            write!(f, "{path:?}, ")?;
        }
        Ok(())
    }
}

impl QueryGraph {
    pub fn add_node(
        &mut self,
        node: Rc<QueryNode>,
    ) -> bool {
        self.nodes.insert(node.alias.clone(), node).is_none()
    }

    pub fn add_relationship(
        &mut self,
        relationship: Rc<QueryRelationship>,
    ) -> bool {
        self.relationships
            .insert(relationship.alias.clone(), relationship)
            .is_none()
    }

    pub fn add_path(
        &mut self,
        path: Rc<QueryPath>,
    ) -> bool {
        self.paths.insert(path.var.clone(), path).is_none()
    }

    #[must_use]
    pub fn variables(&self) -> Vec<Variable> {
        self.nodes
            .keys()
            .chain(self.relationships.keys())
            .chain(self.paths.keys())
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn nodes(&self) -> Vec<Rc<QueryNode>> {
        self.nodes.values().cloned().collect()
    }

    #[must_use]
    pub fn relationships(&self) -> Vec<Rc<QueryRelationship>> {
        self.relationships.values().cloned().collect()
    }

    #[must_use]
    pub fn paths(&self) -> Vec<Rc<QueryPath>> {
        self.paths.values().cloned().collect()
    }

    #[must_use]
    pub fn filter_visited(
        &self,
        visited: &HashSet<u32>,
    ) -> Self {
        let mut res = Self::default();
        for node in self.nodes.values() {
            if !visited.contains(&node.alias.id) {
                res.add_node(node.clone());
            }
        }
        for relationship in self.relationships.values() {
            if !visited.contains(&relationship.alias.id) {
                res.add_relationship(relationship.clone());
            }
        }
        for path in self.paths.values() {
            if !visited.contains(&path.var.id) {
                res.add_path(path.clone());
            }
        }
        res
    }

    #[must_use]
    pub fn connected_components(&self) -> Vec<Self> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in self.nodes.values() {
            if !visited.contains(&node.alias.id) {
                let mut component = Self::default();

                self.dfs(node, &mut visited, &mut component);

                components.push(component);
            }
        }

        components
    }

    fn dfs(
        &self,
        node: &Rc<QueryNode>,
        visited: &mut HashSet<u32>,
        component: &mut Self,
    ) {
        visited.insert(node.alias.id);
        component.add_node(node.clone());

        for relationship in self.relationships.values() {
            if relationship.from.alias.id == node.alias.id {
                if visited.insert(relationship.alias.id) {
                    component.add_relationship(relationship.clone());
                }
                if !visited.contains(&relationship.to.alias.id) {
                    self.dfs(&relationship.to, visited, component);
                }
            } else if relationship.to.alias.id == node.alias.id {
                if visited.insert(relationship.alias.id) {
                    component.add_relationship(relationship.clone());
                }
                if !visited.contains(&relationship.from.alias.id) {
                    self.dfs(&relationship.from, visited, component);
                }
            }
        }

        for path in self.paths.values() {
            if path.vars.iter().any(|id| visited.contains(&id.id)) && visited.insert(path.var.id) {
                debug_assert!(path.vars.iter().all(|id| visited.contains(&id.id)));
                component.add_path(path.clone());
            }
        }
    }
}

#[derive(Debug)]
pub enum QueryIR {
    Call(Rc<String>, Vec<DynTree<ExprIR>>),
    Match {
        pattern: QueryGraph,
        filter: Option<DynTree<ExprIR>>,
        optional: bool,
    },
    Unwind(DynTree<ExprIR>, Variable),
    Merge(QueryGraph),
    Create(QueryGraph),
    Delete(Vec<DynTree<ExprIR>>, bool),
    Set(Vec<(DynTree<ExprIR>, DynTree<ExprIR>, bool)>),
    Remove(Vec<DynTree<ExprIR>>),
    With {
        distinct: bool,
        exprs: Vec<(Variable, DynTree<ExprIR>)>,
        orderby: Vec<(DynTree<ExprIR>, bool)>,
        skip: Option<DynTree<ExprIR>>,
        limit: Option<DynTree<ExprIR>>,
        filter: Option<DynTree<ExprIR>>,
        write: bool,
    },
    Return {
        distinct: bool,
        exprs: Vec<(Variable, DynTree<ExprIR>)>,
        orderby: Vec<(DynTree<ExprIR>, bool)>,
        skip: Option<DynTree<ExprIR>>,
        limit: Option<DynTree<ExprIR>>,
        write: bool,
    },
    Query(Vec<QueryIR>, bool),
}

#[cfg_attr(tarpaulin, skip)]
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
            Self::Match { pattern, .. } => writeln!(f, "MATCH {pattern}"),
            Self::Unwind(l, v) => {
                writeln!(f, "UNWIND {}:", v.as_str())?;
                write!(f, "{l}")
            }
            Self::Merge(p) => writeln!(f, "MERGE {p}"),
            Self::Create(p) => write!(f, "CREATE {p}"),
            Self::Delete(exprs, _) => {
                writeln!(f, "DELETE:")?;
                for expr in exprs {
                    write!(f, "{expr}")?;
                }
                Ok(())
            }
            Self::Set(items) => {
                writeln!(f, "SET:")?;
                for (target, value, _) in items {
                    write!(f, "{target} = {value}")?;
                }
                Ok(())
            }
            Self::Remove(items) => {
                writeln!(f, "REMOVE:")?;
                for item in items {
                    write!(f, "{item}")?;
                }
                Ok(())
            }
            Self::With { exprs, .. } => {
                writeln!(f, "WITH:")?;
                for (name, _) in exprs {
                    write!(f, "{}", name.as_str())?;
                }
                Ok(())
            }
            Self::Return { exprs, .. } => {
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
        &self,
        mut iter: T,
        env: &mut HashSet<u32>,
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
            Self::Match {
                pattern, filter, ..
            } => {
                for node in pattern.nodes.values() {
                    node.attrs.root().validate(env)?;
                    env.insert(node.alias.id);
                }
                for relationship in pattern.relationships.values() {
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.id);
                }
                for path in pattern.paths.values() {
                    if env.contains(&path.var.id) {
                        return Err(format!("Duplicate alias {}", path.var.as_str()));
                    }
                    env.insert(path.var.id);
                }
                if let Some(filter) = filter {
                    filter.root().validate(env)?;
                }
                iter.next().map_or_else(|| Err(String::from(
                        "Query cannot conclude with MATCH (must be a RETURN clause, an update clause, a procedure call or a non-returning subquery)",
                    )), |first| first.inner_validate(iter, env))
            }
            Self::Unwind(l, v) => {
                l.root().validate(env)?;
                if env.contains(&v.id) {
                    return Err(format!("Duplicate alias {}", v.as_str()));
                }
                env.insert(v.id);
                iter.next().map_or_else(|| Err(String::from(
                        "Query cannot conclude with UNWIND (must be a RETURN clause, an update clause, a procedure call or a non-returning subquery)",
                    )), |first| first.inner_validate(iter, env))
            }
            Self::Merge(p) => {
                for node in p.nodes.values() {
                    if env.contains(&node.alias.id) && p.relationships.is_empty() {
                        return Err(format!(
                            "The bound variable {} can't be redeclared in a create clause",
                            node.alias.as_str()
                        ));
                    }
                    node.attrs.root().validate(env)?;
                }
                for node in p.nodes.values() {
                    env.insert(node.alias.id);
                }
                for relationship in p.relationships.values() {
                    if relationship.types.len() != 1 {
                        return Err(String::from(
                            "Exactly one relationship type must be specified for each relation in a MERGE pattern.",
                        ));
                    }
                    relationship.attrs.root().validate(env)?;
                    env.insert(relationship.alias.id);
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Create(p) => {
                for path in p.paths.values() {
                    if env.contains(&path.var.id) {
                        return Err(format!(
                            "The bound variable {} can't be redeclared in a create clause",
                            path.var.as_str()
                        ));
                    }
                    env.insert(path.var.id);
                }
                for node in p.nodes.values() {
                    if env.contains(&node.alias.id) && p.relationships.is_empty() {
                        return Err(format!(
                            "The bound variable {} can't be redeclared in a create clause",
                            node.alias.as_str()
                        ));
                    }
                    node.attrs.root().validate(env)?;
                }
                for node in p.nodes.values() {
                    env.insert(node.alias.id);
                }
                for relationship in p.relationships.values() {
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
            Self::Set(items) => {
                for (target, value, _) in items {
                    target.root().validate(env)?;
                    value.root().validate(env)?;
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Remove(items) => {
                for item in items {
                    item.root().validate(env)?;
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::With { exprs, .. } | Self::Return { exprs, .. } => {
                for (_, expr) in exprs {
                    expr.root().validate(env)?;
                }
                if !exprs.is_empty() {
                    env.clear();
                    let mut seen_aliases = HashSet::new();
                    for (name, _) in exprs {
                        let alias = name.as_str();
                        if !seen_aliases.insert(alias) {
                            return Err(String::from(
                                "Error: Multiple result columns with the same name are not supported.",
                            ));
                        }
                        env.insert(name.id);
                    }
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter, env))
            }
            Self::Query(q, _) => {
                let mut iter = q.iter();
                let first = iter.next().ok_or("Empty query")?;
                first.inner_validate(iter, env)
            }
        }
    }
}
