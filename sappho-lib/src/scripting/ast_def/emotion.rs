use pest::iterators::Pair;
use crate::scripting::ast::{AstNode, Rule, BnumType};
use crate::scripting::ast_expr::build_ast_from_expr;

pub struct EmotionDefAst {
    pub personality_modifiers: Option<Box<AstNode>>,
    pub personality_weights: Option<Box<AstNode>>,
    pub perception_modifiers: Option<Box<AstNode>>,
    pub perception_weights: Option<Box<AstNode>>,
}

pub fn parse_emotion_inner(pair: Pair<Rule>) -> EmotionDefAst {
    let mut emotiondef = EmotionDefAst { 
        personality_modifiers: None, personality_weights: None,
        perception_modifiers: None, perception_weights: None };
    let mut pair = pair.into_inner();
    for stmt in pair {
        let ast_node = build_ast_from_expr(stmt);
        match stmt.as_rule() {
            Rule::emotion_personality_modifiers => {
                emotiondef.personality_modifiers = Some(Box::new(ast_node));
            }
            Rule::emotion_perception_modifiers => {
                emotiondef.perception_modifiers = Some(Box::new(ast_node));
            }
            Rule::emotion_personality_weights => {
                emotiondef.personality_weights = Some(Box::new(ast_node));
            }
            Rule::emotion_perception_weights => {
                emotiondef.perception_weights = Some(Box::new(ast_node));
            }
            unknown_stmt => panic!("Unknown emotion statement {:?}", unknown_stmt)
        }
    }
    emotiondef
}