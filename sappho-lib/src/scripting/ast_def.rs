mod actor_ast;
mod emotion_ast;

use std::ffi::CString;
use pest::iterators::Pair;
use crate::scripting::ast::{AstNode, DefType, Rule};
use crate::scripting::ast_def::actor_ast::{parse_actor_inner, ActorAstNode};
use crate::scripting::ast_def::emotion_ast::{parse_emotion_inner, EmotionDefAst};

#[derive(PartialEq, Debug, Clone)]
pub enum DefInnerAstNode {
    ActorDef(Box<Vec<ActorAstNode>>),
    EmotionDef(Box<EmotionDefAst>)
}

pub fn build_ast_from_definition(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let def_type = pair.next().unwrap();
    let def_type = match def_type.as_str() {
        "ACTOR" => DefType::Actor,
        "VERB" => DefType::Verb,
        "EMOTION" => DefType::Emotion,
        unknown_def_type => panic!("Unexpected def type: {}", unknown_def_type),
    };
    let ident = pair.next().unwrap();
    let display_name = pair.next().unwrap();
    let def_inner = pair.next().unwrap();
    let def_inner = parse_definition_inner(def_inner, def_type);
    AstNode::Def {
        ident: String::from(ident.as_str()),
        display_name: CString::new(display_name.as_str()).unwrap(),
        definition: Box::new(def_inner)
    }
}

fn parse_definition_inner(pair: Pair<Rule>, def_type: DefType) -> DefInnerAstNode {
    match pair.as_rule() {
        Rule::actor_def_inner => {
            if def_type != DefType::Actor {
                panic!("Matched actor definition for definition type {:?}", def_type)
            }
            DefInnerAstNode::ActorDef(Box::new(parse_actor_inner(pair)))
        },
        Rule::emotion_def_inner => {
            DefInnerAstNode::EmotionDef(Box::new(parse_emotion_inner(pair)))  
        },
        unknown_def => panic!("Unexpected definition inner: {:?}", unknown_def),
    }
}