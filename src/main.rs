#![feature(test)]

extern crate test;

mod primes;

fn main() {
    //println!("{}", primes::primes(10).len());
	//println!("{}", primes::primes2(10).len());
	println!("{:?}", primes::primes(1000).len());
}
