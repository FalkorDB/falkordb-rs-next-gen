//! Semantic analysis and name resolution for Cypher queries.
//!
//! The binder performs the second phase of query processing, converting a raw
//! AST (with string variable names) into a bound AST (with resolved variable
//! IDs and types). This phase:
//!
//! 1. **Resolves variable references** - Links variable uses to their definitions
//! 2. **Manages scopes** - Handles variable visibility across clauses
//! 3. **Infers types** - Determines types for expressions and variables
//! 4. **Validates semantics** - Catches errors like undefined variables
//!
//! ## Scope Rules
//!
//! Cypher has specific scoping rules:
//! - Variables defined in MATCH/CREATE are visible in subsequent clauses
//! - WITH/RETURN create new scopes, only explicitly projected variables carry over
//! - CALL procedures can YIELD variables into scope
//!
//! ## Example
//!
//! ```text
//! MATCH (n:Person)     // Defines variable 'n' with id=0
//! WHERE n.age > 18     // References variable with id=0
//! WITH n, n.name AS name  // Creates new scope, projects n (id=1), defines name (id=2)
//! RETURN name          // References variable with id=2
//! ```

use crate::ast::{
    BoundQueryIR, ExprIR, QueryExpr, QueryGraph, QueryIR, QueryNode, QueryPath, QueryRelationship,
    RawQueryIR, SetItem, Variable,
};
use crate::runtime::functions::Type;
use crate::tree;
use orx_tree::{DynTree, NodeRef};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// The binder performs semantic analysis on parsed Cypher queries.
///
/// It resolves variable references, manages scope, and converts the raw AST
/// (with string names) into a bound AST (with numeric variable IDs and types).
pub struct Binder {
    /// Counter for generating unique variable IDs
    next_var_id: u32,
    /// Stack of variable environments (name → Variable mapping)
    env_stack: Vec<HashMap<Arc<String>, Variable>>,
    /// Whether to look up variables in parent scope
    use_parent_scope: bool,
    /// Variables copied from parent scope to current scope
    parent_to_child_scope: HashMap<Arc<String>, Variable>,
    /// Track which variables need to be copied from parent
    copy_from_parent: HashMap<Arc<String>, (Variable, Variable)>,
}

impl Default for Binder {
    fn default() -> Self {
        Self {
            next_var_id: 0,
            env_stack: vec![HashMap::new()],
            use_parent_scope: false,
            parent_to_child_scope: HashMap::new(),
            copy_from_parent: HashMap::new(),
        }
    }
}

/// The type of projection clause (affects scope handling).
#[derive(Clone, Copy)]
enum ProjectionKind {
    With,
    Return,
}

impl Binder {
    /// Binds a raw query IR, resolving all variable references.
    ///
    /// This is the main entry point for semantic analysis.
    pub fn bind(
        mut self,
        ir: RawQueryIR,
    ) -> Result<BoundQueryIR, String> {
        self.bind_ir(ir)
    }

    fn current_env(&self) -> &HashMap<Arc<String>, Variable> {
        self.env_stack
            .last()
            .expect("env_stack should never be empty")
    }

    fn current_env_mut(&mut self) -> &mut HashMap<Arc<String>, Variable> {
        self.env_stack
            .last_mut()
            .expect("env_stack should never be empty")
    }

    fn push_scope(&mut self) {
        self.env_stack.push(HashMap::new());
        self.use_parent_scope = true;
        self.next_var_id = 0;
    }

    fn commit_scope(&mut self) {
        self.use_parent_scope = false;
        self.parent_to_child_scope.clear();
    }

    fn bind_ir(
        &mut self,
        ir: RawQueryIR,
    ) -> Result<BoundQueryIR, String> {
        match ir {
            QueryIR::Query(clauses, write) => {
                let mut bound = Vec::with_capacity(clauses.len());
                for clause in clauses {
                    bound.push(self.bind_ir(clause)?);
                }
                Ok(QueryIR::Query(bound, write))
            }
            QueryIR::Match {
                pattern,
                filter,
                optional,
            } => {
                let pattern = self.bind_graph(pattern, false)?;
                let filter = filter.map(|expr| self.bind_expr(&expr)).transpose()?;
                if let Some(ref f) = filter {
                    if !Self::expr_may_return_boolean(f.root()) {
                        return Err(String::from("Expected boolean predicate"));
                    }
                }
                Ok(QueryIR::Match {
                    pattern,
                    filter,
                    optional,
                })
            }
            QueryIR::Unwind(expr, var_name) => {
                let expr = self.bind_expr(&expr)?;
                let var = self.define_name_in_scope(var_name, Type::Any, false)?;
                Ok(QueryIR::Unwind(expr, var))
            }
            QueryIR::Merge(pattern, on_create, on_match) => {
                let pattern = self.bind_graph(pattern, false)?;
                let on_create = self.bind_set_items(on_create)?;
                let on_match = self.bind_set_items(on_match)?;
                Ok(QueryIR::Merge(pattern, on_create, on_match))
            }
            QueryIR::Create(pattern) => {
                let bound = self.bind_graph_create(pattern)?;
                Ok(QueryIR::Create(bound))
            }
            QueryIR::CreateIndex {
                label,
                attrs,
                index_type,
                entity_type,
                options,
            } => {
                let options = options.map(|expr| self.bind_expr(&expr)).transpose()?;
                Ok(QueryIR::CreateIndex {
                    label,
                    attrs,
                    index_type,
                    entity_type,
                    options,
                })
            }
            QueryIR::DropIndex {
                label,
                attrs,
                index_type,
                entity_type,
            } => Ok(QueryIR::DropIndex {
                label,
                attrs,
                index_type,
                entity_type,
            }),
            QueryIR::Delete(exprs, detach) => {
                let exprs = exprs
                    .iter()
                    .map(|expr| self.bind_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(QueryIR::Delete(exprs, detach))
            }
            QueryIR::Set(items) => Ok(QueryIR::Set(self.bind_set_items(items)?)),
            QueryIR::Remove(items) => {
                let items = items
                    .iter()
                    .map(|expr| self.bind_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(QueryIR::Remove(items))
            }
            QueryIR::LoadCsv {
                file_path,
                headers,
                delimiter,
                var,
            } => {
                let file_path = self.bind_expr(&file_path)?;
                let delimiter = self.bind_expr(&delimiter)?;
                let var = self.define_name_in_scope(var, Type::Any, true)?;
                Ok(QueryIR::LoadCsv {
                    file_path,
                    headers,
                    delimiter,
                    var,
                })
            }
            QueryIR::With {
                distinct,
                all,
                exprs,
                copy_from_parent: _,
                orderby,
                skip,
                limit,
                filter,
                write,
            } => self.bind_projection(
                ProjectionKind::With,
                distinct,
                all,
                exprs,
                orderby,
                skip,
                limit,
                filter,
                write,
            ),
            QueryIR::Return {
                distinct,
                all,
                exprs,
                copy_from_parent: _,
                orderby,
                skip,
                limit,
                write,
            } => self.bind_projection(
                ProjectionKind::Return,
                distinct,
                all,
                exprs,
                orderby,
                skip,
                limit,
                None,
                write,
            ),
            QueryIR::Call(func, args, vars, filter) => {
                let args = args
                    .iter()
                    .map(|expr| self.bind_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;
                let mut bound_vars = Vec::with_capacity(vars.len());
                for name in vars {
                    bound_vars.push(self.define_name_in_scope(name, Type::Any, true)?);
                }
                let filter = filter.map(|expr| self.bind_expr(&expr)).transpose()?;
                if let Some(ref f) = filter {
                    if !Self::expr_may_return_boolean(f.root()) {
                        return Err(String::from("Expected boolean predicate"));
                    }
                }
                Ok(QueryIR::Call(func, args, bound_vars, filter))
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn bind_projection(
        &mut self,
        kind: ProjectionKind,
        distinct: bool,
        all: bool,
        exprs: Vec<(Arc<String>, QueryExpr<Arc<String>>)>,
        orderby: Vec<(QueryExpr<Arc<String>>, bool)>,
        skip: Option<QueryExpr<Arc<String>>>,
        limit: Option<QueryExpr<Arc<String>>>,
        filter: Option<QueryExpr<Arc<String>>>,
        write: bool,
    ) -> Result<QueryIR<Variable>, String> {
        let bound_exprs = exprs
            .iter()
            .map(|(name, expr)| {
                let bound = self.bind_expr(expr)?;
                Self::validate_boolean_operands(&bound)?;
                Ok((name.clone(), bound))
            })
            .collect::<Result<Vec<_>, String>>()?;

        let mut projected = Vec::with_capacity(bound_exprs.len());

        // If `all` is true, project all variables from the current env
        if all {
            let env_copy = self.current_env().clone(); // Clone to avoid borrowing issues
            self.push_scope();
            for (name, var) in &env_copy {
                let bound_var = self.project_name(name, var.ty.clone());
                // Create an expression that refers to the variable
                let expr = Arc::new(DynTree::new(ExprIR::Variable(var.clone())));
                projected.push((bound_var, expr));
            }
            projected.sort_by(|(name_a, _), (name_b, _)| name_a.name.cmp(&name_b.name));
        } else {
            // Project explicitly listed expressions
            self.push_scope();
            let mut seen_aliases = HashSet::new();
            for (name, expr) in bound_exprs {
                if !seen_aliases.insert(name.clone()) {
                    return Err(String::from(
                        "Error: Multiple result columns with the same name are not supported.",
                    ));
                }
                let bound_var = self.project_name(&name, Type::Any);
                if let ExprIR::Variable(var_name) = expr.root().data() {
                    self.parent_to_child_scope
                        .insert(var_name.name.as_ref().unwrap().clone(), bound_var.clone());
                }
                projected.push((bound_var, expr));
            }
        }

        let orderby = orderby
            .iter()
            .map(|(expr, desc)| Ok((self.bind_expr(expr)?, *desc)))
            .collect::<Result<Vec<_>, String>>()?;
        let skip = skip.map(|expr| self.bind_expr(&expr)).transpose()?;
        let limit = limit.map(|expr| self.bind_expr(&expr)).transpose()?;
        let filter = filter.map(|expr| self.bind_expr(&expr)).transpose()?;
        if let Some(ref f) = filter {
            if !Self::expr_may_return_boolean(f.root()) {
                return Err(String::from("Expected boolean predicate"));
            }
        }

        let copy_from_parent = self
            .copy_from_parent
            .values()
            .cloned()
            .collect::<Vec<(Variable, Variable)>>();
        self.copy_from_parent.clear();
        self.commit_scope();

        Ok(match kind {
            ProjectionKind::With => QueryIR::With {
                distinct,
                all,
                exprs: projected,
                copy_from_parent,
                orderby,
                skip,
                limit,
                filter,
                write,
            },
            ProjectionKind::Return => QueryIR::Return {
                distinct,
                all,
                exprs: projected,
                copy_from_parent,
                orderby,
                skip,
                limit,
                write,
            },
        })
    }

    fn bind_graph(
        &mut self,
        graph: QueryGraph<Arc<String>, Arc<String>, Arc<String>>,
        is_create: bool,
    ) -> Result<QueryGraph<Arc<String>, Arc<String>, Variable>, String> {
        let mut bound: QueryGraph<Arc<String>, Arc<String>, Variable> = QueryGraph::default();

        // Pre-register path variables in scope so they can be resolved
        // when binding node/relationship inline properties below.
        for raw_path in graph.paths() {
            self.define_name_in_scope(raw_path.var.clone(), Type::Path, true)?;
        }

        for node in graph.nodes() {
            self.bind_expr(&node.attrs)?;
        }
        for relationship in graph.relationships() {
            self.bind_expr(&relationship.attrs)?;
        }

        // Bind all nodes in the graph, merging duplicates by alias
        for node in graph.nodes() {
            let alias = self.define_name_in_scope(node.alias.clone(), Type::Node, !is_create)?;
            let attrs = self.bind_expr(&node.attrs)?;

            let bound_node = Arc::new(QueryNode::new(alias.clone(), node.labels.clone(), attrs));
            bound.add_node(bound_node);
        }

        // Bind relationships, binding any referenced nodes that weren't in the graph
        for relationship in graph.relationships() {
            let alias = self.define_name_in_scope(
                relationship.alias.clone(),
                Type::Relationship,
                !is_create,
            )?;
            let attrs = self.bind_expr(&relationship.attrs)?;

            // Resolve 'from' node by alias, binding if missing and merging labels if present
            let from_bound = if let Some(bound_node) = bound
                .nodes()
                .iter()
                .find(|n| n.alias.name.as_ref().unwrap().clone() == relationship.from.alias)
            {
                bound_node.clone()
            } else {
                let from_alias = self.define_name_in_scope(
                    relationship.from.alias.clone(),
                    Type::Node,
                    !is_create,
                )?;
                let from_attrs = self.bind_expr(&relationship.from.attrs)?;
                let bound_node = Arc::new(QueryNode::new(
                    from_alias.clone(),
                    relationship.from.labels.clone(),
                    from_attrs,
                ));
                bound.add_node(bound_node.clone());
                bound_node
            };

            // Resolve 'to' node by alias, binding if missing and merging labels if present
            let to_bound = if let Some(bound_node) = bound
                .nodes()
                .iter()
                .find(|n| n.alias.name.as_ref().unwrap().clone() == relationship.to.alias)
            {
                bound_node.clone()
            } else {
                let to_alias = self.define_name_in_scope(
                    relationship.to.alias.clone(),
                    Type::Node,
                    !is_create,
                )?;
                let to_attrs = self.bind_expr(&relationship.to.attrs)?;
                let bound_node = Arc::new(QueryNode::new(
                    to_alias.clone(),
                    relationship.to.labels.clone(),
                    to_attrs,
                ));
                bound.add_node(bound_node.clone());
                bound_node
            };

            // Add to bound graph if not already there
            // Nodes already added via alias_to_bound; no duplicate insert needed

            let rel = Arc::new(QueryRelationship::new(
                alias.clone(),
                relationship.types.clone(),
                attrs,
                from_bound,
                to_bound,
                relationship.bidirectional,
            ));
            bound.add_relationship(rel);
        }

        // Bind paths - path vars reference entities by name
        // For anonymous entities with multiple instances, we use alias_to_vars
        for raw_path in graph.paths() {
            let alias = self.define_name_in_scope(raw_path.var.clone(), Type::Path, true)?;

            let mut vars = Vec::with_capacity(raw_path.vars.len());

            for name in &raw_path.vars {
                // Try environment first (for named entities)
                if let Some(var) = self.current_env().get(name) {
                    vars.push(var.clone());
                } else {
                    vars.push(self.resolve_name(name, &[])?);
                }
            }
            bound.add_path(Arc::new(QueryPath::new(alias, vars)));
        }

        Ok(bound)
    }

    fn bind_graph_create(
        &mut self,
        pattern: QueryGraph<Arc<String>, Arc<String>, Arc<String>>,
    ) -> Result<QueryGraph<Arc<String>, Arc<String>, Variable>, String> {
        // For CREATE clause validation: we need to detect when a variable is declared
        // multiple times OR when it shadows a previous declaration.
        //
        // Key insight: The parser includes ALL nodes in pattern.nodes(), including those
        // that only appear as endpoints of relationships. We need to distinguish:
        // - Bare node patterns: (a) - should be validated for redeclaration
        // - Endpoint-only nodes: x in (x)-[:R]->(y) - should NOT be validated (it's a reference)
        //
        // Strategy: A node should only be validated if it's NOT an endpoint of any relationship.
        // If a node ONLY appears as relationship endpoints, it's being referenced, not declared.

        // Collect all node aliases that are endpoints of relationships
        let mut endpoint_aliases = HashSet::new();
        for rel in pattern.relationships() {
            endpoint_aliases.insert(rel.from.alias.clone());
            endpoint_aliases.insert(rel.to.alias.clone());
        }

        // Track which names we've already processed in this CREATE clause
        let mut defined_in_create = HashSet::new();

        // Validate nodes - only check nodes that are NOT purely endpoints of relationships
        for node in pattern.nodes() {
            if node.alias.starts_with("_anon") {
                continue; // Skip anonymous nodes
            }

            // If this node is ONLY an endpoint (not a bare pattern), skip validation
            // A node is "only an endpoint" if:
            // 1. It has no labels and no properties (standard bare node)
            // 2. It appears in endpoint_aliases
            // Actually, we should check: is there a bare pattern for this node?
            // The parser adds nodes when they're first encountered, either as:
            // - Bare pattern: (a)
            // - Relationship endpoint: (a) in (a)-[:R]->()
            //
            // We can't distinguish these just from the node. But we can use the fact that
            // if a node appears in a relationship, the relationship reference is likely
            // what added it, not a bare pattern.
            //
            // Actually, let me reconsider: the parser is called with the pattern as it appears
            // in the CREATE clause. If the user wrote (x)-[:R]->(y), then x and y in endpoints
            // are the FIRST and ONLY occurrence of x and y in the parsed pattern.
            // They should NOT be validated against env because they're references to matched vars.
            //
            // So the rule is: if a node is an endpoint of ANY relationship, treat it as a reference
            // and don't validate it against env.
            if endpoint_aliases.contains(&node.alias) {
                // This node is an endpoint - it's a reference, not a new declaration
                defined_in_create.insert(node.alias.clone());
                continue;
            }

            // This is a bare node pattern (not an endpoint)
            // Check if this is the first occurrence in CREATE of this name
            if !defined_in_create.contains(&node.alias) {
                // Check if it shadows an existing variable from a previous clause
                if self.current_env().contains_key(&node.alias) {
                    return Err(format!(
                        "The bound variable '{}' can't be redeclared in a CREATE clause",
                        node.alias
                    ));
                }
                // Mark it as defined in this clause
                defined_in_create.insert(node.alias.clone());
            }
        }

        // Check relationships for redeclaration
        for rel in pattern.relationships() {
            if rel.alias.starts_with("_anon") {
                continue; // Skip anonymous relationships
            }

            // If this is the first occurrence in CREATE of this name
            if !defined_in_create.contains(&rel.alias) {
                // Check if it shadows an existing variable from a previous clause
                if self.current_env().contains_key(&rel.alias) {
                    return Err(format!(
                        "The bound variable '{}' can't be redeclared in a CREATE clause",
                        rel.alias
                    ));
                }
                // Mark it as defined in this clause
                defined_in_create.insert(rel.alias.clone());
            }
            // If it's a subsequent occurrence, it's allowed (it's a reference)
        }

        // Bind the pattern - allow reuse since we've already validated
        self.bind_graph(pattern, false)
    }

    fn bind_set_items(
        &mut self,
        items: Vec<SetItem<Arc<String>, Arc<String>>>,
    ) -> Result<Vec<SetItem<Arc<String>, Variable>>, String> {
        let mut res = Vec::with_capacity(items.len());
        for item in items {
            match item {
                SetItem::Attribute(target, value, strict) => res.push(SetItem::Attribute(
                    self.bind_expr(&target)?,
                    self.bind_expr(&value)?,
                    strict,
                )),
                SetItem::Label(name, labels) => {
                    let var = self.resolve_name(&name, &[])?;
                    res.push(SetItem::Label(var, labels));
                }
            }
        }
        Ok(res)
    }

    fn bind_expr(
        &mut self,
        expr: &QueryExpr<Arc<String>>,
    ) -> Result<QueryExpr<Variable>, String> {
        self.bind_expr_with_locals(expr, &mut vec![])
    }

    fn bind_expr_with_locals(
        &mut self,
        expr: &QueryExpr<Arc<String>>,
        locals: &mut Vec<HashMap<Arc<String>, Variable>>,
    ) -> Result<QueryExpr<Variable>, String> {
        let root = expr.root();
        Ok(Arc::new(self.bind_expr_node(expr, root, locals)?))
    }

    #[allow(
        clippy::only_used_in_recursion,
        clippy::too_many_lines,
        clippy::needless_pass_by_value
    )]
    fn bind_expr_node(
        &mut self,
        expr: &DynTree<ExprIR<Arc<String>>>,
        node_ref: orx_tree::Node<orx_tree::Dyn<ExprIR<Arc<String>>>>,
        locals: &mut Vec<HashMap<Arc<String>, Variable>>,
    ) -> Result<DynTree<ExprIR<Variable>>, String> {
        match node_ref.data() {
            ExprIR::Variable(name) => {
                let var = self.resolve_name(name, locals)?;
                Ok(tree!(ExprIR::Variable(var)))
            }
            ExprIR::Quantifier(qt, name) => {
                let bound_var: Variable = self.fresh_var(Some(name.clone()), Type::Any, 0);

                let mut local = HashMap::new();
                local.insert(name.clone(), bound_var.clone());
                locals.push(local);
                let children = node_ref
                    .children()
                    .map(|child| self.bind_expr_node(expr, child, locals))
                    .collect::<Result<Vec<_>, _>>()?;
                locals.pop();

                let mut new_tree = DynTree::new(ExprIR::Quantifier(qt.clone(), bound_var));
                let mut root = new_tree.root_mut();
                for child in children {
                    root.push_child_tree(child);
                }
                Ok(new_tree)
            }
            ExprIR::ListComprehension(name) => {
                let bound_var: Variable = self.fresh_var(Some(name.clone()), Type::Any, 0);

                let mut local = HashMap::new();
                local.insert(name.clone(), bound_var.clone());
                locals.push(local);
                let children = node_ref
                    .children()
                    .map(|child| self.bind_expr_node(expr, child, locals))
                    .collect::<Result<Vec<_>, _>>()?;
                locals.pop();

                // Child 1 is the WHERE condition — validate it returns boolean
                if children.len() > 1 && !Self::expr_may_return_boolean(children[1].root()) {
                    return Err(String::from("Expected boolean predicate"));
                }

                let mut new_tree = DynTree::new(ExprIR::ListComprehension(bound_var));
                let mut root = new_tree.root_mut();
                for child in children {
                    root.push_child_tree(child);
                }
                Ok(new_tree)
            }
            _ => {
                let children = node_ref
                    .children()
                    .map(|child| self.bind_expr_node(expr, child, locals))
                    .collect::<Result<Vec<_>, _>>()?;
                let new_data = match node_ref.data().clone() {
                    ExprIR::Null => ExprIR::Null,
                    ExprIR::Bool(b) => ExprIR::Bool(b),
                    ExprIR::Integer(i) => ExprIR::Integer(i),
                    ExprIR::Float(fl) => ExprIR::Float(fl),
                    ExprIR::String(s) => ExprIR::String(s),
                    ExprIR::List => ExprIR::List,
                    ExprIR::Map => ExprIR::Map,
                    ExprIR::Parameter(p) => ExprIR::Parameter(p),
                    ExprIR::Length => ExprIR::Length,
                    ExprIR::GetElement => ExprIR::GetElement,
                    ExprIR::GetElements => ExprIR::GetElements,
                    ExprIR::IsNode => ExprIR::IsNode,
                    ExprIR::IsRelationship => ExprIR::IsRelationship,
                    ExprIR::Or => ExprIR::Or,
                    ExprIR::Xor => ExprIR::Xor,
                    ExprIR::And => ExprIR::And,
                    ExprIR::Not => ExprIR::Not,
                    ExprIR::Negate => ExprIR::Negate,
                    ExprIR::Eq => ExprIR::Eq,
                    ExprIR::Neq => ExprIR::Neq,
                    ExprIR::Lt => ExprIR::Lt,
                    ExprIR::Gt => ExprIR::Gt,
                    ExprIR::Le => ExprIR::Le,
                    ExprIR::Ge => ExprIR::Ge,
                    ExprIR::In => ExprIR::In,
                    ExprIR::Add => ExprIR::Add,
                    ExprIR::Sub => ExprIR::Sub,
                    ExprIR::Mul => ExprIR::Mul,
                    ExprIR::Div => ExprIR::Div,
                    ExprIR::Pow => ExprIR::Pow,
                    ExprIR::Modulo => ExprIR::Modulo,
                    ExprIR::Distinct => ExprIR::Distinct,
                    ExprIR::Property(prop) => {
                        // Property access is not valid on Path types.
                        if let Some(first_child) = children.first() {
                            let root = first_child.root();
                            let inner = if matches!(root.data(), ExprIR::Paren) {
                                root.get_child(0)
                            } else {
                                Some(root)
                            };
                            if inner.is_some_and(
                                |n| matches!(n.data(), ExprIR::Variable(v) if v.ty == Type::Path),
                            ) {
                                return Err("Type mismatch: expected Map, Node, Edge, \
                                             Datetime, Date, Time, Duration, Null, \
                                             or Point but was Path"
                                    .to_string());
                            }
                        }
                        ExprIR::Property(prop)
                    }
                    ExprIR::FuncInvocation(func) => ExprIR::FuncInvocation(func),
                    ExprIR::Paren => ExprIR::Paren,
                    ExprIR::Variable(_)
                    | ExprIR::Quantifier(_, _)
                    | ExprIR::ListComprehension(_) => unreachable!("handled above"),
                };
                let mut new_tree = DynTree::new(new_data);
                let mut root = new_tree.root_mut();
                for child in children {
                    root.push_child_tree(child);
                }
                Ok(new_tree)
            }
        }
    }

    fn define_name_in_scope(
        &mut self,
        name: Arc<String>,
        ty: Type,
        allow_reuse: bool,
    ) -> Result<Variable, String> {
        // Anonymous variables (_anon_*) should always be fresh, never reused
        if !name.starts_with("_anon")
            && let Some(existing) = self.current_env().get(&name)
        {
            if !allow_reuse {
                return Err(format!("Variable `{name}` already declared"));
            }
            self.ensure_type(existing, &ty)?;
            return Ok(existing.clone());
        }

        let var = self.fresh_var(Some(name.clone()), ty, self.env_stack.len() as u32 - 1);
        self.current_env_mut().insert(name, var.clone());
        Ok(var)
    }

    fn project_name(
        &mut self,
        name: &Arc<String>,
        ty: Type,
    ) -> Variable {
        let var = self.fresh_var(Some(name.clone()), ty, self.env_stack.len() as u32 - 1);
        self.current_env_mut().insert(name.clone(), var.clone());
        var
    }

    fn resolve_name(
        &mut self,
        name: &Arc<String>,
        locals: &[HashMap<Arc<String>, Variable>],
    ) -> Result<Variable, String> {
        // Special placeholder for aggregate function ordering - always create a fresh variable
        if name.as_str() == "__agg_order_by_placeholder__" {
            return Ok(self.fresh_var(
                Some(name.clone()),
                Type::Any,
                self.env_stack.len() as u32 - 1,
            ));
        }

        // Anonymous variables should also create fresh variables
        if name.starts_with("_anon") {
            return Ok(self.fresh_var(
                Some(name.clone()),
                Type::Any,
                self.env_stack.len() as u32 - 1,
            ));
        }

        for scope in locals.iter().rev() {
            if let Some(var) = scope.get(name) {
                return Ok(var.clone());
            }
        }

        if let Some(var) = self.current_env().get(name) {
            return Ok(var.clone());
        }

        if self.use_parent_scope {
            if let Some(var) = self.parent_to_child_scope.get(name) {
                return Ok(var.clone());
            } else if let Some((_, var)) = self.copy_from_parent.get(name) {
                return Ok(var.clone());
            } else if let Some(var) = self
                .env_stack
                .get(self.env_stack.len().saturating_sub(2))
                .and_then(|parent_env| parent_env.get(name))
                .cloned()
            {
                // Copy variable from parent scope into current scope
                let current_scope_id = self.env_stack.len() as u32 - 1;
                let var_id = self.next_var_id;
                self.next_var_id += 1;

                let copied_var = Variable {
                    name: var.name.clone(),
                    id: var_id,
                    scope_id: current_scope_id,
                    ty: var.ty.clone(),
                };
                self.current_env_mut()
                    .insert(name.clone(), copied_var.clone());
                self.copy_from_parent
                    .insert(name.clone(), (var, copied_var.clone()));
                return Ok(copied_var);
            }
        }

        Err(format!("'{}' not defined", name.as_str()))
    }

    const fn fresh_var(
        &mut self,
        name: Option<Arc<String>>,
        ty: Type,
        scope_id: u32,
    ) -> Variable {
        let _current_scope_idx = scope_id as usize;
        let var_id = self.next_var_id;
        self.next_var_id += 1;

        Variable {
            name,
            id: var_id,
            scope_id,
            ty,
        }
    }

    fn ensure_type(
        &self,
        existing: &Variable,
        ty: &Type,
    ) -> Result<(), String> {
        if (ty == &Type::Relationship && (existing.ty == Type::Node || existing.ty == Type::Path))
            || (ty == &Type::Node
                && (existing.ty == Type::Relationship || existing.ty == Type::Path))
            || (ty == &Type::Path
                && (existing.ty == Type::Node || existing.ty == Type::Relationship))
        {
            return Err(format!(
                "The alias '{}' was specified for both a node and a relationship.",
                existing.as_str()
            ));
        }
        Ok(())
    }

    /// Walks an expression tree and validates that every AND/OR/XOR/NOT node
    /// has operands that can return boolean.  Produces a "Type mismatch"
    /// error for non-filter contexts (e.g. RETURN expressions).
    fn validate_boolean_operands(expr: &QueryExpr<Variable>) -> Result<(), String> {
        Self::validate_boolean_operands_impl(expr.root())
    }

    fn validate_boolean_operands_impl(
        node: orx_tree::Node<orx_tree::Dyn<ExprIR<Variable>>>
    ) -> Result<(), String> {
        if matches!(
            node.data(),
            ExprIR::And | ExprIR::Or | ExprIR::Xor | ExprIR::Not
        ) {
            for child in node.children() {
                if !Self::expr_may_return_boolean(child) {
                    return Err(String::from("Type mismatch: expected Boolean"));
                }
            }
        }
        for child in node.children() {
            Self::validate_boolean_operands_impl(child)?;
        }
        Ok(())
    }

    /// Recursively determines whether an expression tree node can return a
    /// boolean value at compile time. For nodes whose type cannot be
    /// determined statically (variables, parameters, properties), returns
    /// `true` (deferring to the runtime check).
    #[allow(clippy::needless_pass_by_value)]
    fn expr_may_return_boolean(node: orx_tree::Node<orx_tree::Dyn<ExprIR<Variable>>>) -> bool {
        match node.data() {
            // Logical operators – result is boolean, but each child must
            // also return boolean (mirrors the C recursive FilterTree_Valid)
            ExprIR::Or | ExprIR::And | ExprIR::Xor | ExprIR::Not => {
                node.children().all(Self::expr_may_return_boolean)
            }

            // Transparent wrappers – recurse into the single child
            ExprIR::Paren | ExprIR::Distinct => {
                node.get_child(0).is_some_and(Self::expr_may_return_boolean)
            }

            // Function calls – use the registered return type
            ExprIR::FuncInvocation(func) => func.ret_type.can_return_boolean(),

            // Non-boolean: literals, arithmetic, subscript, list comprehension
            ExprIR::Integer(_)
            | ExprIR::Float(_)
            | ExprIR::String(_)
            | ExprIR::List
            | ExprIR::Map
            | ExprIR::Negate
            | ExprIR::Add
            | ExprIR::Sub
            | ExprIR::Mul
            | ExprIR::Div
            | ExprIR::Pow
            | ExprIR::Modulo
            | ExprIR::Length
            | ExprIR::GetElement
            | ExprIR::GetElements
            | ExprIR::ListComprehension(_) => false,

            // Boolean literals, comparisons, predicates, and runtime-typed nodes
            ExprIR::Bool(_)
            | ExprIR::Null
            | ExprIR::Eq
            | ExprIR::Neq
            | ExprIR::Lt
            | ExprIR::Gt
            | ExprIR::Le
            | ExprIR::Ge
            | ExprIR::In
            | ExprIR::IsNode
            | ExprIR::IsRelationship
            | ExprIR::Quantifier(_, _)
            | ExprIR::Variable(_)
            | ExprIR::Parameter(_)
            | ExprIR::Property(_) => true,
        }
    }
}
