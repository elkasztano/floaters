# Floaters

Create floating-point numbers pseudorandomly in several experimental ways.
Not only the significand may be filled with pseudorandom bits, but also
at least parts of the exponent, depending on the chosen method. If
the exponent contains pseudorandom bits, the numbers will not be
uniformly distributed. 
Those methods may be useful in some cases where you need a wider range
of numbers, while equidistribution is not a priority.
The sign bit may be pseudorandom as well, depending on the method.

The crate adds a trait for any type that implements `Rng` from `rand_core`.
For convenience, `rand_xoshiro` is re-exported.

In addition, the `SeedStr` trait allows you to create a seed for PRNGs from the xoshiro family by iterating over bytes from a string slice.

Version `0.3.0` comes with an almost complete redesign. Earlier versions contained re-implementations of Xorshift128+ and Xoroshiro256++.

## Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
floaters = "0.3.0"
```

## Examples

```rust
use floaters::rand_core::SeedableRng;
use floaters::{Xoshiro256StarStar, NonCanonical, SeedStr};

fn main() {

    // Generate 5 pseudorandom f64 numbers with pseudorandom bits
    // also in the exponent. The numbers are not equidistributed,
    // but we get a wider range of values.
    
    let mut rng = Xoshiro256StarStar::from_entropy();
    
    println!("Numbers get closer to zero, \
             but are not equidistributed:");
    
    for _ in 0..5 { println!("{}", rng.noncanonical_f64()) }

    // Generate tuples of f32 numbers with a fixed seed.
    // The seed is generated by iterating over the bytes
    // of a string slice.
    // The sequence of numbers will always be the same.
    
    let mut rng2 = Xoshiro256StarStar::seed_from_str(
        "Lorem ipsum dolor sit amet, consectetur adipisici elit, \
        sed eiusmod tempor incidunt ut labore et dolore magna \
        aliqua.");

    // Clock the PRNG several times without generating numbers.
    rng2.idle(512);
    
    println!("\nSigned (f32, f32) with predefined seed:");
    
    for _ in 0..5 { println!("{:?}", rng2.signed_tuple_f32()); }

}
```

## References

[https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)

[https://prng.di.unimi.it](https://prng.di.unimi.it)

[https://mina86.com/2016/random-reals/](https://mina86.com/2016/random-reals/)
