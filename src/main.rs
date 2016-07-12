extern crate sieve_of_eratosthenes;

use sieve_of_eratosthenes::Primes;

fn main() {
	println!("{:?}", Primes::new(1000).num_primes());
}
