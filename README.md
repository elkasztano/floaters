# Floaters

Create floating-point numbers in various ways using pseudo random number generators (PRNG).
This crate should be considered largely experimental.

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
floaters = "0.2.0"
```

## Examples

```rust
use floaters::generators::{Xorshift128p, Xoroshiro256pp};
use floaters::Sign;
use floaters::utilities::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Initialize the Xorshift128+ generator.
    let mut x128p = Xorshift128p::new_random()?;

    // Print 5 roughly equidistributed floating-point numbers.
    println!("canonical f64:");
    for _ in 0..5 {
        println!("{}", x128p.canonical_f64());
    }

    // Print 5 floating-point numbers that may get closer to zero at
    // the cost of not being equidistributed - however, the orders
    // of magnitude should be roughly equidistributed.
    println!("\nnoncanonical f64");
    for _ in 0..5 {
        println!("{}", x128p.noncanonical_f64());
    }

    // Print 5 not equidistributed floating-point numbers with
    // specified parameters including negative numbers.
    println!("\nf64 - params: '57, Sign::Signed'");
    for _ in 0..5 {
        println!("{}", x128p.with_params_f64(57, Sign::Signed));
    }

    // Print 5 f32 tuples instead of single f64's.
    println!("\nf32 tuples");
    for _ in 0..5 {
        println!("{:?}", x128p.tuple_f32());
    }

    // Print f64 floating-point numbers with a specified exponent
    // (11 bits).
    let exponent: u16 = 0b100_0000_0001;
    println!("\nf64 - given exponent: {:011b}", exponent);
    // show minimum and maximum (unsigned) with the given exponent
    println!("bounds (+/-): {:?}", exponent_bounds_f64(exponent));
    for _ in 0..5 {
        let my_number = x128p.exp_f64(exponent, Sign::Signed);
        println!("{}", my_number);
        println!("bits: {:064b}", my_number.to_bits());
    }

    // Generate f64 numbers from 64 pseudorandom bits - anything may
    // happen. The resulting numbers may include Nan,
    // -0 or +/- infinity.
    println!("\nf64 - all bits are pseudorandom");
    for _ in 0..5 {
        println!("{:?}", x128p.wild_f64());
    }

    // Create pseudorandom f32 tuples including Nan, -0 and infinity.
    println!("\nf32 tuple - all bits are pseudorandom");
    for _ in 0..5 {
        println!("{:?}", x128p.tuple_wild_f32());
    }

    // Generate f32 tuples with specified parameters.
    let left_shift = 25;
    let sign = Sign::Unsigned;
    println!("\nf32 tuples with params '{},{:?}'", left_shift, sign);
    for _ in 0..5 {
        println!("{:?}",
        x128p.tuple_with_params_f32(left_shift, sign));
    }

    // Specify exponent in binary format for f32 tuple. (8 bits)
    let expf32: u8 = 0b1000_0001;
    println!("\nexponent for f32 tuple: {:08b}", expf32);
    for _ in 0..5 {
        println!("{:?}", x128p.tuple_exp_f32(expf32, Sign::Signed));
    }

    println!("\nbounds of exponent '{:08b}' (f32): +/-{:?}",
             expf32, exponent_bounds_f32(expf32));

    let mut x128p2 = Xorshift128p::new_from_str(
        "this should work well as a seed");
    x128p2.init(1234);
    let my_f64 = x128p2.exp_f64(0b100_0000_0001u16, Sign::Unsigned);
    println!("\nexample generated with seed: {}", my_f64);

    // Initialize the Xoroshiro256++ generator.
    let mut xrsr256pp = Xoroshiro256pp::new_random()?;

    // Clock it a few times without generating any numbers.
    xrsr256pp.init(321);
    
    // Generate five f32 tuples and print them.
    println!("\nFive f32 tuples generated with Xoroshiro256++:");
    for _ in 0..5 {
        println!("{:?}", xrsr256pp.tuple_f32());
    }

    // Create a vector containing f64 numbers.
    let my_vector: Vec<f64> = (0..5)
        .map(|_| xrsr256pp.canonical_f64() )
        .collect();
    
    println!("\nVector example:\n {:?}", my_vector);

    Ok(())
}
```

## References

[https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)

[https://prng.di.unimi.it](https://prng.di.unimi.it)
