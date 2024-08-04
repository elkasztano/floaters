//! Generate f32 and f64 floating-point numbers pseudorandomly in various
//! mostly experimental ways. Use some pseudorandom bits for the sign bit
//! and/or for exponent bits as well.
//! That way, the number of wasted precious pseudorandom bits is reduced.
//! The values may be (roughly) evenly or unevenly distributed,
//! depending on the chosen method.

mod float;
pub mod utilities;

pub use crate::float::*;
