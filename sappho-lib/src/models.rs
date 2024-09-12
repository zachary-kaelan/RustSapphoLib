mod bnum;
mod bnum_grp;
mod macros;
mod sparse_bnum_grp;
mod sparse_bnum;

pub use bnum::BNumber;
pub use bnum_grp::BnumGroup;
pub use sparse_bnum::SparseBNumber;
pub use sparse_bnum_grp::SparseBnumGroup;

// at z/3,  val = 0.25
// at 1z,   val = 0.5
// at 3z,   val = 0.75
// at 9z,   val = 0.9
// at 19z,  val = 0.95
// at 99z,  val = 0.99
const B_MULTI: f32 = 1.0;
