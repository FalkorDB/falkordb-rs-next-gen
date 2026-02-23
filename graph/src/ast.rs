//! Abstract Syntax Tree (AST) definitions for Cypher queries.
//!
//! This module defines the intermediate representation (IR) for parsed Cypher queries.
//! The AST is produced by the parser ([`crate::cypher`]) and consumed by the binder
//! ([`crate::binder`]) and planner ([`crate::planner`]).
//!
//! ## Key Types
//!
//! - [`Variable`]: A named or anonymous variable in a query
//! - [`ExprIR`]: Expression nodes (literals, operators, function calls)
//! - [`QueryIR`]: Query clause nodes (MATCH, CREATE, RETURN, etc.)
//! - [`QueryGraph`]: Pattern graph structure with nodes, relationships, and paths
//!
//! ## Type Parameters
//!
//! AST types are generic over `TVar` (variable type) to support different stages:
//! - `Arc<String>`: Raw AST before binding (variables are just names)
//! - [`Variable`]: Bound AST with resolved variable IDs and types
//!
//! ## Expression Trees
//!
//! Expressions are stored as trees using `DynTree<ExprIR<TVar>>` from `orx-tree`.
//! Operators are internal nodes with operands as children, supporting arbitrary
//! expression nesting.

use std::{collections::HashSet, fmt::Display, hash::Hash, sync::Arc};

use itertools::Itertools;
use orx_tree::{Dfs, DynTree, NodeRef};

use crate::{
    indexer::{EntityType, IndexType},
    runtime::{
        functions::{GraphFn, Type},
        orderset::OrderSet,
    },
};

/// A variable in a Cypher query, either named or anonymous.
///
/// Variables are assigned unique IDs during binding to distinguish between
/// variables with the same name in different scopes.
///
/// # Fields
/// - `name`: The variable name as it appears in the query (None for anonymous)
/// - `id`: Unique identifier assigned during binding
/// - `scope_id`: The scope in which this variable was defined
/// - `ty`: The inferred or declared type of the variable
#[derive(Clone, Debug)]
pub struct Variable {
    pub name: Option<Arc<String>>,
    pub id: u32,
    pub scope_id: u32,
    pub ty: Type,
}

impl Display for Variable {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{name}")
        } else {
            write!(f, "?{}", self.id)
        }
    }
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

/// Expression IR nodes for the Cypher expression tree.
///
/// Expressions form a tree structure where operators are internal nodes and
/// their operands are children. For example, `a + b * c` becomes:
///
/// ```text
///       Add
///      /   \
///     a    Mul
///         /   \
///        b     c
/// ```
///
/// # Type Parameter
/// - `TVar`: Variable type (`Arc<String>` before binding, `Variable` after)
#[derive(Clone, Debug)]
pub enum ExprIR<TVar> {
    /// NULL literal
    Null,
    /// Boolean literal (true/false)
    Bool(bool),
    /// Integer literal (i64)
    Integer(i64),
    /// Floating point literal (f64)
    Float(f64),
    /// String literal
    String(Arc<String>),
    /// List constructor - children are list elements
    List,
    /// Map constructor - children are key-value pairs
    Map,
    /// Variable reference
    Variable(TVar),
    /// Query parameter reference ($param)
    Parameter(String),
    /// Length/size of a list or string
    Length,
    /// Element access (list[index] or map.key)
    GetElement,
    /// Slice access (list[start..end])
    GetElements,
    /// Type check: is value a node?
    IsNode,
    /// Type check: is value a relationship?
    IsRelationship,
    /// Logical OR
    Or,
    /// Logical XOR
    Xor,
    /// Logical AND
    And,
    /// Logical NOT
    Not,
    /// Numeric negation
    Negate,
    /// Equality comparison
    Eq,
    /// Inequality comparison
    Neq,
    /// Less than
    Lt,
    /// Greater than
    Gt,
    /// Less than or equal
    Le,
    /// Greater than or equal
    Ge,
    /// IN operator (element in list)
    In,
    /// Addition or string concatenation
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Power/exponentiation
    Pow,
    /// Modulo
    Modulo,
    /// DISTINCT modifier for expressions
    Distinct,
    /// Property access (e.g., n.prop)
    Property(Arc<String>),
    /// Function call with function definition
    FuncInvocation(Arc<GraphFn>),
    /// List quantifier (all/any/none/single)
    Quantifier(QuantifierType, TVar),
    /// List comprehension [x IN list | expr]
    ListComprehension(TVar),
    /// Parenthesized expression (for precedence)
    Paren,
}

#[cfg_attr(tarpaulin, skip)]
impl<TVar: Display> Display for ExprIR<TVar> {
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
            Self::Variable(id) => write!(f, "{id}"),
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
            Self::Property(prop) => write!(f, "property({prop})"),
            Self::FuncInvocation(func) => write!(f, "{}()", func.name),
            Self::Quantifier(quantifier_type, var) => {
                write!(f, "{quantifier_type} {var}")
            }
            Self::ListComprehension(var) => {
                write!(f, "list comp({var})")
            }
            Self::Paren => write!(f, "()"),
        }
    }
}

/// Quantifier types for list predicates (all, any, none, single).
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

/// Trait for checking if an expression contains aggregation functions.
pub trait SupportAggregation {
    /// Returns true if this expression tree contains any aggregation function
    /// (e.g., count, sum, avg, collect).
    fn is_aggregation(&self) -> bool;
}

impl SupportAggregation for DynTree<ExprIR<Variable>> {
    fn is_aggregation(&self) -> bool {
        self.root().indices::<Dfs>().any(|idx| {
            matches!(
                self.node(idx).data(),
                ExprIR::FuncInvocation(func) if func.is_aggregate()
            )
        })
    }
}

/// A node pattern in a MATCH or CREATE clause.
///
/// Represents patterns like `(n:Person {name: 'Alice'})` where:
/// - `alias` is the variable `n`
/// - `labels` contains `Person`
/// - `attrs` contains the property filter expression
#[derive(Debug)]
pub struct QueryNode<L, TVar> {
    pub alias: TVar,
    pub labels: OrderSet<L>,
    pub attrs: QueryExpr<TVar>,
}

#[cfg_attr(tarpaulin, skip)]
impl<L: Display + PartialEq, TVar: Display + PartialEq> Display for QueryNode<L, TVar> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.labels.is_empty() {
            return write!(f, "({})", self.alias);
        }
        write!(f, "({}:{})", self.alias, self.labels.iter().join(":"))
    }
}

impl<L, TVar> QueryNode<L, TVar> {
    #[must_use]
    pub const fn new(
        alias: TVar,
        labels: OrderSet<L>,
        attrs: QueryExpr<TVar>,
    ) -> Self {
        Self {
            alias,
            labels,
            attrs,
        }
    }
}

/// A relationship pattern in a MATCH or CREATE clause.
///
/// Represents patterns like `(a)-[r:KNOWS]->(b)` where:
/// - `alias` is the variable `r`
/// - `types` contains `KNOWS` (can have multiple for OR: `[:A|B]`)
/// - `from` and `to` are the connected nodes
/// - `bidirectional` is true for undirected patterns `-[]-`
#[derive(Debug)]
pub struct QueryRelationship<T, L, TVar> {
    pub alias: TVar,
    pub types: Vec<T>,
    pub attrs: QueryExpr<TVar>,
    pub from: Arc<QueryNode<L, TVar>>,
    pub to: Arc<QueryNode<L, TVar>>,
    pub bidirectional: bool,
}

#[cfg_attr(tarpaulin, skip)]
impl<T: Display, L: Display, TVar: Display> Display for QueryRelationship<T, L, TVar> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let direction = if self.bidirectional { "" } else { ">" };
        if self.types.is_empty() {
            return write!(
                f,
                "({})-[{}]-{}({})",
                self.from.alias, self.alias, direction, self.to.alias
            );
        }
        write!(
            f,
            "({})-[{}:{}]-{}({})",
            self.from.alias,
            self.alias,
            self.types.iter().join("|"),
            direction,
            self.to.alias
        )
    }
}

impl<T, L, TVar> QueryRelationship<T, L, TVar> {
    #[must_use]
    pub const fn new(
        alias: TVar,
        types: Vec<T>,
        attrs: QueryExpr<TVar>,
        from: Arc<QueryNode<L, TVar>>,
        to: Arc<QueryNode<L, TVar>>,
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

/// A named path pattern in a MATCH clause.
///
/// Represents patterns like `p = (a)-[*]->(b)` where:
/// - `var` is the path variable `p`
/// - `vars` contains all variables in the path pattern
#[derive(Debug)]
pub struct QueryPath<TVar> {
    pub var: TVar,
    pub vars: Vec<TVar>,
}

impl<TVar> QueryPath<TVar> {
    #[must_use]
    pub const fn new(
        var: TVar,
        vars: Vec<TVar>,
    ) -> Self {
        Self { var, vars }
    }
}

/// A graph pattern containing nodes, relationships, and paths.
///
/// This represents the pattern portion of MATCH, CREATE, and MERGE clauses.
/// The graph can be decomposed into connected components for query optimization.
///
/// Uses `Arc` for sharing patterns between different parts of the query plan,
/// avoiding expensive cloning of complex patterns.
#[derive(Clone, Debug)]
pub struct QueryGraph<T, L, TVar> {
    nodes: Vec<Arc<QueryNode<L, TVar>>>,
    relationships: Vec<Arc<QueryRelationship<T, L, TVar>>>,
    paths: Vec<Arc<QueryPath<TVar>>>,
}

impl<T, L, TVar> Default for QueryGraph<T, L, TVar> {
    fn default() -> Self {
        Self {
            nodes: Vec::default(),
            relationships: Vec::default(),
            paths: Vec::default(),
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl<T: Display + PartialEq, L: Display + PartialEq, TVar: Display + PartialEq + Eq + Hash> Display
    for QueryGraph<T, L, TVar>
{
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
            write!(f, "{}, ", path.var)?;
        }
        Ok(())
    }
}

impl<T, L, TVar: Clone + Hash + Eq> QueryGraph<T, L, TVar> {
    pub fn add_node(
        &mut self,
        node: Arc<QueryNode<L, TVar>>,
    ) -> bool {
        if self.nodes.iter().any(|n| n.alias == node.alias) {
            return false;
        }
        self.nodes.push(node);
        true
    }

    pub fn add_relationship(
        &mut self,
        relationship: Arc<QueryRelationship<T, L, TVar>>,
    ) -> bool {
        if self
            .relationships
            .iter()
            .any(|r| r.alias == relationship.alias)
        {
            false
        } else {
            self.relationships.push(relationship);
            true
        }
    }

    pub fn add_path(
        &mut self,
        path: Arc<QueryPath<TVar>>,
    ) -> bool {
        if self.paths.iter().any(|p| p.var == path.var) {
            false
        } else {
            self.paths.push(path);
            true
        }
    }

    #[must_use]
    pub fn variables(&self) -> Vec<TVar> {
        self.nodes
            .iter()
            .map(|n| n.alias.clone())
            .chain(self.relationships.iter().map(|r| r.alias.clone()))
            .chain(self.paths.iter().map(|p| p.var.clone()))
            .collect()
    }

    #[must_use]
    pub fn nodes(&self) -> Vec<Arc<QueryNode<L, TVar>>> {
        self.nodes.clone()
    }

    #[must_use]
    pub fn relationships(&self) -> Vec<Arc<QueryRelationship<T, L, TVar>>> {
        self.relationships.clone()
    }

    #[must_use]
    pub fn paths(&self) -> Vec<Arc<QueryPath<TVar>>> {
        self.paths.clone()
    }
}

impl<T, L> QueryGraph<T, L, Variable> {
    #[must_use]
    pub fn filter_visited(
        &self,
        visited: &HashSet<u32>,
    ) -> Self
    where
        T: Default,
        L: Default,
    {
        let mut res = Self::default();
        for node in &self.nodes {
            if !visited.contains(&node.alias.id) {
                res.add_node(node.clone());
            }
        }
        for relationship in &self.relationships {
            if !visited.contains(&relationship.alias.id) {
                res.add_relationship(relationship.clone());
            }
        }
        for path in &self.paths {
            if !visited.contains(&path.var.id) {
                res.add_path(path.clone());
            }
        }
        res
    }

    #[must_use]
    pub fn connected_components(&self) -> Vec<Self>
    where
        T: Default,
        L: Default,
    {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in &self.nodes {
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
        node: &Arc<QueryNode<L, Variable>>,
        visited: &mut HashSet<u32>,
        component: &mut Self,
    ) {
        visited.insert(node.alias.id);
        component.add_node(node.clone());

        for relationship in &self.relationships {
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

        for path in &self.paths {
            if path.vars.iter().all(|id| visited.contains(&id.id)) && visited.insert(path.var.id) {
                component.add_path(path.clone());
            }
        }
    }
}

/// Type alias for expression trees.
pub type QueryExpr<TVar> = Arc<DynTree<ExprIR<TVar>>>;

/// An item in a SET clause - either property assignment or label modification.
#[derive(Clone, Debug)]
pub enum SetItem<L, TVar> {
    /// Property assignment: `n.prop = value` (replace=true) or `n += {props}` (replace=false)
    Attribute(QueryExpr<TVar>, QueryExpr<TVar>, bool),
    /// Label assignment: `SET n:Label`
    Label(TVar, OrderSet<L>),
}

#[cfg_attr(tarpaulin, skip)]
impl<L: Display + PartialEq, TVar: Display> Display for SetItem<L, TVar> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Attribute(target, value, replace) => {
                let op = if *replace { "=" } else { "+=" };
                write!(f, "{target} {op} {value}")
            }
            Self::Label(var, labels) => {
                write!(f, "{var}:")?;
                let mut first = true;
                for i in 0..labels.len() {
                    if !first {
                        write!(f, ":")?;
                    }
                    first = false;
                    write!(f, "{}", &labels[i])?;
                }
                Ok(())
            }
        }
    }
}

/// Query clause IR - represents each clause type in a Cypher query.
///
/// A complete query is a sequence of these clauses. The planner converts
/// this AST into an execution plan.
#[derive(Debug)]
pub enum QueryIR<TVar> {
    /// CALL procedure(args) YIELD outputs WHERE filter
    Call(
        Arc<GraphFn>,
        Vec<QueryExpr<TVar>>,
        Vec<TVar>,
        Option<QueryExpr<TVar>>,
    ),
    /// MATCH pattern WHERE filter (optional flag for OPTIONAL MATCH)
    Match {
        pattern: QueryGraph<Arc<String>, Arc<String>, TVar>,
        filter: Option<QueryExpr<TVar>>,
        optional: bool,
    },
    /// UNWIND list AS var
    Unwind(QueryExpr<TVar>, TVar),
    /// MERGE pattern ON CREATE SET ... ON MATCH SET ...
    Merge(
        QueryGraph<Arc<String>, Arc<String>, TVar>,
        Vec<SetItem<Arc<String>, TVar>>,
        Vec<SetItem<Arc<String>, TVar>>,
    ),
    /// CREATE pattern
    Create(QueryGraph<Arc<String>, Arc<String>, TVar>),
    /// DELETE exprs (detach flag for DETACH DELETE)
    Delete(Vec<QueryExpr<TVar>>, bool),
    /// SET items
    Set(Vec<SetItem<Arc<String>, TVar>>),
    /// REMOVE items (properties or labels)
    Remove(Vec<QueryExpr<TVar>>),
    /// LOAD CSV FROM path AS var
    LoadCsv {
        file_path: QueryExpr<TVar>,
        headers: bool,
        delimiter: QueryExpr<TVar>,
        var: TVar,
    },
    /// WITH clause for intermediate projections and aggregations
    With {
        distinct: bool,
        all: bool,
        exprs: Vec<(TVar, QueryExpr<TVar>)>,
        copy_from_parent: Vec<(Variable, Variable)>,
        orderby: Vec<(QueryExpr<TVar>, bool)>,
        skip: Option<QueryExpr<TVar>>,
        limit: Option<QueryExpr<TVar>>,
        filter: Option<QueryExpr<TVar>>,
        write: bool,
    },
    Return {
        distinct: bool,
        all: bool,
        exprs: Vec<(TVar, QueryExpr<TVar>)>,
        copy_from_parent: Vec<(Variable, Variable)>,
        orderby: Vec<(QueryExpr<TVar>, bool)>,
        skip: Option<QueryExpr<TVar>>,
        limit: Option<QueryExpr<TVar>>,
        write: bool,
    },
    CreateIndex {
        label: Arc<String>,
        attrs: Vec<Arc<String>>,
        index_type: IndexType,
        entity_type: EntityType,
        options: Option<QueryExpr<TVar>>,
    },
    DropIndex {
        label: Arc<String>,
        attrs: Vec<Arc<String>>,
        index_type: IndexType,
        entity_type: EntityType,
    },
    Query(Vec<Self>, bool),
}

#[cfg_attr(tarpaulin, skip)]
impl<TVar: Display + Eq + Hash> Display for QueryIR<TVar> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Call(func, args, _, _) => {
                writeln!(f, "{}():", func.name)?;
                for arg in args {
                    write!(f, "{arg}")?;
                }
                Ok(())
            }
            Self::Match { pattern, .. } => writeln!(f, "MATCH {pattern}"),
            Self::Unwind(l, v) => {
                writeln!(f, "UNWIND {v}:")?;
                write!(f, "{l}")
            }
            Self::Merge(p, _, _) => writeln!(f, "MERGE {p}"),
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
                for item in items {
                    write!(f, "{item}")?;
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
            Self::LoadCsv { file_path, var, .. } => {
                writeln!(f, "LOAD CSV FROM {file_path} AS {var:}:")
            }
            Self::With { exprs, .. } => {
                writeln!(f, "WITH:")?;
                for (name, _) in exprs {
                    write!(f, "{name}")?;
                }
                Ok(())
            }
            Self::Return { exprs, .. } => {
                writeln!(f, "RETURN:")?;
                for (name, _) in exprs {
                    write!(f, "{name}")?;
                }
                Ok(())
            }
            Self::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options: _options,
            } => {
                writeln!(
                    f,
                    "CREATE {index_type:?} {entity_type:?} INDEX ON :{label}({attrs:?})"
                )
            }
            Self::DropIndex {
                label,
                attrs,
                index_type,
                entity_type,
            } => {
                writeln!(
                    f,
                    "DROP {index_type:?} {entity_type:?} INDEX ON :{label}({attrs:?})"
                )
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

impl<TVar: Eq + Hash> QueryIR<TVar> {
    pub fn validate(&self) -> Result<(), String> {
        self.inner_validate(std::iter::empty())
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cognitive_complexity)]
    fn inner_validate<'a, T>(
        &self,
        mut iter: T,
    ) -> Result<(), String>
    where
        T: Iterator<Item = &'a Self>,
        TVar: 'a,
    {
        match self {
            Self::Call(proc, args, _, _) => {
                if proc.name == "db.idx.fulltext.createNodeIndex" {
                    match args[0].root().data() {
                        ExprIR::String(_) => {}
                        ExprIR::Map => {
                            let mut has_labels = false;
                            for child in args[0].root().children() {
                                if let ExprIR::String(label) = child.data()
                                    && label.as_str() == "label"
                                {
                                    has_labels = true;
                                    break;
                                }
                            }
                            if !has_labels {
                                return Err(String::from("Label is missing"));
                            }
                        }
                        _ => {
                            return Err(String::from(
                                "The first argument of a procedure call must be a string or a map with a 'label' key",
                            ));
                        }
                    }
                }
                Ok(())
            }
            Self::Match { pattern, .. } => {
                Self::validate_inlined_properties(pattern)?;
                iter.next().map_or_else(|| Err(String::from(
                        "Query cannot conclude with MATCH (must be a RETURN clause, an update clause, a procedure call or a non-returning subquery)",
                    )), |first| first.inner_validate(iter))
            }
            Self::Unwind(_, _) => {
                iter.next().map_or_else(|| Err(String::from(
                        "Query cannot conclude with UNWIND (must be a RETURN clause, an update clause, a procedure call or a non-returning subquery)",
                    )), |first| first.inner_validate(iter))
            }
            Self::Merge(p, on_create_set_items, on_match_set_items) => {
                Self::validate_inlined_properties(p)?;
                for relationship in &p.relationships {
                    if relationship.types.len() != 1 {
                        return Err(String::from(
                            "Exactly one relationship type must be specified for each relation in a MERGE pattern.",
                        ));
                    }
                }
                Self::validate_set_items(on_create_set_items)?;
                Self::validate_set_items(on_match_set_items)?;
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::Create(p) => {
                Self::validate_inlined_properties(p)?;
                for relationship in &p.relationships {
                    if relationship.types.len() != 1 {
                        return Err(String::from(
                            "Exactly one relationship type must be specified for each relation in a CREATE pattern.",
                        ));
                    }
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::Delete(_exprs, _) => {
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::Set(items) => {
                Self::validate_set_items(items)?;
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::Remove(items) => {
                for item in items {
                    if  matches!(item.root().data(), ExprIR::Property(_)) && matches!(item.root().child(0).data(), ExprIR::Null) {
                        return Err("Type mismatch: expected Node or Relationship but was Null".to_string());
                    }
                }
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::LoadCsv { .. } => {
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::With { exprs: _, orderby: _orderby, .. } | Self::Return { exprs: _, orderby: _orderby, .. } => {
                iter.next()
                    .map_or(Ok(()), |first| first.inner_validate(iter))
            }
            Self::CreateIndex { .. } => iter
                .next()
                .map_or(Ok(()), |first| first.inner_validate(iter)),
            Self::DropIndex { .. } => iter
                .next()
                .map_or(Ok(()), |first| first.inner_validate(iter)),
            Self::Query(q, _) => {
                let mut iter = q.iter();
                let first = iter.next().ok_or("Error: empty query.")?;
                first.inner_validate(iter)
            }
        }
    }

    fn validate_set_items(items: &Vec<SetItem<Arc<String>, TVar>>) -> Result<(), String> {
        for item in items {
            if let SetItem::Attribute(target, _, _) = item {
                if let ExprIR::Property(_) = target.root().data()
                    && let ExprIR::Variable(_) = target.root().child(0).data()
                {
                } else if let ExprIR::Variable(_) = target.root().data() {
                } else {
                    return Err(String::from(
                        "FalkorDB does not currently support non-alias references on the left-hand side of SET expressions",
                    ));
                }
            }
        }
        Ok(())
    }

    /// Validates that inlined properties in node/relationship patterns are maps.
    /// Mirrors `_ValidateInlinedProperties` in the C `ast_validations.c`.
    fn validate_inlined_properties(
        p: &QueryGraph<Arc<String>, Arc<String>, TVar>
    ) -> Result<(), String> {
        for node in &p.nodes {
            if !matches!(node.attrs.root().data(), ExprIR::Map) {
                return Err(String::from(
                    "Encountered unhandled type in inlined properties.",
                ));
            }
        }
        for rel in &p.relationships {
            if !matches!(rel.attrs.root().data(), ExprIR::Map) {
                return Err(String::from(
                    "Encountered unhandled type in inlined properties.",
                ));
            }
        }
        Ok(())
    }
}

/// Type alias for unbound query IR (variables are just string names).
pub type RawQueryIR = QueryIR<Arc<String>>;

/// Type alias for bound query IR (variables have resolved IDs and types).
pub type BoundQueryIR = QueryIR<Variable>;
