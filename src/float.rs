use rand::{Rng, SeedableRng};

/// Parameter for functions that return either signed or unsigned values.
#[derive(PartialEq, Copy, Clone)]
pub enum Sign {
    Signed,
    Unsigned
}

/// The core element of the crate. Contains methods that generate
/// floating point numbers out of pseudorandom `u64` integers.
pub trait NonCanonical {
    /// Generates an `f64`. The maximum is 0.9999999999999999.
    /// The minimum is 0.000030517578125, and therefore much closer
    /// to zero than with the standard method. However, it comes at the cost of not
    /// getting equidistributed values. The orders of magnitude are roughly
    /// equidistributed.   
    /// # Example
    /// ```rust
    /// use floaters::rand_core::SeedableRng;
    /// use floaters::{Xoshiro256PlusPlus, NonCanonical};
    ///
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(78787878);
    /// rng.idle(125);
    /// assert_eq!(0.0008912212147751437, rng.noncanonical_f64());
    /// ```
    fn noncanonical_f64(&mut self) -> f64;

    /// Advance the pseudorandom number generator `n` steps and ignore
    /// the result.
    fn idle(&mut self, n: usize);
 
    /// Generate a signed `f64` roughly equidistributed in the range
    /// `-1.0` - `1.0`.
    /// The tenth least significant bit of the PRNG's output
    /// is used as sign bit.
    /// # Example
    /// ```rust
    /// use floaters::rand_core::SeedableRng;
    /// use floaters::{Xoshiro256PlusPlus, NonCanonical};
    ///
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(1234);
    /// rng.idle(10);
    /// assert_eq!(-0.8926449323284786, rng.signed_uniform());
    /// ```
    fn signed_uniform(&mut self) -> f64;

    /// Generates an `f64` with a specified exponent. If `Sign` is the `Signed` variant,
    /// you will also get signed numbers. The mantissa will always be pseudorandomly 
    /// generated.
    /// Only the lowest 11 bits of the specified `u16` will be used as the exponent, due to
    /// the specifications of the `f64` type.
    /// # Example
    /// ```rust
    /// use floaters::rand_core::SeedableRng;
    /// use floaters::{Xoshiro256StarStar, NonCanonical, Sign};
    ///
    /// let mut rng = Xoshiro256StarStar::seed_from_u64(7878);
    /// rng.idle(42);
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
    /// ```rust
    /// use floaters::rand_core::SeedableRng;
    /// use floaters::{Xoshiro256StarStar, NonCanonical, Sign};
    ///
    /// let mut rng = Xoshiro256StarStar::seed_from_u64(42);
    /// rng.idle(7878);
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
    /// depending on the chosen PRNG.
    ///
    /// # Example
    /// ```rust
    /// use floaters::rand_core::SeedableRng;
    /// use floaters::{Xoshiro256StarStar, NonCanonical};
    ///
    /// let mut rng = Xoshiro256StarStar::seed_from_u64(43214321);
    /// rng.idle(987);
    /// assert_eq!( (0.009435893, 0.24692793),
    ///     rng.noncanonical_tuple_f32() );
    /// ```
    fn noncanonical_tuple_f32(&mut self) -> (f32, f32);

    /// Generates an signed `f32` tuple.
    fn signed_tuple_f32(&mut self) -> (f32, f32);
    
    /// Generates an `f32` tuple with specified parameters.
    /// Values are not evenly distributed.
    /// The `left_shift` parameter saturates at 21 and 29.
    /// Creates signed values if `signed` is the `Signed` variant.
    ///
    /// # Example
    /// ```rust
    /// use floaters::rand_core::SeedableRng;
    /// use floaters::{Xoshiro256PlusPlus, NonCanonical, Sign};
    /// 
    /// let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    /// rng.idle(1234);
    /// let tuple_f32 = rng.tuple_f32_with_params(26, Sign::Signed);
    ///
    /// assert_eq!( (-0.0095176855, 0.24847426), tuple_f32 );
    /// ```
    fn tuple_f32_with_params(&mut self, left_shift: i8, signed: Sign) -> (f32, f32);
   
    /// Generates an `f64` out of 64 pseudorandom bits including
    /// `-0`, infinity and Nan.
    fn wild_f64(&mut self) -> f64;

    /// Generates an `f32` tuple out of 64 pseudorandom bits including
    /// `-0`, infinity and Nan.
    fn wild_tuple_f32(&mut self) -> (f32, f32);
}

impl<T: Rng> NonCanonical for T {
   
    fn idle(&mut self, n: usize) {
        for _ in 0..n {
            let _ = self.next_u64();
        }
    }

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

    fn tuple_f32_with_params(&mut self, left_shift: i8, signed: Sign) -> (f32, f32) {
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

/// Create a seed for the PRNG using a `&str`.
pub trait SeedStr {

    /// Generate seed for the PRNG with a `&str`.
    /// The seed is initially filled entirely with binary
    /// `1`s and modified by iteratively applying `XOR`
    /// operations with bytes from the `&str`.
    ///
    /// # Example
    /// ```rust
    /// use floaters::rand_core::{SeedableRng, RngCore};
    /// use floaters::{Xoshiro512StarStar, SeedStr, NonCanonical};
    ///
    /// let mut large_rng = Xoshiro512StarStar::seed_from_str(
    /// "Lorem ipsum dolor sit amet, consectetur adipisici elit, \
    /// sed eiusmod tempor incidunt ut labore et dolore magna \
    /// aliqua.");
    ///
    /// large_rng.idle(42);
    ///
    /// assert_eq!(0x3d2687892b2357c1, large_rng.next_u64());
    /// ```
    fn seed_from_str(input: &str) -> Self;
}

impl SeedStr for rand_xoshiro::Xoshiro512StarStar {

    fn seed_from_str(input: &str) -> Self {
        let seed512 = u8_64_from_str(input);
        let seed = rand_xoshiro::Seed512(seed512);
        Self::from_seed(seed)
    }

}

impl SeedStr for rand_xoshiro::Xoshiro512PlusPlus {

    fn seed_from_str(input: &str) -> Self {
        let seed512 = u8_64_from_str(input);
        let seed = rand_xoshiro::Seed512(seed512);
        Self::from_seed(seed)
    }

}

impl SeedStr for rand_xoshiro::Xoshiro512Plus {

    fn seed_from_str(input: &str) -> Self {
        let seed512 = u8_64_from_str(input);
        let seed = rand_xoshiro::Seed512(seed512);
        Self::from_seed(seed)
    }

}

impl SeedStr for rand_xoshiro::Xoshiro256StarStar {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_32_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoshiro256PlusPlus {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_32_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoshiro256Plus {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_32_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoshiro128StarStar {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_16_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoshiro128PlusPlus {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_16_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoshiro128Plus {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_16_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoroshiro128StarStar {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_16_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoroshiro128PlusPlus {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_16_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoroshiro128Plus {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_16_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoroshiro64StarStar {
    
    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_8_from_str(input))
    }

}

impl SeedStr for rand_xoshiro::Xoroshiro64Star {

    fn seed_from_str(input: &str) -> Self {
        Self::from_seed(u8_8_from_str(input))
    }

}

fn u8_64_from_str(input: &str) -> [u8; 64] {

    let mut seed = [u8::MAX; 64];

    for (i, byte) in input.as_bytes().iter().enumerate() {
        seed[i % 64] ^= *byte;
    }

    seed
}

fn u8_32_from_str(input: &str) -> [u8; 32] {
    
    let mut seed = [u8::MAX; 32];
    
    for (i, byte) in input.as_bytes().iter().enumerate() {
        seed[i % 32] ^= *byte;
    }

    seed
}

fn u8_16_from_str(input: &str) -> [u8; 16] {

    let mut seed = [u8::MAX; 16];

    for (i, byte) in input.as_bytes().iter().enumerate() {
        seed[i % 16] ^= *byte;
    }

    seed
}

fn u8_8_from_str(input: &str) -> [u8; 8] {

    let mut seed = [u8::MAX; 8];

    for (i, byte) in input.as_bytes().iter().enumerate() {
        seed[i % 8] ^= *byte;
    }

    seed
}

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
