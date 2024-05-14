# Floaters

Create pseudorandom floating-point numbers in various ways using the Xorshift128+ algorithm.

## Usage

Add the following line to your Cargo.toml:

```toml
[dependencies]
floaters = "0.1.0"
```

## Examples

```rust
use floaters::Xorshift128p;
use floaters::utilities::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // initialize the pseudorandom number generator
    let mut x128p = Xorshift128p::new_random()?;

    // print 5 roughly equidistributed floating-point numbers
    println!("canonical f64:");
    for _ in 0..5 {
        println!("{}", x128p.canonical_f64());
    }

    // print 5 floating-point numbers that may get closer to zero at the cost
    // of not being equidistributed - however, the orders of magnitude should
    // be roughly equidistributed
    println!("\nnoncanonical f64");
    for _ in 0..5 {
        println!("{}", x128p.noncanonical_f64());
    }

    // print 5 not equidistributed floating-point numbers with specified parameters
    // including negative numbers
    println!("\nf64 - params: '57,true'");
    for _ in 0..5 {
        println!("{}", x128p.with_params_f64(57, true));
    }

    // print 5 f32 tuples instead of single f64's
    println!("\nf32 tuples");
    for _ in 0..5 {
        println!("{:?}", x128p.tuple_f32());
    }

    // print floating-point numbers with a specified exponent (11 bits)
    let exponent: u16 = 0b100_0000_0001;
    println!("\nf64 - given exponent: {:011b}", exponent);
    // show minimum and maximum (unsigned) with the given exponent
    println!("bounds (+/-): {:?}", exponent_bounds_f64(exponent));
    for _ in 0..5 {
        let my_number = x128p.exp_f64(exponent, true);
        println!("{}", my_number);
        println!("bits: {:064b}", my_number.to_bits());
    }

    // generate f64 numbers from 64 pseudorandom bits - anything may happen
    // the resulting numbers may include Nan, -0 or +/- infinity
    println!("\nf64 unrestricted - all bits are pseudorandom");
    for _ in 0..5 {
        println!("{:?}", x128p.wild_f64());
    }

    // create pseudorandom f32 tuples including Nan, -0 and infinity
    println!("\nf32 tuple - all bits are pseudorandom");
    for _ in 0..5 {
        println!("{:?}", x128p.tuple_wild_f32());
    }

    // generate f32 tuples with specified parameters
    let left_shift = 25;
    let sign = false;
    println!("\nf32 tuples with params '{},{:?}'", left_shift, sign);
    for _ in 0..5 {
        println!("{:?}",
        x128p.tuple_with_params_f32(left_shift, sign));
    }

    // specify exponent in binary format for f32 tuple
    let exponentf32: u8 = 0b1000_0001;
    println!("\nexponent for f32 tuple: {:08b}", exponentf32);
    for _ in 0..5 {
        println!("{:?}", x128p.tuple_exp_f32(exponentf32, true));
    }

    println!("\nbounds of exponent '{:08b}' (f32): {:?}",
             exponentf32, exponent_bounds_f32(exponentf32));

    let mut x128p2 = Xorshift128p::new_from_str("this should work well as a seed");
    x128p2.init(1234);
    let generated_f64 = x128p2.exp_f64(0b100_0000_0001u16, false);
    println!("\nexample generated with seed: {}", generated_f64);

    Ok(())
}
```

## References

[https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)

[https://prng.di.unimi.it](https://prng.di.unimi.it)
