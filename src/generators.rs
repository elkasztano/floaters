//! Pseudorandom number generators (PRNG). Currently the Xorshift128+ and the Xoroshiro256++
//! generators are implemented. This module should be considered the core element of the crate.

use crate::getrandom_nonzero64vec;
use crate::Sign;

#[derive(Debug,Copy,Clone)]
pub struct Xorshift128p {
    pub state: [u64; 2],
}

impl Xorshift128p {
    /// Create a new state for the Xorshift128+ generator.
    /// Both elements of the array are set to `u64::MAX`.
    pub fn new() -> Self {
        Self {
            state: [u64::MAX; 2],
        }
    }

    /// Create a new state for the Xorshift128+ generator using 16 random bytes.
    /// # Errors
    /// An error occurs if the retrieval of truly random bytes fails.
    pub fn new_random() -> Result<Self, Box<dyn std::error::Error>> {
        let values = getrandom_nonzero64vec(2)?;
        let a = values[0];
        let b = values[1];
        Ok(Self {
            state: [a, b],
        })
    }

    /// Create a new state for the Xorshift128+ generator using an array of two 64 bit
    /// unsigned integers.
    /// # Panics
    /// Panics if both provided integers are zero.
    pub fn new_from_seed(seed: &[u64; 2]) -> Self {
        assert!(seed[0] != 0 || seed[1] != 0);
        Self {
            state: [seed[0], seed[1]],
        }
    }

    /// Create a new state for the Xorshift128+ generator by iterating over the
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

    /// Create a new state for the Xorshift128+ generator by iterating over a byte
    /// slice. If the given slice is shorter than 16 bytes, the iteration will be
    /// repeated until 16 bytes are reached.
    /// # Examples
    /// ```rust
    /// use floaters::generators::Xorshift128p;
    /// use floaters::Sign;
    /// let mut x128p = Xorshift128p::new_from_bytes(&[0x12, 0x23, 0x34, 0x35]);
    /// x128p.init(1337);
    /// let generated_f64 = x128p.canonical_f64();
    /// assert_eq!(0.4392372924505977, generated_f64);
    /// ```
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

    /// Clock the Xorshift128+ generator n times without generating numbers.
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
        crate::float::canonical(self.sum())
    }

    /// Generates an `f64`. The maximum is 0.9999999999999999.
    /// The minimum is 0.000030517578125, and therefore much closer
    /// to zero than with the standard method. However, it comes at the cost of not
    /// getting equidistributed values. The orders of magnitude are roughly
    /// equidistributed.
    pub fn noncanonical_f64(&mut self) -> f64 {
        self.clock();
        crate::float::noncanonical(self.sum())
    }

    /// Generates an `f64` with the possibility of partially finetuning the bitwise
    /// operations. Reasonable values for `left_shift` are in the range of `53..=61`.
    /// The higher the value the closer you will get to zero.
    /// Outside that range you might get unexpected values or
    /// an overflow error. If `Sign` is the `Signed` variant, you will also get
    /// signed numbers. The numbers closest to zero in the given example are
    /// 0.0078125 and -0.0078125. The numbers farthest from zero are
    /// 0.9999999999999999 and -0.9999999999999999.
    /// # Examples
    /// ```rust
    /// use floaters::generators::Xorshift128p;
    /// use floaters::Sign;
    /// let mut x128p = Xorshift128p::new();
    /// let generated_f64 = x128p.with_params_f64(55, Sign::Signed);
    /// assert_eq!(-0.007812500014551858, generated_f64);
    /// ```
    pub fn with_params_f64(&mut self, left_shift: i8, sign: Sign) -> f64 {
        self.clock();
        crate::float::with_params(self.sum(), left_shift, sign)
    }

    /// Generates an `f64` with a specified exponent. If `Sign` is the `Signed` variant,
    /// you will also get signed numbers. The mantissa will always be pseudorandomly 
    /// generated.
    /// Only the lowest 11 bits of the specified `u16` will be used as the exponent, due to
    /// the specifications of the `f64` type.
    /// # Examples
    /// ```rust
    /// use floaters::generators::Xorshift128p;
    /// use floaters::Sign;
    /// let mut x128p = Xorshift128p::new_from_str("this should work well as a seed");
    /// x128p.init(1234);
    /// let generated_f64 = x128p.exp_f64(0b100_0000_0001u16, Sign::Unsigned);
    /// assert_eq!(4.920146994744552, generated_f64);
    /// ```
    pub fn exp_f64(&mut self, exponent: u16, sign: Sign) -> f64 {
        self.clock();
        crate::float::exponent(self.sum(), exponent, sign)
    }

    /// Generates an `f64` out of 64 pseudorandom bits without any further specification.
    /// Results may include Nan, -0 and infinity.
    pub fn wild_f64(&mut self) -> f64 {
        self.clock();
        f64::from_bits(self.sum())
    }

    /// Generates an `f32` tuple everytime the Xorshift128+ generator is clocked.
    /// The first element is built from the lower 32 bits, and the second element
    /// is built from the higher 32 bits of the underlying u64.
    /// Values generated by this method are roughly equidistributed
    /// in the unit interval.
    pub fn tuple_canonical_f32(&mut self) -> (f32, f32) {
        self.clock();
        crate::float::canonical_tuple(self.sum())
    }

    /// Generates an `f32` tuple. Values are not evenly distributed, but come closer
    /// to zero. The minimum value is 0.0078125, the maximum is 0.99999994.
    pub fn tuple_f32(&mut self) -> (f32, f32) {
        self.clock();
        crate::float::noncanonical_tuple(self.sum())
    }

    /// Generates an `f32` tuple. Values are likely not evenly distributed.
    /// Reasonable values for `left_shift` are in the range `21..=29`.
    /// Creates signed values if `signed` is `true`.
    pub fn tuple_with_params_f32(&mut self, left_shift: i8, signed: Sign) -> (f32, f32) {
        self.clock();
        crate::float::tuple_with_params(self.sum(), left_shift, signed)
    }
    
    /// Generates an `f32` tuple from 64 pseudorandom bits. Values may include
    /// Nan, -0 and infinity.
    pub fn tuple_wild_f32(&mut self) -> (f32, f32) {
        self.clock();
        crate::float::tuple_wild(self.sum())
    }

    /// Generates an `f32` tuple pseudorandomly with a given exponent.
    /// Ideally, the exponent is specified in binary format.
    /// Depending on the `signed` parameter, the resulting values may be
    /// signed or unsigned.
    pub fn tuple_exp_f32(&mut self, exponent: u8, signed: Sign) -> (f32, f32) {
        self.clock();
        crate::float::tuple_exp(self.sum(), exponent, signed) 
    }
    
}

#[derive(Debug,Copy,Clone)]
pub struct Xoroshiro256pp {
    pub state: [u64; 4],
}

impl Xoroshiro256pp {
    /// Create a new state for the Xoroshiro256++ generator.
    /// All four elements of the array are set to `u64::MAX`.
    pub fn new() -> Self {
        Self {
            state: [u64::MAX; 4],
        }
    }

    /// Create a new state for the Xoroshiro256++ generator using 32 random bytes.
    /// # Errors
    /// An error occurs if the retrieval of truly random bytes fails.
    pub fn new_random() -> Result<Self, Box<dyn std::error::Error>> {
        let values = getrandom_nonzero64vec(4)?;
        let a = values[0];
        let b = values[1];
        let c = values[2];
        let d = values[3];
        Ok(Self {
            state: [a, b, c, d],
        })
    }

    /// Create a new state for the Xoroshiro256++ generator using an array 
    /// of four 64 bit unsigned integers.
    /// # Panics
    /// Panics if all four specified unsigned integers are zero.
    pub fn new_from_seed(seed: &[u64; 4]) -> Self {
        assert!(seed[0] != 0 || seed[1] != 0 || seed[2] != 0 || seed[3] != 0);
        Self {
            state: [seed[0], seed[1], seed[2], seed[3]],
        }
    }

    /// Create a new state for the Xoroshiro256++ generator by iterating over the
    /// bytes from a `&str`.
    /// # Panics
    /// Panics if all four resulting `u64`s are zero.
    pub fn new_from_str(input: &str) -> Self {
    
        let s = input.to_string();

        let mut seed = [u64::MAX; 4];

        for (i, byte) in s.as_bytes().iter().enumerate() {
            seed[(i/8) % 4] ^= (*byte as u64) << ( i % 8 * 8);
        }

        assert!(seed[0] != 0 || seed[1] != 0 || seed[2] != 0 || seed[3] != 0);
        Self {
            state: [seed[0], seed[1], seed[2], seed[3]],
        }
    }

    /// Create a new state for the Xoroshiro256++ generator by iterating over a byte
    /// slice. If the given slice is shorter than 32 bytes, the iteration will be
    /// repeated until 32 bytes are reached.
    /// # Panics
    /// Panics if all four resulting `u64`s are zero.
    pub fn new_from_bytes(seed: &[u8]) -> Self {
        
        let seed_len = seed.len();
        let mut extended = [0u8; 32];
        
        for i in 0..32 {
            extended[i] = seed[i % seed_len];
        }

        let mut a: u64 = 0;
        let mut b: u64 = 0;
        let mut c: u64 = 0;
        let mut d: u64 = 0;

        for i in 0..8 {
            a |= (extended[i + 8 * 0] as u64) << (i * 8);
            b |= (extended[i + 8 * 1] as u64) << (i * 8);
            c |= (extended[i + 8 * 2] as u64) << (i * 8);
            d |= (extended[i + 8 * 3] as u64) << (i * 8);
        }
        
        assert!(seed[0] != 0 || seed[1] != 0 || seed[2] != 0 || seed[3] != 0);
        Self {
            state: [a, b, c, d]
        }
    }

    fn clock(&mut self) {
        
        let t: u64 = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(45);

    }

    fn sum(&self) -> u64 {
        (self.state[0].wrapping_add(self.state[3]))
            .rotate_left(23)
            .wrapping_add(self.state[0])
    }

    /// Clock the Xoroshiro256++ generator n times without generating numbers.
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
        crate::float::canonical(self.sum())
    }

    /// Generates an `f64`. The maximum is 0.9999999999999999.
    /// The minimum is 0.000030517578125, and therefore much closer
    /// to zero than with the standard method. However, it comes at the cost of not
    /// getting equidistributed values. The orders of magnitude are roughly
    /// equidistributed.
    pub fn noncanonical_f64(&mut self) -> f64 {
        self.clock();
        crate::float::noncanonical(self.sum())
    }

    /// Generates an `f64` with the possibility of partially finetuning the bitwise
    /// operations. Reasonable values for `left_shift` are in the range of `53..=61`.
    /// The higher the value the closer you will get to zero.
    /// Outside that range you might get unexpected values or
    /// an overflow error. If `Sign` is the `Signed` variant, you will also get
    /// signed numbers. The numbers closest to zero in the given example are
    /// 0.0078125 and -0.0078125. The numbers farthest from zero are
    /// 0.9999999999999999 and -0.9999999999999999.
    /// # Examples
    /// ```rust
    /// use floaters::generators::Xoroshiro256pp;
    /// use floaters::Sign;
    /// let mut xrsr256pp = Xoroshiro256pp::new();
    /// xrsr256pp.init(5);
    /// let generated_f64 = xrsr256pp.with_params_f64(55, Sign::Signed);
    /// assert_eq!(-0.008299830861627796, generated_f64);
    /// ```
    pub fn with_params_f64(&mut self, left_shift: i8, sign: Sign) -> f64 {
        self.clock();
        crate::float::with_params(self.sum(), left_shift, sign)
    }

    /// Generates an `f64` with a specified exponent. If `Sign` is the `Signed` variant,
    /// you will also get signed numbers. The mantissa will always be pseudorandomly 
    /// generated. Only the lowest 11 bits of the specified `u16` will be used as the 
    /// exponent, due to the specifications of the `f64` type.
    /// # Examples
    /// ```rust
    /// use floaters::generators::Xoroshiro256pp;
    /// use floaters::Sign;
    /// let mut xrsr256pp = Xoroshiro256pp::new_from_str("This might be a good seed.");
    /// xrsr256pp.init(1234);
    /// let generated_f64 = xrsr256pp.exp_f64(0b100_0000_0001u16, Sign::Unsigned);
    /// assert_eq!(7.168326901177531, generated_f64);
    /// ```
    pub fn exp_f64(&mut self, exponent: u16, sign: Sign) -> f64 {
        self.clock();
        crate::float::exponent(self.sum(), exponent, sign)
    }

    /// Generates an `f64` out of 64 pseudorandom bits without any further specification.
    /// Results may include Nan, -0 and infinity.
    pub fn wild_f64(&mut self) -> f64 {
        self.clock();
        f64::from_bits(self.sum())
    }

    /// Generates an `f32` tuple everytime the generator is clocked.
    /// The first element is built from the lower 32 bits, and the second element
    /// is built from the higher 32 bits of the underlying u64.
    /// Values generated by this method are roughly equidistributed
    /// in the unit interval.
    pub fn tuple_canonical_f32(&mut self) -> (f32, f32) {
        self.clock();
        crate::float::canonical_tuple(self.sum())
    }

    /// Generates an `f32` tuple. Values are not evenly distributed, but come closer
    /// to zero. The minimum value is 0.0078125, the maximum is 0.99999994.
    pub fn tuple_f32(&mut self) -> (f32, f32) {
        self.clock();
        crate::float::noncanonical_tuple(self.sum())
    }

    /// Generates an `f32` tuple. Values are likely not evenly distributed.
    /// Reasonable values for `left_shift` are in the range `21..=29`.
    /// Creates signed values if `signed` is `true`.
    pub fn tuple_with_params_f32(&mut self, left_shift: i8, signed: Sign) -> (f32, f32) {
        self.clock();
        crate::float::tuple_with_params(self.sum(), left_shift, signed)
    }
    
    /// Generates an `f32` tuple from 64 pseudorandom bits. Values may include
    /// Nan, -0 and infinity.
    pub fn tuple_wild_f32(&mut self) -> (f32, f32) {
        self.clock();
        crate::float::tuple_wild(self.sum())
    }

    /// Generates an `f32` tuple pseudorandomly with a given exponent.
    /// Ideally, the exponent is specified in binary format.
    /// Depending on the `signed` parameter, the resulting values may be
    /// signed or unsigned.
    pub fn tuple_exp_f32(&mut self, exponent: u8, signed: Sign) -> (f32, f32) {
        self.clock();
        crate::float::tuple_exp(self.sum(), exponent, signed) 
    }
    
}
