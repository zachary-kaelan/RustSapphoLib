use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use once_cell::sync::{Lazy, OnceCell};
use crate::comps::Stage;
use crate::Manager;

static STAGES: Lazy<Mutex<HashMap<String, Arc<Stage>>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });

impl Manager {
    pub fn get_stage(id: &String) -> std::sync::Weak<Stage> {
        let stages = STAGES.lock().unwrap();
        Arc::downgrade(&Arc::clone(stages.get(id).unwrap()))
    }

    pub fn init_stages(stages: Vec<Stage>) {
        // Once in the arcs the stages can't be mutated
        let mut stages_map = STAGES.lock().unwrap();
        for mut stage in stages {
            stage.init_stage();
            stages_map.insert(stage.display_name.clone(), Arc::new(stage));
        }
    }
}