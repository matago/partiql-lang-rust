use partiql_eval as eval;
use partiql_eval::env::basic::MapBindings;
use partiql_logical as logical;
use partiql_parser::{Parsed, ParserResult};
use partiql_value::Value;

mod test_value;
pub(crate) use test_value::TestValue;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum EvaluationMode {
    Coerce,
    Error,
}

#[track_caller]
#[inline]
pub(crate) fn parse(statement: &str) -> ParserResult {
    partiql_parser::Parser::default().parse(statement)
}

#[track_caller]
#[inline]
pub(crate) fn lower(parsed: &Parsed) -> logical::LogicalPlan<logical::BindingsOp> {
    partiql_logical_planner::lower(parsed)
}

#[track_caller]
#[inline]
pub(crate) fn evaluate(
    logical: logical::LogicalPlan<logical::BindingsOp>,
    bindings: MapBindings<Value>,
) -> Value {
    let planner = eval::plan::EvaluatorPlanner;

    let mut plan = planner.compile(&logical);

    if let Ok(out) = plan.execute_mut(bindings) {
        out.result
    } else {
        Value::Missing
    }
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn fail_syntax(statement: &str) {
    let res = parse(statement);
    assert!(
        res.is_err(),
        "For `{statement}`, expected `Err(_)`, but was `{res:#?}`"
    );
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn pass_syntax(statement: &str) -> Parsed {
    let res = parse(statement);
    assert!(
        res.is_ok(),
        "For `{statement}`, expected `Ok(_)`, but was `{res:#?}`"
    );
    res.unwrap()
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn fail_semantics(_statement: &str) {
    todo!("fail_semantics")
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn pass_semantics(statement: &str) {
    let parsed = pass_syntax(statement);
    // TODO add Result to lower call
    let lowered: Result<_, ()> = Ok(lower(&parsed));
    assert!(
        lowered.is_ok(),
        "For `{statement}`, expected `Ok(_)`, but was `{lowered:#?}`"
    );
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn fail_eval(statement: &str, mode: EvaluationMode, env: &Option<TestValue>) {
    if let EvaluationMode::Error = mode {
        eprintln!("EvaluationMode::Error currently unsupported");
        return;
    }

    let parsed = parse(statement);
    let lowered = lower(&parsed.expect("parse"));
    let bindings = env
        .as_ref()
        .map(|e| (&e.value).into())
        .unwrap_or_else(MapBindings::default);
    let out = evaluate(lowered, bindings);

    println!("{:?}", &out);
    // TODO assert failure
}

#[track_caller]
#[inline]
#[allow(dead_code)]
pub(crate) fn pass_eval(
    statement: &str,
    mode: EvaluationMode,
    env: &Option<TestValue>,
    expected: &TestValue,
) {
    if let EvaluationMode::Error = mode {
        eprintln!("EvaluationMode::Error currently unsupported");
        return;
    }

    let parsed = parse(statement);
    let lowered = lower(&parsed.expect("parse"));
    let bindings = env
        .as_ref()
        .map(|e| (&e.value).into())
        .unwrap_or_else(MapBindings::default);
    let out = evaluate(lowered, bindings);

    println!("{:?}", &out);
    assert_eq!(out, expected.value);
}

#[allow(dead_code)]
pub(crate) fn environment() -> Option<TestValue> {
    None
}

// The `partiql_tests` module will be generated by `build.rs` build script.
#[cfg(feature = "conformance_test")]
mod partiql_tests;
