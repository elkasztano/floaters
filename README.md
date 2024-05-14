# Floaters

Create pseudorandom floating-point numbers in various ways using the Xorshift128+ algorithm.

## Usage

Add the following line to your Cargo.toml:

```toml
[dependencies]
floaters = "0.1.0"
```

## Example

```rust
use floaters::Xorshift128p;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // initialize the pseudorandom number generator
    let mut x128p = Xorshift128p::new_random()?;

    // print 5 roughly equidistributed floating-point numbers
    print!("\n");
    for _ in 0..5 {
        println!("canonical f64: {}", x128p.canonical_f64());
    }

    // print 5 floating-point numbers that come closer to zero at the cost
    // of not being equidistributed - however, the orders of magnitude should
    // be roughly equidistributed
    print!("\n");
    for _ in 0..5 {
        println!("noncanonical f64: {}", x128p.noncanonical_f64());
    }

    // print 5 not equidistributed floating-point numbers with specified parameters
    // including negative numbers
    print!("\n");
    for _ in 0..5 {
        println!("param 57,truef64: {}", x128p.noncanonical_with_params_f64(57, true));
    }

    // print 5 f32 tuples instead of single f64's
    print!("\n");
    for _ in 0..5 {
        println!("f32 tuple: {:?}", x128p.tuple_f32());
    }

    // initialize the PRNG with `[u64::MAX; 2]` to create reproducible results
    // you might want to clock the PRNG several times before you actually use it
    let mut x128b = Xorshift128p::new();
    x128b.init(512);
    let my_float = x128b.noncanonical_with_params_f64(55, true);
    println!("my float: {}", my_float);

    Ok(())
}
```

## References

[https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)

[https://prng.di.unimi.it](https://prng.di.unimi.it)
