use std::collections::HashMap;
use crate::comps::actor::EmotionDef;


pub struct DefDatabase {
    pub emotion_defs: HashMap<String, EmotionDef>
}