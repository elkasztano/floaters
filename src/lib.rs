//! Generate f32 and f64 floating-point numbers pseudorandomly in various ways.
//! The values may be evenly or unevenly distributed, depending on the chosen method.
//! The crate should be considered mostly experimental.
//! The used generators are _not_ cryptographically secure.

use getrandom::*;

pub mod generators;

pub(crate) mod float;

/// Argument for functions that return either signed or unsigned values.
/// ## Example
/// ```rust
/// use floaters::Sign;
/// use floaters::generators::Xoroshiro256pp;
///
/// let mut xrsr256pp = Xoroshiro256pp::new();
/// xrsr256pp.init(7238);
/// let my_exp = 0b100_0000_0001u16;
/// let my_f64 = xrsr256pp.exp_f64(my_exp, Sign::Signed);
/// assert_eq!(-5.151647679447546, my_f64);
/// ```
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Sign {
    Signed,
    Unsigned,
}

fn getrandom_u64() -> Result<u64, Box<dyn std::error::Error>> {
    let mut array = [0u8; 8];
    getrandom(&mut array)?;
    let mut output: u64 = 0;
    for i in 0..8 {
        output += (array[i] as u64) << (i * 8);
    }
    Ok(output)
}

fn getrandom_nonzero64vec(n: usize) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut output = Vec::<u64>::with_capacity(n);
    'outer: loop {
        for _ in 0..n { output.push(getrandom_u64()?); }
        for i in 0..n { if output[i] != 0u64 { break 'outer; } }
    }
    Ok(output)
}

pub mod utilities {
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

}
