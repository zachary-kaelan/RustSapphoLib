use pest::iterators::Pair;
use crate::scripting::ast::{AstNode, Rule, BnumType};
use crate::scripting::ast_expr::build_ast_from_expr;

pub enum ActorAstNode {
    ActorGroup {
        bnum_type: BnumType,
        target: Box<Vec<String>>,
        group: Box<AstNode>,
    },
    BnumTargetAssign(Box<AstNode>)
}

pub fn parse_actor_inner(pair: Pair<Rule>) -> Vec<ActorAstNode> {
    let mut nodes: Vec<ActorAstNode> = vec![];
    let mut pair = pair.into_inner();
    for stmt in pair {
        match stmt.as_rule() {
            Rule::actor_group => {
                nodes.push(parse_actor_group(stmt))   
            }
            Rule::bnum_target_assign => {
                nodes.push(ActorAstNode::BnumTargetAssign(Box::new(build_ast_from_expr(stmt)))))
            }
            unknown_stmt => panic!("Unknown actor statement {:?}", unknown_stmt)
        }
    }
    nodes
}

fn parse_actor_group(actor_group: Pair<Rule>) -> ActorAstNode {
    let mut pair = actor_group.into_inner();
    let group_type = pair.next().unwrap();
    let bnum_group = pair.next().unwrap();
    let bnum_group = build_ast_from_expr(bnum_group);
    let mut target: Vec<String> = vec![];
    let bnum_type = match group_type.as_rule() {
        Rule::perception_group_type => {
            let mut group_type = group_type.into_inner();
            let target = group_type.next().unwrap();
            target.push(String::from(target.as_str()));
            BnumType::Perception
        }
        Rule::circumstantial_group_type => {
            let mut group_type = group_type.into_inner();
            let target1 = group_type.next().unwrap();
            let target2 = group_type.next().unwrap();
            target.push(String::from(target1.as_str()));
            target.push(String::from(target2.as_str()));
            BnumType::Circumstantial
        }
        _ => {
            match group_type.as_str() {
                "PERSONALITY" => BnumType::Personality,
                "SELF" => BnumType::SelfPerception,
                "ACCORDANCE" => BnumType::Accordance,
                unknown_group_type => panic!("Unknown actor group type {}", unknown_group_type)
            }
        }
    };
    ActorAstNode::ActorGroup {
        bnum_type,
        target: Box::new(target),
        group: Box::new(bnum_group)
    }
}