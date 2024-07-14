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

In version `0.4.0` the `SeedStr` trait was removed entirely. If you need
this kind of functionality, you may use the `rand_seeder` crate instead.

## Examples

![plot](https://github.com/elkasztano/floaters/blob/main/prng_walk.png?raw=true)

The above plot was created with the following code:


```rust
use floaters::{Xoshiro256PlusPlus, NonCanonical, Sign};
use floaters::rand_core::RngCore;
use rand_seeder::Seeder;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new("random_walk.png", (800, 600))
        .into_drawing_area();
    
    root.fill(&WHITE)?;

    let mut prng: Xoshiro256PlusPlus =
        Seeder::from("walk the line").make_rng();

    (0..1000).for_each(|_| { prng.next_u64(); } );

    let mut x: f64 = 50.0;

    let mut numbers = Vec::<f64>::with_capacity(100);

    (0..100).for_each(|_| {
        x += prng.with_params_f64(55, Sign::Signed);
        numbers.push(x);
    });

    let mut chart = ChartBuilder::on(&root)
        .caption("(pseudo)random walk",
            ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..100f64, 40f64..60f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(numbers
                                    .iter()
                                    .enumerate()
                                    .map(|x| (x.0 as f64, *x.1)),
                                        &RED,))?;

    root.present()?;

    Ok(())
}
```

A more basic example:

```rust
use floaters::rand_core::{RngCore, SeedableRng};
use floaters::{Xoshiro256StarStar, NonCanonical};

fn main() {

    // Generate 5 pseudorandom f64 numbers with pseudorandom bits
    // also in the exponent. The numbers are not equidistributed,
    // but we get a wider range of values.
    
    let mut rng = Xoshiro256StarStar::from_entropy();
    
    println!("Numbers get closer to zero, \
             but are not equidistributed:");
    
    for _ in 0..5 { println!("{}", rng.noncanonical_f64()) }

    // Generate tuples of f32 numbers with a truly random seed.
    
    // Clock the PRNG several times without generating numbers.
    (0..512).for_each(|_| { rng.next_u64(); } );
    
    println!("\nSigned (f32, f32):");
    
    for _ in 0..5 { println!("{:?}", rng.signed_tuple_f32()); }

}
```




## References

[https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)

[https://prng.di.unimi.it](https://prng.di.unimi.it)

[https://mina86.com/2016/random-reals/](https://mina86.com/2016/random-reals/)
