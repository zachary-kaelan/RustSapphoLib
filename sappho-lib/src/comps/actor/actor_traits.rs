use std::fmt;
use std::fmt::Formatter;
use super::Actor;

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.actor_state.lock().unwrap().personality)
    }
}