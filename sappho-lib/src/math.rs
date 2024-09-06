pub mod bnum;

// at z/3,  val = 0.25
// at 1z,   val = 0.5
// at 3z,   val = 0.75
// at 9z,   val = 0.9
// at 19z,  val = 0.95
// at 99z,  val = 0.99
const B_MULTI: f32 = 1.0;