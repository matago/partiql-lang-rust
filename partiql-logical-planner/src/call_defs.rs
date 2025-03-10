use itertools::Itertools;
use partiql_logical as logical;
use partiql_logical::ValueExpr;
use partiql_value::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use unicase::UniCase;

#[derive(Debug, Eq, PartialEq)]
pub enum CallArgument {
    Positional(ValueExpr),
    Named(String, ValueExpr),
}

#[derive(Debug)]
pub struct CallDef {
    names: Vec<&'static str>,
    overloads: Vec<CallSpec>,
}

impl CallDef {
    pub(crate) fn lookup(&self, args: &Vec<CallArgument>) -> ValueExpr {
        'overload: for overload in &self.overloads {
            let formals = &overload.input;
            if formals.len() != args.len() {
                continue 'overload;
            }

            let mut actuals = vec![];
            for i in 0..formals.len() {
                let formal = &formals[i];
                let actual = &args[i];
                if let Some(vexpr) = formal.transform(actual) {
                    actuals.push(vexpr);
                } else {
                    continue 'overload;
                }
            }

            return (overload.output)(actuals);
        }

        todo!("mismatched formal/actual arguments to {}", &self.names[0])
    }
}

impl CallDef {}

#[derive(Debug, Copy, Clone)]
pub enum CallSpecArg {
    Positional,
    Named(UniCase<&'static str>),
}

impl CallSpecArg {
    pub(crate) fn transform(&self, arg: &CallArgument) -> Option<ValueExpr> {
        match (self, arg) {
            (CallSpecArg::Positional, CallArgument::Positional(ve)) => Some(ve.clone()),
            (CallSpecArg::Named(formal_name), CallArgument::Named(arg_name, ve)) => {
                if formal_name == &UniCase::new(arg_name.as_str()) {
                    Some(ve.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl CallSpecArg {}

pub struct CallSpec {
    input: Vec<CallSpecArg>,
    output: Box<dyn Fn(Vec<ValueExpr>) -> logical::ValueExpr>,
}

impl Debug for CallSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallSpec [{:?}]", &self.input)
    }
}

fn function_call_def_char_len() -> CallDef {
    CallDef {
        names: vec!["char_length", "character_length"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::CharLength,
                    arguments: args,
                })
            }),
        }],
    }
}

fn function_call_def_octet_len() -> CallDef {
    CallDef {
        names: vec!["octet_length"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::OctetLength,
                    arguments: args,
                })
            }),
        }],
    }
}

fn function_call_def_bit_len() -> CallDef {
    CallDef {
        names: vec!["bit_length"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::BitLength,
                    arguments: args,
                })
            }),
        }],
    }
}

fn function_call_def_lower() -> CallDef {
    CallDef {
        names: vec!["lower"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::Lower,
                    arguments: args,
                })
            }),
        }],
    }
}

fn function_call_def_upper() -> CallDef {
    CallDef {
        names: vec!["upper"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::Upper,
                    arguments: args,
                })
            }),
        }],
    }
}

fn function_call_def_substring() -> CallDef {
    CallDef {
        names: vec!["substring"],
        overloads: vec![
            CallSpec {
                input: vec![
                    CallSpecArg::Positional,
                    CallSpecArg::Positional,
                    CallSpecArg::Positional,
                ],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::Substring,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![CallSpecArg::Positional, CallSpecArg::Positional],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::Substring,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![
                    CallSpecArg::Positional,
                    CallSpecArg::Named("from".into()),
                    CallSpecArg::Named("for".into()),
                ],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::Substring,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![CallSpecArg::Positional, CallSpecArg::Named("from".into())],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::Substring,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![CallSpecArg::Positional, CallSpecArg::Named("for".into())],
                output: Box::new(|mut args| {
                    args.insert(1, ValueExpr::Lit(Box::new(Value::Integer(0))));
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::Substring,
                        arguments: args,
                    })
                }),
            },
        ],
    }
}

fn function_call_def_position() -> CallDef {
    CallDef {
        names: vec!["position"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional, CallSpecArg::Named("in".into())],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::Position,
                    arguments: args,
                })
            }),
        }],
    }
}

fn function_call_def_trim() -> CallDef {
    CallDef {
        names: vec!["trim"],
        overloads: vec![
            CallSpec {
                input: vec![
                    CallSpecArg::Named("leading".into()),
                    CallSpecArg::Named("from".into()),
                ],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::LTrim,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![
                    CallSpecArg::Named("trailing".into()),
                    CallSpecArg::Named("from".into()),
                ],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::RTrim,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![
                    CallSpecArg::Named("both".into()),
                    CallSpecArg::Named("from".into()),
                ],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::BTrim,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![CallSpecArg::Named("from".into())],
                output: Box::new(|mut args| {
                    args.insert(
                        0,
                        ValueExpr::Lit(Box::new(Value::String(" ".to_string().into()))),
                    );

                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::BTrim,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![CallSpecArg::Positional],
                output: Box::new(|mut args| {
                    args.insert(
                        0,
                        ValueExpr::Lit(Box::new(Value::String(" ".to_string().into()))),
                    );
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::BTrim,
                        arguments: args,
                    })
                }),
            },
            CallSpec {
                input: vec![CallSpecArg::Positional, CallSpecArg::Named("from".into())],
                output: Box::new(|args| {
                    logical::ValueExpr::Call(logical::CallExpr {
                        name: logical::CallName::BTrim,
                        arguments: args,
                    })
                }),
            },
        ],
    }
}

fn function_call_def_coalesce() -> CallDef {
    CallDef {
        names: vec!["coalesce"],
        overloads: (0..15)
            .map(|n| CallSpec {
                input: std::iter::repeat(CallSpecArg::Positional)
                    .take(n)
                    .collect_vec(),
                output: Box::new(|args| {
                    logical::ValueExpr::CoalesceExpr(logical::CoalesceExpr { elements: args })
                }),
            })
            .collect_vec(),
    }
}

fn function_call_def_nullif() -> CallDef {
    CallDef {
        names: vec!["nullif"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional, CallSpecArg::Positional],
            output: Box::new(|mut args| {
                assert_eq!(args.len(), 2);
                let rhs = Box::new(args.pop().unwrap());
                let lhs = Box::new(args.pop().unwrap());
                logical::ValueExpr::NullIfExpr(logical::NullIfExpr { lhs, rhs })
            }),
        }],
    }
}

fn function_call_def_exists() -> CallDef {
    CallDef {
        names: vec!["exists"],
        overloads: vec![CallSpec {
            input: vec![CallSpecArg::Positional],
            output: Box::new(|args| {
                logical::ValueExpr::Call(logical::CallExpr {
                    name: logical::CallName::Exists,
                    arguments: args,
                })
            }),
        }],
    }
}

/// Function symbol table
#[derive(Debug)]
pub struct FnSymTab {
    calls: HashMap<UniCase<String>, CallDef>,
    synonyms: HashMap<UniCase<String>, UniCase<String>>,
}

impl FnSymTab {
    pub fn lookup(&self, fn_name: &str) -> Option<&CallDef> {
        self.synonyms
            .get(&fn_name.into())
            .and_then(|name| self.calls.get(name))
    }
}

pub fn function_call_def() -> FnSymTab {
    let mut calls: HashMap<UniCase<String>, CallDef> = HashMap::new();
    let mut synonyms: HashMap<UniCase<String>, UniCase<String>> = HashMap::new();

    for def in [
        function_call_def_char_len(),
        function_call_def_octet_len(),
        function_call_def_bit_len(),
        function_call_def_lower(),
        function_call_def_upper(),
        function_call_def_substring(),
        function_call_def_position(),
        function_call_def_trim(),
        function_call_def_coalesce(),
        function_call_def_nullif(),
        function_call_def_exists(),
    ] {
        assert!(!def.names.is_empty());
        let primary = def.names[0];
        synonyms.insert(primary.into(), primary.into());
        for &name in &def.names[1..] {
            synonyms.insert(name.into(), primary.into());
        }

        calls.insert(primary.into(), def);
    }

    FnSymTab { calls, synonyms }
}
