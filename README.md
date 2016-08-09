# Sieve of Eratosthenes

This projects is simple implementation of the sieve of Eratosthenes I used for finding primes for some of the project Euler problems.

The algorithm taken from the course INF2220 at the University of Oslo.

Usage:

```rust
extern crate sieve_of_eratosthenes;

use sieve_of_eratosthenes::Primes;

fn main() {
    let primes = Primes::new(1000);
    println!("{:?}", primes.num_primes());

    // Print all primes
    for i in primes.iter() {
        print!("{}", i);
    }
}
```
