use std::collections::HashMap;
use crate::comps::EmotionDef;
use crate::managers::Manager;
use once_cell::sync::Lazy;

static EMOTION_DEFS: Lazy<HashMap<String, EmotionDef>> = Lazy::new(HashMap::new);


impl Manager {
    pub fn get_emotion_def(id: &String) -> &EmotionDef {
        EMOTION_DEFS.get(id).unwrap()
    }
}