//! Generate f32 and f64 floating-point numbers pseudorandomly in various ways.
//! The values may be evenly or unevenly distributed, depending on the chosen method.
//! The crate should be considered mostly experimental.
//! 
//! `rand_xoshiro` is re-exported for convenience.
//! See
//! [https://docs.rs/rand_xoshiro/0.6.0/rand_xoshiro/index.html](https://docs.rs/rand_xoshiro/0.6.0/rand_xoshiro/index.html).

mod float;
pub mod utilities;

pub use crate::float::*;

#[doc(no_inline)]
pub use rand_xoshiro::*;
