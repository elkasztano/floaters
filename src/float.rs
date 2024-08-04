use rand::Rng;

/// A trait containing methods that generate
/// floating point numbers out of pseudorandom `u64` integers in
/// several experimental ways.
pub trait NonCanonical {
    /// Generates an `f64`. The maximum is 0.9999999999999999.
    /// The minimum is 0.000030517578125, and therefore much closer
    /// to zero than with the standard method. However, it comes at the cost of not
    /// getting equidistributed values. The orders of magnitude are roughly
    /// equidistributed.   
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256PlusPlus;
    /// use floaters::NonCanonical;
    ///
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(78787878);
    /// (0..125).for_each(|_| { rng.next_u64(); } );
    /// assert_eq!(0.0008912212147751437, rng.noncanonical_f64());
    /// ```
    fn noncanonical_f64(&mut self) -> f64;
 
    /// Generate a signed `f64` roughly equidistributed in the range
    /// `-1.0` - `1.0`.
    /// The eleventh least significant bit of the PRNG's output
    /// is used as sign bit.
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256PlusPlus;
    /// use floaters::NonCanonical;
    ///
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(1234);
    /// (0..10).for_each(|_| { rng.next_u64(); } );
    /// assert_eq!(-0.8926449323284786, rng.signed_uniform());
    /// ```
    fn signed_uniform(&mut self) -> f64;

    /// Generates an `f64` with a specified exponent. If `Sign` is the `Signed` variant,
    /// you will also get signed numbers. The mantissa will always be pseudorandomly 
    /// generated.
    /// Only the lowest 11 bits of the specified `u16` will be used as the exponent, due to
    /// the specifications of the `f64` type.
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256StarStar;
    /// use floaters::{NonCanonical, Sign};
    ///
    /// let mut rng = Xoshiro256StarStar::seed_from_u64(7878);
    /// (0..42).for_each(|_| { rng.next_u64(); } );
    /// let exp = 0b100_0000_0001;
    /// let sign = Sign::Signed;
    /// assert_eq!(-4.569802780283289, rng.exp_f64(exp, sign));
    /// ```
    fn exp_f64(&mut self, exponent: u16, signed: Sign) -> f64;

    /// Generate an `f64` by specifying parameters.
    /// Reasonable values for `left_shift` are in the range
    /// `53..=61`. The higher the value the closer the number gets
    /// to zero. The `left_shift` parameter is automatically kept
    /// within that range, i.e. it saturates at 53 and 61 in order to
    /// prevent overflow errors and/or unexpected behaviour.
    /// If `Sign` is the `Signed` variant, you will also get signed 
    /// numbers.
    /// The numbers closest to zero in the given example are
    /// 0.0078125 and -0.0078125. The numbers farthest from zero
    /// are 0.9999999999999999 and -0.9999999999999999.
    /// The numbers are not evenly distributed.
    /// 
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256StarStar;
    /// use floaters::{NonCanonical, Sign};
    ///
    /// let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    /// (0..7878).for_each(|_| { rng.next_u64(); } );
    /// assert_eq!(-0.05579455843802982, rng.with_params_f64(55, Sign::Signed));
    /// ```
    fn with_params_f64(&mut self, left_shift: i8, signed: Sign) -> f64;

    /// Generates an `f32` tuple. Values are not evenly distributed, but get
    /// close to zero. The minimum is 0.0078125, the maximum is 0.99999994.
    ///
    /// The first element of the tuple is generated from the
    /// lowest 32 bits, and the second element is generated using the
    /// highest 32 bits of the underlying `u64`.
    /// Keep in mind that the lowest bits might be of low linear complexity,
    /// depending on the chosen (P)RNG.
    ///
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256StarStar;
    /// use floaters::NonCanonical;
    ///
    /// let mut rng = Xoshiro256StarStar::seed_from_u64(43214321);
    /// (0..987).for_each(|_| { rng.next_u64(); } );
    /// assert_eq!( (0.009435893, 0.24692793),
    ///     rng.noncanonical_tuple_f32() );
    /// ```
    fn noncanonical_tuple_f32(&mut self) -> (f32, f32);

    /// Generates a signed `f32` tuple. The `.0` element of
    /// the tuple might be of low linear complexity, depending
    /// on the chosen (P)RNG.
    fn signed_tuple_f32(&mut self) -> (f32, f32);
    
    /// Generates an `f32` tuple with specified parameters.
    /// Values are likely not evenly distributed.
    /// The `left_shift` parameter saturates at 21 and 29.
    /// Creates signed values if `Sign` is the `Signed` variant.
    /// The `.0` element of the tuple might be of low linear
    /// complexity, depending on the chosen (P)RNG.
    ///
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256PlusPlus;
    /// use floaters::{NonCanonical, Sign};
    /// 
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    /// (0..1234).for_each(|_| { rng.next_u64(); } );
    /// let tuple_f32 = rng.with_params_tuple_f32(26, Sign::Signed);
    ///
    /// assert_eq!( (-0.0095176855, 0.24847426), tuple_f32 );
    /// ```
    fn with_params_tuple_f32(&mut self, left_shift: i8, signed: Sign) -> (f32, f32);

    /// Generates an `f32` tuple with a specified exponent.
    /// Creates signed values if `Sign` is the `Signed` variant.
    /// The `.0` element of the tuple might be of low linear
    /// complexity, depending on the chosen (P)RNG.
    ///
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro256PlusPlus;
    /// use floaters::{NonCanonical, Sign};
    ///
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    /// (0..1234).for_each(|_| { rng.next_u64(); } );
    /// let tuple_f32 = rng.exp_f32(0b1000_0001u8, Sign::Signed);
    ///
    /// assert_eq!((-4.873055f32, 7.951176f32), tuple_f32);
    /// ```
    fn exp_f32(&mut self, exp: u8, signed: Sign) -> (f32, f32);
   
    /// Generates an `f64` out of 64 pseudorandom bits including
    /// negative zero, infinity, negative infinity and Nan.
    ///
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro512StarStar;
    /// use floaters::NonCanonical;
    ///
    /// let mut rng = Xoshiro512StarStar::seed_from_u64(1234);
    /// (0..123).for_each(|_| { rng.next_u64(); } );
    /// let wild_f64 = rng.wild_f64();
    ///
    /// assert_eq!(2.640929385653772e-105, wild_f64);
    /// ```
    fn wild_f64(&mut self) -> f64;

    /// Generates an `f32` tuple out of 64 pseudorandom bits including
    /// negative zero, infinity, negative infinity and Nan.
    /// Keep in mind that the lower bits of the underlying `u64`
    /// might be of low linear complexity, so you might prefer the
    /// second element of the tuple.
    ///
    /// # Example
    /// ```
    /// use rand_core::{RngCore, SeedableRng};
    /// use rand_xoshiro::Xoshiro512StarStar;
    /// use floaters::NonCanonical;
    ///
    /// let mut rng = Xoshiro512StarStar::seed_from_u64(123);
    /// (0..123).for_each(|_| { rng.next_u64(); } );
    /// let wild_f32 = rng.wild_tuple_f32();
    ///
    /// assert_eq!((-6.6361835e20, -1.9641858), wild_f32);
    /// ```
    fn wild_tuple_f32(&mut self) -> (f32, f32);
}

impl<T: Rng> NonCanonical for T {
   
    fn noncanonical_f64(&mut self) -> f64 {
        let mut x = self.next_u64();
        x |= u64::MAX << (56 + 2) >> 2;
        x &= !(3u64 << 62 | 1u64 << 52);
        f64::from_bits(x)
    }

    fn exp_f64(&mut self, exponent: u16, signed: Sign) -> f64 {
        let mut x = self.next_u64();
        let exp = (exponent << 5 >> 5) as u64;
        if signed == Sign::Signed
        { x &= !(2047u64 << 52); } else { x &= !(4095u64 << 52); }
        x |= exp << 52;
        f64::from_bits(x)
    }

    fn signed_uniform(&mut self) -> f64 {
        let x = self.next_u64();
        let sign_bit = x >> 10 << 63;
        let output = (x >> 11) as f64 * 1.110223e-16;
        f64::from_bits(output.to_bits() | sign_bit)
    }

    fn with_params_f64(&mut self, left_shift: i8, signed: Sign) -> f64 {
        let left_shift_sat = if left_shift < 53 { 53 }
            else if left_shift > 61 { 61 }
            else { left_shift };
        let mut x = self.next_u64();
        let sign_mask = if signed == Sign::Signed { 1u64 } else { 3u64 };
        let ls = left_shift_sat as usize;
        x |= u64::MAX << (ls + 2) >> 2;
        x &= !(sign_mask << 62 | 1u64 << 52);
        f64::from_bits(x)
    }

    fn noncanonical_tuple_f32(&mut self) -> (f32, f32) {
        let x = self.next_u64();
        let (mut le, mut be) = u32_from_u64(x);
        ( f32_from_u32(&mut le, 26, Sign::Unsigned),
        f32_from_u32(&mut be, 26, Sign::Unsigned) )
    }
    
    fn signed_tuple_f32(&mut self) -> (f32, f32) {
        let x = self.next_u64();
        let (le, be) = u32_from_u64(x);
        ( f32_with_sign(le), f32_with_sign(be) )
    }

    fn exp_f32(&mut self, exp: u8, signed: Sign) -> (f32, f32) {
        let x = self.next_u64();
        let (mut le, mut be) = u32_from_u64(x);
        ( specified_exp_f32(&mut le, exp, signed),
        specified_exp_f32(&mut be, exp, signed) )
    }

    fn with_params_tuple_f32(&mut self, left_shift: i8, signed: Sign) -> (f32, f32) {
        let left_shift_sat = if left_shift < 21 { 21 }
            else if left_shift > 29 { 29 }
            else { left_shift };
        let x = self.next_u64();
        let (mut le, mut be) = u32_from_u64(x);
        ( f32_from_u32(&mut le, left_shift_sat, signed),
        f32_from_u32(&mut be, left_shift_sat, signed) )
    }

    fn wild_f64(&mut self) -> f64 {
        let x = self.next_u64();
        f64::from_bits(x)
    }

    fn wild_tuple_f32(&mut self) -> (f32, f32) {
        let x = self.next_u64();
        let (le, be) = u32_from_u64(x);
        ( f32::from_bits(le),
        f32::from_bits(be) )
    }

}

/// Return either signed or unsigned values.
#[derive(PartialEq, Copy, Clone)]
pub enum Sign {
    Signed,
    Unsigned
}

// f32 helper functions

fn u32_from_u64(bits: u64) -> (u32, u32) {
    ( (bits << 32 >> 32) as u32,
    (bits >> 32) as u32 )
}

// reasonable values for left_shift: 26, 21..=29
fn f32_from_u32(bits: &mut u32, left_shift: i8, signed: Sign) -> f32 {
    let sign_mask = if signed == Sign::Signed { 1u32 } else { 3u32 };
    *bits |= u32::MAX << (left_shift + 2) >> 2;
    *bits &= !(sign_mask << 30 | 1u32 << 23);
    f32::from_bits(*bits)
}

fn f32_with_sign(bits: u32) -> f32 {
    let sign_bit = bits >> 8 << 31;
    let output = (bits >> 9) as f32 * 1.192093e-07;
    f32::from_bits(output.to_bits() | sign_bit)
}

fn specified_exp_f32(bits: &mut u32, exponent: u8, signed: Sign) -> f32 {
    if signed == Sign::Signed
        { *bits &= !(255u32 << 23); } 
    else
        { *bits &= !(511u32 << 23); }
    *bits |= (exponent as u32) << 23;
    f32::from_bits(*bits)
}
