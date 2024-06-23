//! Generate f32 and f64 floating-point numbers pseudorandomly in various ways.
//! The values may be evenly or unevenly distributed, depending on the chosen method.
//! Additionally, you can generate a seed for the PRNGs from the `rand_xoshiro`
//! crate simply by specifying a `&str`.
//! The crate should be considered mostly experimental.

mod float;
pub mod utilities;

pub use crate::float::*;

#[doc(no_inline)]
/// Re-export for convenience.
pub use rand_xoshiro::*;
