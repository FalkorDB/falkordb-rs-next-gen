use std::{fmt::Display, vec::IntoIter};

use orx_tree::{Dyn, DynTree, NodeMut, NodeRef};

use crate::{
    ast::{ExprIR, NodePattern, Pattern, QueryIR, RelationshipPattern, SupportAggregation},
    tree,
    value::Value,
};

#[derive(Clone, Debug)]
pub enum IR {
    Empty,
    Call(String, Vec<DynTree<ExprIR>>),
    Unwind(DynTree<ExprIR>, String, Option<IntoIter<Value>>),
    Create(Pattern),
    Delete(Vec<DynTree<ExprIR>>),
    NodeScan(NodePattern),
    RelationshipScan(RelationshipPattern),
    Filter(DynTree<ExprIR>),
    Aggregate(String, Vec<DynTree<ExprIR>>, Vec<DynTree<ExprIR>>),
    With(Vec<DynTree<ExprIR>>, bool, bool),
    Return(Vec<DynTree<ExprIR>>, bool, bool),
    Commit,
}

impl Display for ExprIR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Integer(x) => write!(f, "{x}"),
            Self::Float(x) => write!(f, "{x}"),
            Self::String(x) => write!(f, "\"{x}\""),
            Self::Var(x) => write!(f, "{x}"),
            Self::Parameter(x) => write!(f, "${x}"),
            Self::List => write!(f, "list"),
            Self::Length => write!(f, "length"),
            Self::GetElement => write!(f, "get_element"),
            Self::GetElements => write!(f, "get_elements"),
            Self::IsNull => write!(f, "is_null"),
            Self::IsNode => write!(f, "is_node"),
            Self::IsRelationship => write!(f, "is_relationship"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::And => write!(f, "and"),
            Self::Not => write!(f, "not"),
            Self::Negate => write!(f, "negate"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Lt => write!(f, "lt"),
            Self::Gt => write!(f, "gt"),
            Self::Le => write!(f, "le"),
            Self::Ge => write!(f, "ge"),
            Self::In => write!(f, "in"),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Pow => write!(f, "^"),
            Self::Modulo => write!(f, "%"),
            Self::FuncInvocation(x) => write!(f, "{x}()"),
            Self::Map => write!(f, "{{}}"),
            Self::Set(x) => write!(f, "{x} ="),
        }
    }
}

impl Display for IR {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Call(_, _) => write!(f, "Call"),
            Self::Unwind(_, _, _) => write!(f, "Unwind"),
            Self::Create(_) => write!(f, "Create"),
            Self::Delete(_) => write!(f, "Delete"),
            Self::NodeScan(_) => write!(f, "NodeScan"),
            Self::RelationshipScan(_) => write!(f, "RelationshipScan"),
            Self::Filter(_) => write!(f, "Filter"),
            Self::Aggregate(_, _, _) => write!(f, "Aggregate"),
            Self::With(_, _, _) => write!(f, "With"),
            Self::Return(_, _, _) => write!(f, "Return"),
            Self::Commit => write!(f, "Commit"),
        }
    }
}

pub struct Planner {
    var_id: u64,
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

impl Planner {
    #[must_use]
    pub const fn new() -> Self {
        Self { var_id: 0 }
    }

    fn next_var(&mut self) -> String {
        let id = self.var_id;
        self.var_id += 1;
        format!("var_{id}")
    }

    fn plan_aggregation(
        &mut self,
        agg_ctx_var: String,
        expr: &mut NodeMut<Dyn<ExprIR>>,
    ) {
        match expr.data() {
            ExprIR::FuncInvocation(_) => {
                expr.push_child_tree(tree!(ExprIR::Var(agg_ctx_var)));
            }
            ExprIR::Set(_) => {
                self.plan_aggregation(agg_ctx_var, &mut expr.get_child_mut(0).unwrap());
            }
            _ => unreachable!(),
        }
    }

    fn plan_match(
        &mut self,
        mut pattern: Pattern,
    ) -> DynTree<IR> {
        if pattern.relationships.is_empty() {
            let mut iter = pattern.nodes.into_iter().rev();
            let mut body = tree!(IR::NodeScan(iter.next().unwrap()));
            for node in iter {
                body = tree!(IR::NodeScan(node), body);
            }
            return body;
        }
        if pattern.relationships.len() == 1 {
            return tree!(IR::RelationshipScan(pattern.relationships.pop().unwrap()));
        }
        tree!(IR::Empty)
    }

    fn plan_return(
        &mut self,
        exprs: Vec<DynTree<ExprIR>>,
        write: bool,
    ) -> DynTree<IR> {
        let res = if exprs.iter().any(|e| e.root().is_aggregation()) {
            let mut group_by_keys = Vec::new();
            let mut aggregations = Vec::new();
            let agg_ctx_var = self.next_var();
            for expr in exprs {
                if expr.root().is_aggregation() {
                    // self.plan_aggregation(agg_ctx_var.to_string(), &mut expr.root_mut());
                    aggregations.push(expr);
                } else {
                    group_by_keys.push(expr);
                }
            }
            tree!(IR::Aggregate(agg_ctx_var, group_by_keys, aggregations))
        } else {
            tree!(IR::Return(exprs, write, false))
        };
        if write { tree!(IR::Commit, res) } else { res }
    }

    fn plan_query(
        &mut self,
        q: Vec<QueryIR>,
        write: bool,
    ) -> DynTree<IR> {
        let iter = &mut q.into_iter();
        let mut res = self.plan_inner(iter.next().unwrap());
        for e in iter {
            let mut n = self.plan_inner(e);
            n.root_mut().push_child_tree(res);
            res = n;
        }
        if write {
            res = tree!(IR::Commit, res);
        }
        res
    }

    fn plan_inner(
        &mut self,
        ir: QueryIR,
    ) -> DynTree<IR> {
        match ir {
            QueryIR::Call(name, exprs) => tree!(IR::Call(name, exprs)),
            QueryIR::Match(pattern) => self.plan_match(pattern),
            QueryIR::Unwind(expr, alias) => tree!(IR::Unwind(expr, alias, None)),
            QueryIR::Where(expr) => tree!(IR::Filter(expr)),
            QueryIR::Create(pattern) => tree!(IR::Create(pattern)),
            QueryIR::Delete(exprs) => tree!(IR::Delete(exprs)),
            QueryIR::With(exprs, write) => tree!(IR::With(exprs, write, false)),
            QueryIR::Return(exprs, write) => self.plan_return(exprs, write),
            QueryIR::Query(q, write) => self.plan_query(q, write),
        }
    }

    #[must_use]
    pub fn plan(
        &mut self,
        ir: QueryIR,
        _debug: bool,
    ) -> DynTree<IR> {
        self.plan_inner(ir)
    }
}
