//! Generate f32 and f64 floating-point numbers pseudorandomly in various ways with the xorshift128+ algorithm.
//! The values may be evenly or unevenly distributed, depending on the chosen method.

use getrandom::*;

fn getrandom_u64() -> Result<u64, Box<dyn std::error::Error>> {
    let mut array = [0u8; 8];
    getrandom(&mut array)?;
    let mut output: u64 = 0;
    for i in 0..8 {
        output += (array[i] as u64) << (i * 8);
    }
    Ok(output)
}

fn getrandom_nonzero64() -> Result<u64, Box<dyn std::error::Error>> {
    loop {
       let rnd = getrandom_u64()?;
       if rnd != 0u64 { break Ok(rnd); }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Xorshift128p {
    pub state: [u64; 2],
}

impl Xorshift128p {
    /// Create a new state for the xorshift128+ generator.
    /// Both elements of the array are set to `u64::MAX`.
    pub fn new() -> Self {
        Self {
            state: [u64::MAX; 2],
        }
    }

    /// Create a new state for the xorshift128+ generator using 16 random bytes.
    /// # Errors
    /// An error occurs if the retrieval of truly random bytes fails.
    pub fn new_random() -> Result<Self, Box<dyn std::error::Error>> {
        let a = getrandom_nonzero64()?;
        let b = getrandom_nonzero64()?;
        Ok(Self {
            state: [a, b],
        })
    }

    /// Create a new state for the xorshift128+ generator using an array of two 64 bit
    /// unsigned integers.
    /// # Panics
    /// Panics if both provided integers are zero.
    pub fn new_from_seed(seed: &[u64; 2]) -> Self {
        assert!(seed[0] != 0 || seed[1] != 0);
        Self {
            state: [seed[0], seed[1]],
        }
    }

    /// Create a new state for the xorshift128+ generator by iterating over the
    /// bytes from a `&str`.
    /// # Panics
    /// Panics if both resulting `u64`s are zero.
    pub fn new_from_str(input: &str) -> Self {
    
        let s = input.to_string();

        let mut seed = [u64::MAX; 2];

        for (i, byte) in s.as_bytes().iter().enumerate() {
            seed[(i/8) % 2] ^= (*byte as u64) << ( i % 8 * 8);
        }

        assert!(seed[0] != 0 || seed[1] != 0);
        Self {
            state: [seed[0], seed[1]],
        }
    }

    /// Create a new state for the xorshift128+ generator by iterating over a byte
    /// slice. If the given slice is shorter than 16 bytes, the iteration will be
    /// repeated until 16 bytes are reached.
    /// # Panics
    /// Panics if both resulting `u64`s are zero.
    pub fn new_from_bytes(seed: &[u8]) -> Self {
        let seed_len = seed.len();
        let mut extended = [0u8; 16];
        for i in 0..16 {
            extended[i] = seed[i % seed_len];
        }
        let mut a: u64 = 0;
        let mut b: u64 = 0;

        for i in 0..8 {
            a |= (extended[i] as u64) << (i * 8);
            b |= (extended[i + 8] as u64) << (i * 8);
        }
        
        assert!(seed[0] != 0 || seed[1] != 0);
        Self {
            state: [a, b]
        }
    }

    fn clock(&mut self) {
        let mut t = self.state[0];
        let s = self.state[1];
        self.state[0] = s;
        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        self.state[1] = t;
    }

    fn sum(&mut self) -> u64 {
        self.state[0].wrapping_add(self.state[1])
    }

    /// Clock the xorshift128+ generator n times without generating numbers.
    /// Typically used for initialization.
    pub fn init(&mut self, n: usize) {
        for _ in 0..n {
            self.clock();
        }
    }

    /// Generates an `f64`. Numbers generated by this method are roughly equidistributed
    /// in the unit interval.
    pub fn canonical_f64(&mut self) -> f64 {
        self.clock();
        (self.sum() >> 11) as f64 * 1.110223e-16 // = 0x1.0p-53 hex literal
    }

    /// Generates an `f64`. The maximum is 0.9999999999999999.
    /// The minimum is 0.000030517578125, and therefore much closer
    /// to zero than with the standard method. However, it comes at the cost of not
    /// getting equidistributed values. The orders of magnitude are roughly
    /// equidistributed.
    pub fn noncanonical_f64(&mut self) -> f64 {
        self.clock();
        let mut sum = self.sum();
        sum |= u64::MAX << (56 + 2) >> 2; // set bits that should be 1
        sum &= !(3u64 << 62 | 1u64 << 52); // clear bits that should be 0
        f64::from_bits(sum)
    }

    /// Generates an `f64` with the possibility of partially finetuning the bitwise
    /// operations. Reasonable values for `left_shift` are in the range of `53..=61`.
    /// The higher the value the closer you will get to zero.
    /// Outside that range you might get unexpected values or
    /// an overflow error. If `signed` is set to `true`, you will also get
    /// signed numbers. The numbers closest to zero in the given example are
    /// 0.0078125 and -0.0078125. The numbers farthest from zero are
    /// 0.9999999999999999 and -0.9999999999999999.
    /// # Examples
    /// ```rust
    /// use floaters::Xorshift128p;
    /// let mut x128p = Xorshift128p::new();
    /// let generated_f64 = x128p.with_params_f64(55, true);
    /// assert_eq!(-0.007812500014551858, generated_f64);
    /// ```
    pub fn with_params_f64(&mut self, left_shift: i8, signed: bool) -> f64 {
        let sign_mask = if signed { 1u64 } else { 3u64 };
        let ls = left_shift as usize;
        self.clock();
        let mut sum = self.sum();
        sum |= u64::MAX << (ls + 2) >> 2;
        sum &= !(sign_mask << 62 | 1u64 << 52);
        f64::from_bits(sum)
    }

    /// Generates an `f64` with a specified exponent. If `signed` is set to `true`, you will
    /// also get signed numbers. The mantissa will always be pseudorandomly generated.
    /// Only the lowest 11 bits of the specified `u16` will be used as the exponent, due to
    /// the specifications of the `f64` type.
    /// # Examples
    /// ```rust
    /// use floaters::Xorshift128p;
    /// let mut x128p = Xorshift128p::new_from_str("this should work well as a seed");
    /// x128p.init(1234);
    /// let generated_f64 = x128p.exp_f64(0b100_0000_0001u16, false);
    /// assert_eq!(4.920146994744552, generated_f64);
    /// ```
    pub fn exp_f64(&mut self, exponent: u16, signed: bool) -> f64 {
        let exp = (exponent << 5 >> 5) as u64;
        self.clock();
        let mut sum = self.sum();
        if signed { sum &= !(2047u64 << 52); } else { sum &= !(4095u64 << 52); }
        sum |= exp << 52;
        f64::from_bits(sum)
    }

    /// Generates an `f64` out of 64 pseudorandom bits without any further specification.
    /// Results may include Nan, -0 and infinity.
    pub fn wild_f64(&mut self) -> f64 {
        self.clock();
        f64::from_bits(self.sum())
    }

    /// Generates an `f32` tuple everytime the xorshift128+ generator is clocked.
    /// The first element is built from the lower 32 bits, and the second element
    /// is built from the higher 32 bits of the underlying u64.
    /// Values generated by this method are roughly equidistributed
    /// in the unit interval.
    pub fn tuple_canonical_f32(&mut self) -> (f32, f32) {
        self.clock();
        let sum = self.sum();
        let (le, be) = Self::u32_from_u64(sum);
        ( (le >> 9) as f32 * 1.192093e-07, // = 0x1.0p-23 hex literal
        (be >> 9) as f32 * 1.192093e-07 )
    }

    /// Generates an `f32` tuple. Values are not evenly distributed, but come closer
    /// to zero. The minimum value is 0.0078125, the maximum is 0.99999994.
    pub fn tuple_f32(&mut self) -> (f32, f32) {
        self.clock();
        let sum = self.sum();
        let (mut le, mut be) = Self::u32_from_u64(sum);
        ( Self::f32_from_u32(&mut le, 26, false),
        Self::f32_from_u32(&mut be, 26, false) )
    }

    /// Generates an `f32` tuple. Values are likely not evenly distributed.
    /// Reasonable values for `left_shift` are in the range `21..=29`.
    /// Creates signed values if `signed` is `true`.
    pub fn tuple_with_params_f32(&mut self, left_shift: i8, signed: bool) -> (f32, f32) {
        self.clock();
        let (mut le, mut be) = Self::u32_from_u64(self.sum());
        ( Self::f32_from_u32(&mut le, left_shift, signed),
        Self::f32_from_u32(&mut be, left_shift, signed) )
    }
    
    /// Generates an `f32` tuple from 64 pseudorandom bits. Values may include
    /// Nan, -0 and infinity.
    pub fn tuple_wild_f32(&mut self) -> (f32, f32) {
        self.clock();
        let (le, be) = Self::u32_from_u64(self.sum());
        ( f32::from_bits(le), f32::from_bits(be) )
    }

    /// Generates an `f32` tuple pseudorandomly with a given exponent.
    /// Ideally, the exponent is specified in binary format.
    /// Depending on the `signed` parameter, the resulting values may be
    /// signed or unsigned.
    pub fn tuple_exp_f32(&mut self, exponent: u8, signed: bool) -> (f32, f32) {
        self.clock();
        let (mut le, mut be) = Self::u32_from_u64(self.sum());
        ( Self::specified_exp_f32(&mut le, exponent, signed),
        Self::specified_exp_f32(&mut be, exponent, signed) )
    }

    // reasonable values for left_shift: 26, 21..=29
    fn f32_from_u32(bits: &mut u32, left_shift: i8, signed: bool) -> f32 {
        let sign_mask = if signed { 1u32 } else { 3u32 };
        *bits |= u32::MAX << (left_shift + 2) >> 2;
        *bits &= !(sign_mask << 30 | 1u32 << 23);
        f32::from_bits(*bits)
    }

    fn u32_from_u64(bits: u64) -> (u32, u32) {
        ( (bits << 32 >> 32) as u32,
        (bits >> 32) as u32 )
    }

    fn specified_exp_f32(bits: &mut u32, exponent: u8, signed: bool) -> f32 {
        if signed { *bits &= !(255u32 << 23); } else { *bits &= !(511u32 << 23); }
        *bits |= (exponent as u32) << 23;
        f32::from_bits(*bits)
    }
}


pub mod utilities {
//! This module contains functions that return the minimum and maximum of a given exponent.
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
