use partiql_value::{BindingsName, Value};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct OpId(usize);

impl OpId {
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct LogicalPlan<T>
where
    T: Default,
{
    nodes: Vec<T>,
    /// Third argument indicates the branch number into the outgoing node.
    edges: Vec<(OpId, OpId, u8)>,
}

impl<T> LogicalPlan<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_operator(&mut self, op: T) -> OpId {
        self.nodes.push(op);
        OpId(self.operator_count())
    }

    #[inline]
    pub fn add_flow(&mut self, src: OpId, dst: OpId) {
        assert!(src.index() <= self.operator_count());
        assert!(dst.index() <= self.operator_count());

        self.edges.push((src, dst, 0));
    }

    #[inline]
    pub fn add_flow_with_branch_num(&mut self, src: OpId, dst: OpId, branch_num: u8) {
        assert!(src.index() <= self.operator_count());
        assert!(dst.index() <= self.operator_count());

        self.edges.push((src, dst, branch_num));
    }

    #[inline]
    pub fn extend_with_flows(&mut self, flows: &[(OpId, OpId)]) {
        flows.iter().for_each(|&(s, d)| self.add_flow(s, d));
    }

    #[inline]
    pub fn operator_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn operators(&self) -> &Vec<T> {
        &self.nodes
    }

    pub fn flows(&self) -> &Vec<(OpId, OpId, u8)> {
        &self.edges
    }
}

// TODO: other expressions modeled in logical plan and evaluator -- IN, IS, BETWEEN

// TODO we should replace this enum with some identifier that can be looked up in a symtab/funcregistry?
#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}

// TODO we should replace this enum with some identifier that can be looked up in a symtab/funcregistry?
#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BinaryOp {
    And,
    Or,
    Concat,
    Eq,
    Neq,
    Gt,
    Gteq,
    Lt,
    Lteq,

    // Arithmetic ops
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,

    In,
}

#[derive(Clone, Debug)]
pub enum PathComponent {
    Key(String),
    Index(i64),
}

#[derive(Clone, Debug)]
pub struct IsTypeExpr {
    pub not: bool,
    pub expr: Box<ValueExpr>,
    pub is_type: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    NullType,
    BooleanType,
    Integer2Type,
    Integer4Type,
    Integer8Type,
    DecimalType,
    NumericType,
    RealType,
    DoublePrecisionType,
    TimestampType,
    CharacterType,
    CharacterVaryingType,
    MissingType,
    StringType,
    SymbolType,
    BlobType,
    ClobType,
    DateType,
    TimeType,
    ZonedTimestampType,
    StructType,
    TupleType,
    ListType,
    SexpType,
    BagType,
    AnyType,
    // TODO CustomType
}

#[derive(Clone, Debug)]
pub struct NullIfExpr {
    pub lhs: Box<ValueExpr>,
    pub rhs: Box<ValueExpr>,
}

#[derive(Clone, Debug)]
pub struct CoalesceExpr {
    pub elements: Vec<ValueExpr>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum ValueExpr {
    // TODO other variants
    UnExpr(UnaryOp, Box<ValueExpr>),
    BinaryExpr(BinaryOp, Box<ValueExpr>, Box<ValueExpr>),
    Lit(Box<Value>),
    Path(Box<ValueExpr>, Vec<PathComponent>),
    VarRef(BindingsName),
    TupleExpr(TupleExpr),
    ListExpr(ListExpr),
    BagExpr(BagExpr),
    BetweenExpr(BetweenExpr),
    SubQueryExpr(SubQueryExpr),
    SimpleCase(SimpleCase),
    SearchedCase(SearchedCase),
    IsTypeExpr(IsTypeExpr),
    NullIfExpr(NullIfExpr),
    CoalesceExpr(CoalesceExpr),
}

#[derive(Clone, Debug, Default)]
pub struct TupleExpr {
    pub attrs: Vec<ValueExpr>,
    pub values: Vec<ValueExpr>,
}

impl TupleExpr {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ListExpr {
    pub elements: Vec<ValueExpr>,
}

impl ListExpr {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct BagExpr {
    pub elements: Vec<ValueExpr>,
}

impl BagExpr {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub struct BetweenExpr {
    pub value: Box<ValueExpr>,
    pub from: Box<ValueExpr>,
    pub to: Box<ValueExpr>,
}

#[derive(Clone, Debug)]
pub struct SimpleCase {
    pub expr: Box<ValueExpr>,
    pub cases: Vec<(Box<ValueExpr>, Box<ValueExpr>)>,
    pub default: Option<Box<ValueExpr>>,
}

#[derive(Clone, Debug)]
pub struct SearchedCase {
    pub cases: Vec<(Box<ValueExpr>, Box<ValueExpr>)>,
    pub default: Option<Box<ValueExpr>>,
}

// Bindings -> Bindings : Where, OrderBy, Offset, Limit, Join, SetOp, Select, Distinct, GroupBy, Unpivot, Let
// Values   -> Bindings : From
// Bindings -> Values   : Select Value

#[derive(Debug, Clone, Default)]
pub enum BindingsExpr {
    Scan(Scan),
    Unpivot(Unpivot),
    Filter(Filter),
    OrderBy,
    Offset,
    Limit,
    Join(Join),
    SetOp,
    Project(Project),
    ProjectValue(ProjectValue),
    Distinct,
    GroupBy,
    #[default]
    Sink,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum BindingsToValueExpr {}

#[derive(Debug)]
#[allow(dead_code)] // TODO remove once out of PoC
pub enum ValueToBindingsExpr {}

/// [`Scan`] bridges from [`ValueExpr`]s to [`BindingExpr`]s
#[derive(Debug, Clone)]
pub struct Scan {
    pub expr: ValueExpr,
    pub as_key: String,
    pub at_key: Option<String>,
}

/// [`Unpivot`] bridges from [`ValueExpr`]s to [`BindingExpr`]s
#[derive(Debug, Clone)]
pub struct Unpivot {
    pub expr: ValueExpr,
    pub as_key: String,
    pub at_key: Option<String>,
}

#[derive(Debug, Clone)]
pub enum JoinKind {
    Inner,
    Left,
    Right,
    Full,
    Cross,
    // TODO revisit JOINS to consider the `Lateral` logic as part of current joins
    CrossLateral,
}

#[derive(Debug, Clone)]
pub struct Join {
    pub kind: JoinKind,
    pub on: Option<ValueExpr>,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub expr: ValueExpr,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub exprs: HashMap<String, ValueExpr>,
}

#[derive(Debug, Clone)]
pub struct ProjectValue {
    pub expr: ValueExpr,
}

#[derive(Clone, Debug)]
pub struct SubQueryExpr {
    pub plan: LogicalPlan<BindingsExpr>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let mut p: LogicalPlan<BindingsExpr> = LogicalPlan::new();
        let a = p.add_operator(BindingsExpr::OrderBy);
        let b = p.add_operator(BindingsExpr::Sink);
        let c = p.add_operator(BindingsExpr::Limit);
        let d = p.add_operator(BindingsExpr::GroupBy);
        let e = p.add_operator(BindingsExpr::Offset);
        p.add_flow(a, b);
        p.add_flow(a, c);
        p.add_flow(b, c);
        p.extend_with_flows(&[(c, d), (d, e)]);
        assert_eq!(5, p.operators().len());
        assert_eq!(5, p.flows().len());
    }
}
