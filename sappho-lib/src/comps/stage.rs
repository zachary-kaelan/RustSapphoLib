use std::collections::HashMap;
use std::rc::Rc;
use crate::comps::Actor;

/// A location for Actors to interact.
pub struct Stage {
    pub id: String,
    pub display_name: String,
    actors: HashMap<String, Rc<Actor>>
}

impl Stage {    
    pub fn new(id: String, display_name: String, actors: Option<HashMap<String, Rc<Actor>>>) -> Self {
        Self { id, display_name, actors: actors.unwrap_or(HashMap::new()) }
    }

    /// Add an actor to the stage.
    pub fn add_actor(&mut self, actor: Rc<Actor>) {
        self.actors.insert(actor.as_ref().id.to_string(), actor);
    }

    /// Move an actor to another stage.
    pub fn move_actor_to(&mut self, actor_id: &String, other: &mut Self) {
        other.add_actor(self.actors.remove(actor_id).unwrap())
    }
    
    /// Move all actors to another stage.
    pub fn move_all_actors_to(&mut self, other: &mut Self) {
        for actor_id in self.actors.keys() {
            self.move_actor_to(actor_id, other)
        }
    }
    
    /// Whether an actor is on a stage.
    pub fn on_stage(&self, actor_id: &String) -> bool {
        self.actors.contains_key(actor_id)
    }
}