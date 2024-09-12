use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::comps::Stage;
use crate::Manager;

static STAGES: Lazy<RwLock<HashMap<String, Arc<Stage>>>> = Lazy::new(|| { RwLock::new(HashMap::new()) });

impl Manager {
    pub fn get_stage(id: &String) -> std::sync::Weak<Stage> {
        let stages = STAGES.read().unwrap();
        Arc::downgrade(&Arc::clone(stages.get(id).unwrap()))
    }

    pub fn init_stages(stages: Vec<Stage>) {
        // Once in the arcs the stages can't be mutated
        let mut stages_map = STAGES.write().unwrap();
        for mut stage in stages {
            stage.init_stage();
            stages_map.insert(stage.display_name.clone(), Arc::new(stage));
        }
    }
}