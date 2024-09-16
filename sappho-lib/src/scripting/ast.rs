use std::ffi::{CString};
use crate::{BNumber, BnumGroup, SparseBnumGroup};
use self::AstNode::*;
use pest::iterators::Pair;
use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;
use crate::scripting::ast::Rule::bnum_target;

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
    Personality
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DefType {
    Actor,
    Verb,
    Emotion
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Float(f32),
    FloatOptional(Option<f32>),
    BNumber(BNumber),
    BnumGroup(SparseBnumGroup),
    BnumTuple(SparseBnumGroup),
    Def {
        ident: String,
        display_name: CString,
        definition: Box<Vec<AstNode>>
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
    BnumAssign {
        ident: String,
        expr: Box<AstNode>,
    },
    BnumTargetAssign {
        bnum_type: BnumType,
        ident: String,
        target: Box<Vec<String>>,
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
            Rule::expr => {

            }
            Rule::definition => {

            }
            _ => {}
        }
    }

    Ok(ast)
}