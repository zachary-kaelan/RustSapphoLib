#[derive(Debug)]
pub enum Error {
    InsufficientTupleArgs(usize),
    UnrecognizedBNumberAlias(String),
}
