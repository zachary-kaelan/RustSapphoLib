use crate::comps::{EmotionDef, VerbDef};
use crate::managers::Manager;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static EMOTION_DEFS: Lazy<HashMap<String, EmotionDef>> =
    Lazy::new(HashMap::new);
static VERB_DEFS: Lazy<HashMap<String, VerbDef>> = Lazy::new(HashMap::new);

impl Manager {
    pub fn get_emotion_def(id: &String) -> &EmotionDef {
        EMOTION_DEFS.get(id).unwrap()
    }

    pub fn get_verb_def(id: &String) -> &VerbDef {
        VERB_DEFS.get(id).unwrap()
    }
}
