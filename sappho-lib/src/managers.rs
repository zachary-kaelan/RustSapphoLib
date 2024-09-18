mod actor_manager;
mod def_managers;
mod stage_manager;

pub struct Manager;

use crate::comps::EmotionDef;
use crate::consts::BNUM_GROUP_SIZE;
use crate::scripting::*;
use crate::value_aliases::{AliasType, Aliases};
use crate::{SparseBNumber, SparseBnumGroup};
use std::collections::HashMap;
use std::ops::Deref;
use std::str;

struct SparseActor {
    pub personality: SparseBnumGroup,
    pub accordance: SparseBnumGroup,
    pub self_perceptions: SparseBnumGroup,
    pub perceptions: HashMap<String, SparseBnumGroup>,
}

impl SparseActor {
    fn new() -> Self {
        Self {
            personality: SparseBnumGroup::new(None),
            accordance: SparseBnumGroup::new(None),
            self_perceptions: SparseBnumGroup::new(None),
            perceptions: HashMap::new(),
        }
    }

    fn assign(&mut self, node: &AstNode) {
        match node {
            AstNode::BnumTargetAssign {
                bnum_type,
                ident,
                target,
                expr,
            } => {
                let f = match expr.as_ref() {
                    AstNode::Float(f) => Some(*f),
                    AstNode::FloatOptional(f) => *f,
                    _ => {
                        unreachable!()
                    }
                };

                let bnum_group = match bnum_type {
                    BnumType::Accordance => &mut self.accordance,
                    BnumType::SelfPerception => &mut self.self_perceptions,
                    BnumType::Personality => &mut self.personality,
                    BnumType::Perception => {
                        let target =
                            target.get(0).expect("No target specified");
                        self.perceptions.entry(target.clone()).or_insert(
                            SparseBnumGroup::from([None; BNUM_GROUP_SIZE]),
                        )
                    }
                    BnumType::Circumstantial => {
                        todo!()
                    }
                };

                match Aliases::get_index_for_bnum_alias(bnum_type, ident) {
                    None => {
                        panic!("Alias {} not found", ident)
                    }
                    Some(i) => {
                        bnum_group.give_values();
                        bnum_group.values.unwrap()[i] = SparseBNumber::new(f);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Manager {
    pub fn init(scripts_folder: &str) {
        let dir = std::fs::read_dir(scripts_folder);
        let dir = match dir {
            Ok(d) => d,
            Err(e) => panic!("{}", e),
        };
        let mut actor_display_names: HashMap<String, String> = HashMap::new();
        let mut actor_defs: HashMap<String, SparseActor> = HashMap::new();
        let mut assignments: Vec<AstNode> = vec![];
        let mut emotion_defs: Vec<EmotionDef> = vec![];

        for file in dir {
            let file = match file {
                Ok(f) => f,
                Err(_) => continue,
            };

            match file.file_type() {
                Ok(file_type) => {
                    if !file_type.is_file() {
                        continue;
                    }
                }
                Err(_) => continue,
            }

            if !file.file_name().to_str().unwrap().ends_with(".sappho") {
                continue;
            }

            let file_text =
                std::fs::read_to_string(file.path()).expect("Cannot read file");
            let ast_nodes = parse(&file_text).expect("Unsuccessful parse");

            for node in ast_nodes {
                match node {
                    AstNode::Def {
                        ident,
                        display_name,
                        definition,
                    } => match definition.deref() {
                        DefInnerAstNode::ActorDef(def_ast) => {
                            actor_display_names.insert(
                                ident.clone(),
                                display_name.into_string().unwrap(),
                            );
                            let mut sparse_actor = SparseActor::new();

                            for def_ast_node in def_ast {
                                match def_ast_node {
                                    ActorAstNode::ActorGroup {
                                        bnum_type,
                                        target,
                                        group,
                                    } => {
                                        match bnum_type {
                                            BnumType::Accordance => {
                                                sparse_actor.accordance = extract_bnum_group_or_tuple(
                                                    &Some(group.clone()),
                                                    AliasType::Accordance,
                                                )
                                                    .expect("Accordance failed");
                                            }
                                            BnumType::SelfPerception => {
                                                sparse_actor.self_perceptions =
                                                    extract_bnum_group_or_tuple(
                                                        &Some(group.clone()),
                                                        AliasType::Perception,
                                                    )
                                                    .expect("Perception failed");
                                            }
                                            BnumType::Personality => {
                                                sparse_actor.personality = extract_bnum_group_or_tuple(
                                                    &Some(group.clone()),
                                                    AliasType::Personality,
                                                )
                                                    .expect("Personality failed");
                                            }
                                            BnumType::Perception => {
                                                sparse_actor.perceptions.insert(
                                                    target.get(0).unwrap().clone(),
                                                    extract_bnum_group_or_tuple(
                                                        &Some(group.clone()),
                                                        AliasType::Perception,
                                                    )
                                                        .expect("Perception failed"),
                                                );
                                            }
                                            BnumType::Circumstantial => {
                                                todo!()
                                            }
                                        }
                                    }
                                    ActorAstNode::BnumTargetAssign(assign) => {
                                        sparse_actor.assign(assign.as_ref())
                                    }
                                }
                            }

                            actor_defs.insert(ident, sparse_actor);
                        }
                        DefInnerAstNode::EmotionDef(def_ast) => {
                            let def_ast = def_ast.as_ref();
                            let def = EmotionDef::new(
                                ident.clone(),
                                display_name.into_string().unwrap(),
                                extract_bnum_group_or_tuple(
                                    &def_ast.personality_modifiers,
                                    AliasType::Personality,
                                )
                                .expect(&ident[..]),
                                extract_weights_group_or_tuple(
                                    &def_ast.personality_weights,
                                    AliasType::Personality,
                                )
                                .expect(&ident[..]),
                                extract_bnum_group_or_tuple(
                                    &def_ast.perception_modifiers,
                                    AliasType::Perception,
                                )
                                .expect(&ident[..]),
                                extract_weights_group_or_tuple(
                                    &def_ast.perception_weights,
                                    AliasType::Perception,
                                )
                                .expect(&ident[..]),
                            );
                            emotion_defs.push(def);
                        }
                    },
                    AstNode::BnumTargetAssign { .. } => assignments.push(node),
                    _ => panic!("Unexpected top level node {:?}", node),
                }
            }
        }

        let buddy = actor_defs.get("buddy").unwrap();
        println!("PERSONALITY: {}", buddy.personality);
        if !buddy.perceptions.is_empty() {
            println!("PERCEPTIONS");
            for (ident, perception) in &buddy.perceptions {
                println!("{}: {}", ident, perception);
            }
        }

        println!("Scripts loaded");
    }
}

#[cfg(test)]
mod tests {
    use crate::Manager;
    use std;

    #[test]
    fn parse_test() {
        Manager::init("src/scripting")
    }
}
