use super::Actor;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.id,
            self.actor_state.read().unwrap().personality
        )
    }
}
