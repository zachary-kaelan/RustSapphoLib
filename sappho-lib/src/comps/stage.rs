use crate::comps::Actor;
use crate::Manager;
use serde::{self, Deserialize, Serialize};
use std::sync::{Arc, RwLock, Weak};

/// A location for Actors to interact.
#[derive(Deserialize, Serialize)]
pub struct Stage {
    pub id: String,
    pub display_name: String,
    actor_names: RwLock<Vec<String>>,
    #[serde(skip)]
    actors: RwLock<Vec<Weak<Actor>>>,
    #[serde(skip)]
    initialized: bool,
}

impl Stage {
    pub fn new(
        id: String,
        display_name: String,
        actors: Option<Vec<Weak<Actor>>>,
    ) -> Self {
        let (actor_names, actors) = match actors {
            None => (Vec::new(), Vec::new()),
            Some(a) => {
                let actors: Vec<Weak<Actor>> = a
                    .iter()
                    .filter_map(|a2| a2.upgrade())
                    .map(|a3| Arc::downgrade(&a3))
                    .collect::<Vec<Weak<Actor>>>();
                let actor_names = actors
                    .iter()
                    .map(|a2| a2.upgrade().unwrap().display_name.clone())
                    .collect::<Vec<String>>();
                (actor_names, actors)
            }
        };
        Self {
            id,
            display_name,
            actor_names: RwLock::new(actor_names),
            actors: RwLock::new(actors),
            initialized: true,
        }
    }

    /// Add an actor to the stage.
    pub fn add_actor(&self, actor: &Arc<Actor>) {
        self.actor_names.write().unwrap().push(actor.id.clone());
        self.actors.write().unwrap().push(Arc::downgrade(actor));
    }

    /// Move an actor to another stage.
    pub fn move_actor_to(&self, actor_id: &String, other: &Self) {
        let mut actor_names = self.actor_names.write().unwrap();
        let index = actor_names.iter().position(|x| x.eq(actor_id)).unwrap();
        actor_names.remove(index);
        other.add_actor(
            &self
                .actors
                .write()
                .unwrap()
                .remove(index)
                .upgrade()
                .unwrap(),
        )
    }

    /// Move all actors to another stage.
    pub fn move_all_actors_to(&mut self, other: &mut Self) {
        let mut actor_names = self.actor_names.write().unwrap();
        let mut actors = self.actors.write().unwrap();
        for _ in 0..actor_names.len() {
            other.add_actor(&actors.pop().unwrap().upgrade().unwrap())
        }
        actor_names.clear()
    }

    /// Whether an actor is on a stage.
    pub fn on_stage(&self, actor_id: &String) -> bool {
        self.actor_names.read().unwrap().contains(actor_id)
    }

    pub fn init_stage(&mut self) -> bool {
        if self.initialized {
            return false;
        }

        // Collect the actors to be added first
        let actors_to_add: Vec<_> = self
            .actor_names
            .read()
            .unwrap()
            .iter()
            .map(|actor_name| Manager::get_actor(actor_name).upgrade().unwrap())
            .collect();

        // Now we can mutate self
        for actor in actors_to_add {
            self.add_actor(&actor);
        }

        self.initialized = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::comps::{Actor, Stage};
    use crate::Manager;
    use std::sync::{Arc, RwLock};

    #[test]
    fn stage_init() {
        let actor = Arc::new(Actor::default(
            String::from("actor1"),
            String::from("Actor 1"),
        ));
        Manager::add_actor(&Arc::clone(&actor));
        let mut stage1 = Stage {
            id: String::from("stage1"),
            display_name: String::from("Stage 1"),
            actor_names: RwLock::new(vec![String::from("actor1")]),
            actors: RwLock::new(Vec::new()),
            initialized: false,
        };
        stage1.init_stage();
        assert!(stage1.on_stage(&actor.id));
    }

    #[test]
    fn stage_move() {
        let stage1 =
            Stage::new(String::from("stage1"), String::from("Stage 1"), None);
        let stage2 =
            Stage::new(String::from("stage2"), String::from("Stage 2"), None);
        let actor = Arc::new(Actor::default(
            String::from("actor1"),
            String::from("Actor 1"),
        ));
        stage1.add_actor(&Arc::clone(&actor));
        assert!(stage1.on_stage(&actor.id));
        assert!(!stage2.on_stage(&actor.id));
        stage1.move_actor_to(&actor.id, &stage2);
        assert!(!stage1.on_stage(&actor.id));
        assert!(stage2.on_stage(&actor.id));
    }
}
