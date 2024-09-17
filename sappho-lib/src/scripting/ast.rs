use crate::consts::{SparseBnumGroupT, BNUM_GROUP_SIZE};
use crate::scripting::ast_def::{build_ast_from_definition, DefInnerAstNode};
use crate::scripting::ast_expr::build_ast_from_expr;
use crate::value_aliases::{get_value_aliases_from_type, ValueAliasType};
use crate::{BNumber, SparseBNumber, SparseBnumGroup};
use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;
use std::ffi::CString;

#[derive(Parser)]
#[grammar = "src/scripting/sappho_script.pest"]
pub struct SapphoParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MonadicOp {
    Positive,
    Negate,
    Abs,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DyadicOp {
    Plus,
    Minus,
    Times,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TriadicOp {
    Blend,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BnumType {
    Accordance,
    Perception,
    Circumstantial,
    SelfPerception,
    Personality,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DefType {
    Actor,
    Verb,
    Emotion,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Float(f32),
    FloatOptional(Option<f32>),
    BNumber(BNumber),
    BnumWeightGroup(Vec<AstNode>),
    BnumGroup(Vec<AstNode>),
    BnumWeightTuple(Vec<AstNode>),
    BnumTuple(Vec<AstNode>),
    Def {
        ident: String,
        display_name: CString,
        definition: Box<DefInnerAstNode>,
    },
    MonadicOp {
        op: MonadicOp,
        expr: Box<AstNode>,
    },
    DyadicOp {
        op: DyadicOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    TriadicOp {
        op: TriadicOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        arg: Box<AstNode>,
    },
    BnumWeight {
        ident: String,
        expr: Box<AstNode>,
    },
    BnumAssign {
        ident: String,
        expr: Box<AstNode>,
    },
    BnumTargetAssign {
        bnum_type: BnumType,
        ident: String,
        target: Vec<String>,
        expr: Box<AstNode>,
    },
    Type(String),
    Ident(String),
    Str(CString),
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = SapphoParser::parse(Rule::document, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::bnum_target_assign => ast.push(build_ast_from_expr(pair)),
            Rule::definition => ast.push(build_ast_from_definition(pair)),
            _ => {}
        }
    }

    Ok(ast)
}

pub fn extract_bnum_group_or_tuple(
    node: &Option<Box<AstNode>>,
    value_type: ValueAliasType,
) -> Result<SparseBnumGroup, super::error::Error> {
    match node {
        None => Ok(SparseBnumGroup::new(None)),
        Some(node) => match node.as_ref() {
            AstNode::BnumGroup(g) => {
                let aliases = get_value_aliases_from_type(value_type);
                let mut bnums: [Option<f32>; BNUM_GROUP_SIZE] = [None; BNUM_GROUP_SIZE];
                for node in g {
                    match node {
                        AstNode::BnumAssign { ident, expr } => {
                            let f = match expr.as_ref() {
                                AstNode::Float(f) => Some(*f),
                                AstNode::FloatOptional(f) => *f,
                                _ => {
                                    unreachable!()
                                }
                            };
                            let index = aliases.iter().position(|x1| x1 == ident);
                            match index {
                                None => {
                                    return Err(super::error::Error::UnrecognizedBNumberAlias(
                                        ident.clone(),
                                    ));
                                }
                                Some(i) => {
                                    bnums[i] = f;
                                }
                            }
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                Ok(SparseBnumGroup::from(bnums))
            }
            AstNode::BnumTuple(g) => {
                if g.len() != BNUM_GROUP_SIZE {
                    Err(super::error::Error::InsufficientTupleArgs(g.len()))
                } else {
                    let bnums = extract_bnum_tuple(g);
                    Ok(SparseBnumGroup::from(bnums))
                }
            }
            _ => unreachable!(),
        },
    }
}

pub fn extract_weights_group_or_tuple(
    node: &Option<Box<AstNode>>,
    value_type: ValueAliasType,
) -> Result<Option<[Option<f32>; BNUM_GROUP_SIZE]>, super::error::Error> {
    match node {
        None => Ok(None),
        Some(node) => match node.as_ref() {
            AstNode::BnumWeightGroup(g) => {
                let aliases = get_value_aliases_from_type(value_type);
                let mut bnums: [Option<f32>; BNUM_GROUP_SIZE] = [None; BNUM_GROUP_SIZE];
                for node in g {
                    match node {
                        AstNode::BnumWeight { ident, expr } => {
                            let f = match expr.as_ref() {
                                AstNode::Float(f) => Some(*f),
                                AstNode::FloatOptional(f) => *f,
                                _ => {
                                    unreachable!()
                                }
                            };
                            let index = aliases.iter().position(|x1| x1 == ident);
                            match index {
                                None => {
                                    return Err(super::error::Error::UnrecognizedBNumberAlias(
                                        ident.clone(),
                                    ));
                                }
                                Some(i) => {
                                    bnums[i] = f;
                                }
                            }
                        }
                        _ => {
                            unreachable!()
                        }
                    }
                }
                Ok(Some(bnums))
            }
            AstNode::BnumWeightTuple(g) => {
                if g.len() != BNUM_GROUP_SIZE {
                    Err(super::error::Error::InsufficientTupleArgs(g.len()))
                } else {
                    let bnums = extract_bnum_tuple(g);
                    Ok(Some(bnums))
                }
            }
            _ => unreachable!(),
        },
    }
}

fn extract_bnum_tuple(g: &Vec<AstNode>) -> [Option<f32>; 4] {
    let bnums: [Option<f32>; BNUM_GROUP_SIZE] = g
        .iter()
        .map(|x| match x {
            AstNode::Float(f) => Some(*f),
            AstNode::FloatOptional(f) => *f,
            _ => {
                unreachable!()
            }
        })
        .collect::<Vec<Option<f32>>>()
        .try_into()
        .expect("Incorrect Length");
    bnums
}

#[cfg(test)]
mod tests {
    use crate::scripting::ast::parse;
    use std;

    #[test]
    fn parse_test() {
        let file_text =
            std::fs::read_to_string("src/scripting/test.sappho").expect("Cannot read file");
        let ast_node = parse(&file_text).expect("Unsuccessful parse");
        println!("{:?}", &ast_node);
        panic!("");
    }
}
