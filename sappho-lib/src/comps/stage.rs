use std::collections::HashMap;
use std::ops::{Deref, Index};
use std::rc::Rc;
use crate::comps::Actor;
use serde::{self, Deserialize, Serialize};

/// A location for Actors to interact.
#[derive(Deserialize, Serialize)]
pub struct Stage {
    pub id: String,
    pub display_name: String,
    actor_names: Vec<String>,
    #[serde(skip)]
    actors: Vec<Rc<Actor>>
}

impl Stage {
    pub fn new(id: String, display_name: String, actors: Option<Vec<Rc<Actor>>>) -> Self {
        let (actor_names, actors) = match actors {
            None => { (Vec::new(), Vec::new()) }
            Some(a) => { (a.iter().map(|x| x.as_ref().id.clone()).collect::<Vec<_>>(), a) }
        };
        Self { id, display_name, actor_names, actors }
    }

    /// Add an actor to the stage.
    pub fn add_actor(&mut self, actor: Rc<Actor>) {
        self.actor_names.push(actor.as_ref().id.clone());
        self.actors.push(actor);
    }

    /// Move an actor to another stage.
    pub fn move_actor_to(&mut self, actor_id: &String, other: &mut Self) {
        let index = self.actor_names.iter().position(|x| x.eq(actor_id)).unwrap();
        self.actor_names.remove(index);
        other.add_actor(self.actors.remove(index))
    }

    /// Move all actors to another stage.
    pub fn move_all_actors_to(&mut self, other: &mut Self) {
        for _ in 0..self.actor_names.len() {
            other.add_actor(self.actors.pop().unwrap())
        }
        self.actor_names.clear()
    }

    /// Whether an actor is on a stage.
    pub fn on_stage(&self, actor_id: &String) -> bool {
        todo!()
    }
}