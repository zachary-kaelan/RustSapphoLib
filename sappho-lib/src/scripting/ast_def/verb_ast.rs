use crate::scripting::ast::{AstNode, BnumType, Rule};
use crate::scripting::ast_expr::build_ast_from_expr;
use pest::iterators::Pair;

#[derive(PartialEq, Debug, Clone)]
pub enum VerbAstNode {
    VerbWeights {
        bnum_type: BnumType,
        target: Vec<String>,
        weights: Box<AstNode>,
    },
    VerbMath(Box<AstNode>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct VerbDefAst {
    pub personality_weights: Option<Box<AstNode>>,
    pub perception_weights: Option<Box<AstNode>>,
}

pub fn parse_verb_inner(pair: Pair<Rule>) -> Vec<VerbAstNode> {
    let mut nodes: Vec<VerbAstNode> = vec![];
    let pair = pair.into_inner();
    for stmt in pair {
        match stmt.as_rule() {
            Rule::verb_group => nodes.push(parse_verb_group(stmt)),
            Rule::verb_personality_weights => {
                nodes.push(VerbAstNode::BnumTargetAssign(Box::new(
                    build_ast_from_expr(stmt),
                )))
            }
            unknown_stmt => {
                panic!("Unknown verb statement {:?}", unknown_stmt)
            }
        }
    }
    nodes
}

fn parse_verb_group(verb_group: Pair<Rule>) -> VerbAstNode {
    let mut pair = verb_group.into_inner();
    let group_type = pair.next().unwrap();
    let bnum_group = pair.next().unwrap();
    let bnum_group = build_ast_from_expr(bnum_group);
    let mut targets: Vec<String> = vec![];
    let bnum_type = match group_type.as_rule() {
        Rule::perception_group_type => {
            let mut group_type = group_type.into_inner();
            let target = group_type.next().unwrap();
            let target = target.as_str();
            targets.push(String::from(&target[1..target.len() - 1]));
            BnumType::Perception
        }
        Rule::circumstantial_group_type => {
            let mut group_type = group_type.into_inner();
            let target1 = group_type.next().unwrap();
            let target2 = group_type.next().unwrap();
            targets.push(String::from(target1.as_str()));
            targets.push(String::from(target2.as_str()));
            BnumType::Circumstantial
        }
        _ => match group_type.as_str() {
            "PERSONALITY" => BnumType::Personality,
            "SELF" => BnumType::SelfPerception,
            "ACCORDANCE" => BnumType::Accordance,
            unknown_group_type => {
                panic!("Unknown verb group type {}", unknown_group_type)
            }
        },
    };
    VerbAstNode::VerbGroup {
        bnum_type,
        target: targets,
        group: Box::new(bnum_group),
    }
}
