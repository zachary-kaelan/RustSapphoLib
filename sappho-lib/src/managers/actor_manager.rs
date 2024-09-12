use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::comps::Actor;
use crate::Manager;

static ACTORS: Lazy<RwLock<HashMap<String, Arc<Actor>>>> = Lazy::new(|| { RwLock::new(HashMap::new()) });

impl Manager {
    pub fn get_actor(id: &String) -> std::sync::Weak<Actor> {
        let actors_map = ACTORS.read().unwrap();
        Arc::downgrade(&Arc::clone(actors_map.get(id).unwrap()))
    }
    
    pub fn add_actor(actor: &Arc<Actor>) {
        let mut actors = ACTORS.write().unwrap();
        actors.insert(actor.id.clone(), Arc::clone(actor));
    }

    pub fn init_actors(actors: Vec<Actor>) {
        // Once in the arcs the actors can't be mutated
        let mut actors_map = ACTORS.write().unwrap();
        for mut actor in actors {
            actor.init_actor();
            actors_map.insert(actor.display_name.clone(), Arc::new(actor));
        }
    }
}