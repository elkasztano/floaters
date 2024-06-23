//! Contains helper functions that return the minimum and maximum of a given exponent.
//! The resulting numbers are always unsigned.
//! The mantissa of the minimum is just zero, and the mantissa of the maximum is filled up
//! with binary ones.
  
/// Get unsigned minimum and maximum values of a given exponent.
/// The exponent is ideally specified in binary format.
/// The exponent of an `f64` is 11 bits wide.
/// # Examples
/// ```rust
/// use floaters::utilities::exponent_bounds_f64;
/// let exp: u16 = 0b100_0000_0001;
/// let (lower, upper) = exponent_bounds_f64(exp);
/// assert_eq!(
///     (4.0, 7.999999999999999),
///     (lower, upper)
/// );
/// ```
pub fn exponent_bounds_f64(exponent: u16) -> (f64, f64) {
    let exp = (exponent << 5 >> 5) as u64;
    ( f64::from_bits(0u64 | exp << 52),
      f64::from_bits(!(1u64 << 63 | 2047u64 << 52) | exp << 52 ) )
}

/// Get unsigned minimum and maximum values of a given exponent.
/// The exponent is ideally specified in binary format.
/// The exponent of an `f32` is 8 bits wide.
/// # Examples
/// ```rust
/// use floaters::utilities::exponent_bounds_f32;
/// let exp: u8 = 0b1000_0001;
/// let (lower, upper) = exponent_bounds_f32(exp);
/// assert_eq!(
///     (4.0, 7.9999995),
///     (lower, upper)
/// );
/// ```
pub fn exponent_bounds_f32(exp: u8) -> (f32, f32) {
    ( f32::from_bits(0u32 | (exp as u32) << 23),
      f32::from_bits(!(1u32 << 31 | 255u32 << 23) | (exp as u32) << 23 ) )
}
