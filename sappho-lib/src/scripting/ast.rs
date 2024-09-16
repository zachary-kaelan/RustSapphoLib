use std::ffi::{CString};
use crate::{BNumber, SparseBnumGroup};
use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;
use crate::scripting::ast_def::{build_ast_from_definition, DefInnerAstNode};
use crate::scripting::ast_expr::build_ast_from_expr;

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
        definition: Box<DefInnerAstNode>
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
            Rule::bnum_target_assign => {
                ast.push(build_ast_from_expr(pair))
            }
            Rule::definition => {
                ast.push(build_ast_from_definition(pair))
            }
            _ => {}
        }
    }

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use std;
    use crate::scripting::ast::parse;

    #[test]
    fn parse_test() {
        let file_text = std::fs::read_to_string("src/scripting/test.sappho").expect("Cannot read file");
        let ast_node = parse(&file_text).expect("Unsuccessful parse");
        println!("{:?}", &ast_node);
        panic!("");
    }
}