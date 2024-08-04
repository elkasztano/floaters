//! Contains helper functions that simulate the output or show
//! minimum and maximum values of some methods in the `NonCanonical`
//! trait.
//! The functions may help you to find the right parameters for
//! various methods.

/// Get unsigned minimum and maximum values of a given exponent.
/// The exponent is ideally specified in binary format.
/// The exponent of an `f64` is 11 bits wide.
/// 
/// # Example
/// ```
/// use floaters::utilities::exponent_bounds_f64;
/// let exp: u16 = 0b100_0000_0001;
/// let (lower, upper) = exponent_bounds_f64(exp);
/// 
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
/// 
/// # Example
/// ```
/// use floaters::utilities::exponent_bounds_f32;
/// let exp: u8 = 0b1000_0001;
/// let (lower, upper) = exponent_bounds_f32(exp);
/// 
/// assert_eq!(
///     (4.0, 7.9999995),
///     (lower, upper)
/// );
/// ```
pub fn exponent_bounds_f32(exp: u8) -> (f32, f32) {
    ( f32::from_bits(0u32 | (exp as u32) << 23),
      f32::from_bits(!(1u32 << 31 | 255u32 << 23) | (exp as u32) << 23 ) )
}

/// Get unsigned minimum value of a given `left_shift`
/// parameter for the `with_params_f64` method.
/// The maximum will always be the `f64` closest to one.
/// Will return `None` if `left_shift` is outside the range
/// `21..=29`.
/// 
/// # Example
/// ```
/// use floaters::utilities::params_min_f64;
/// let left_shift = 56i8;
/// 
/// assert_eq!(
///     params_min_f64(left_shift),
///     Some(0.000030517578125)
/// );
/// ```
pub fn params_min_f64(left_shift: i8) -> Option<f64> {

    shift_unsigned_f64(0u64, left_shift)

}

/// Get unsigned minimum value of a given `left_shift`
/// parameter for the `with_params_tuple_f32` method.
/// The maximum will always be the `f32` closest to one.
/// Will return `None` if `left_shift` is outside the range
/// `21..=29`.
///
/// # Example
/// ```
/// use floaters::utilities::params_min_f32;
/// let left_shift = 26i8;
/// 
/// assert_eq!(
///     params_min_f32(left_shift),
///     Some(0.0078125)
/// );
/// ```
pub fn params_min_f32(left_shift: i8) -> Option<f32> {

    shift_unsigned_f32(0u32, left_shift)

}

/// Simulate the `with_params_f64` method by specifying both
/// the `left_shift` parameter and the underlying `u64`.
/// Will return `None` if `left_shift` is outside the range
/// `53..=61`.
///
/// # Example
/// ```
/// use floaters::utilities::simulate_params_f64;
/// let x = 12345u64;
/// let left_shift = 57i8;
/// 
/// assert_eq!(
///     simulate_params_f64(x, left_shift),
///     Some(0.0000000004656612873090157)
/// );
///
/// assert_eq!(
///     simulate_params_f64(123, 123),
///     None
/// );
/// ```
pub fn simulate_params_f64(x: u64, left_shift: i8) -> Option<f64> {

    shift_unsigned_f64(x, left_shift)

}

/// Partially simulate the `with_params_tuple_f32` method by
/// specifying the `left_shift` parameter and an underlying
/// `u32`.
/// Will return `None` if `left_shift` is outside the range
/// `21..=29`.
///
/// # Example
/// ```
/// use floaters::utilities::simulate_params_f32;
/// let x = 54321u32;
/// let left_shift = 23i8;
/// 
/// assert_eq!(
///     simulate_params_f32(x, left_shift),
///     Some(0.5032378)
/// );
///
/// assert_eq!(
///     simulate_params_f32(123, -123),
///     None
/// );
/// ```
pub fn simulate_params_f32(x: u32, left_shift: i8) -> Option<f32> {

    shift_unsigned_f32(x, left_shift)

}

// conversion helper functions

fn shift_unsigned_f64(x: u64, left_shift: i8) -> Option<f64> {

    if left_shift < 53 || left_shift > 61 {

        None

    } else {

        let ls = left_shift as u64;
        let mut r = x | ( u64::MAX << (ls + 2) >> 2 );
        r &= !(3u64 << 62 | 1u64 << 52);
        Some( f64::from_bits(r) )

    }

}

fn shift_unsigned_f32(x: u32, left_shift: i8) -> Option<f32> {
    
    if left_shift < 21 || left_shift > 29 {

        None

    } else {

        let ls = left_shift as u32;
        let mut r = x | ( u32::MAX << (ls + 2) >> 2 );
        r &= !(3u32 << 30 | 1u32 << 23);
        Some( f32::from_bits(r) )

    }

}
