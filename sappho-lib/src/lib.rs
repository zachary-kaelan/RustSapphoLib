mod models;
mod comps;
mod consts;
mod value_aliases;
mod managers;

pub use crate::models::{BNumber, BnumGroup, SparseBNumber, SparseBnumGroup};
pub use crate::managers::Manager;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
