// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
pub fn primes(n: usize) -> Vec<usize> {
	if n < 2 {
		return vec![];
	}
	let mut a = vec![true; n];

	for i in 2..((n as f64).sqrt().ceil() as usize) {
		if a[i] {
			let mut j = i * i;
			let mut cnt = 1;
			while j < n {
				a[j] = false;
				j = i*i + i*cnt;
				cnt += 1;
			}
		}
	}
	(a[2..]).iter().zip(2..).filter(|&(&a, _)| a).map(|(_, b)| b).collect::<Vec<usize>>()
}

#[allow(dead_code)]
pub fn primes2(n: usize) -> Vec<usize> {
	if n < 2 {
		return vec![];
	}

	let mut a = vec![true; n+1];
	let mut p = 2;

	while p*p <= n {
		let mut j = p*p;
		while j <= n {
			a[j] = false;
			j += p;
		}
		//while {!a[{p+=1;p}]} {}
		while {p+=1; !a[p]} {}
	}
	a[2..].iter().zip(2..).filter(|&(&a, _)| a).map(|(_, b)| b).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
	use super::*;
	//mod primes;
	use test::Bencher;

	#[bench]
	fn bench_primes1_10(b: &mut Bencher) {
		b.iter(|| primes(10));
	}

	#[bench]
	fn bench_primes2_10(b: &mut Bencher) {
		b.iter(|| primes2(10));
	}

	#[bench]
	fn bench_primes1_1000000(b: &mut Bencher) {
		b.iter(|| primes(1000000));
	}

	#[bench]
	fn bench_primes2_1000000(b: &mut Bencher) {
		b.iter(|| primes2(1000000));
	}

	/*#[bench]
	fn bench_primes1_100000000(b: &mut Bencher) {
		b.iter(|| primes(100000000));
	}*/
}
