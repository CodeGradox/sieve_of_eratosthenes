pub struct Primes {
	bit_arr: Vec<u8>,
	max_num: usize,
}

impl Primes {
	pub fn new(num: usize) -> Self {
		let mut primes = Primes {
			bit_arr: vec![255u8; (num / 16) + 1],
			max_num: num,
		};
		primes.generate_primes();
		primes
	}

	/// Returns a bit mask for the num.
	/// The mask coresponds to the position of num in the bitmap array which stores
	/// all the odd numbers. Num is an odd number.
	fn mask(num: &usize) -> u8 {
		// The bit mask is the position of num in a given bit.
		// 1 => 10000000, 3 => 01000000, 5 => 00100000, ...
		128 >> ((num % 16) >> 1)
	}

	/// Marks num as not prime in the number array.
	fn cross_out(&mut self, num: &usize) {
		// Marks num in the bit array as 0 (not a prime)
		self.bit_arr[num >> 4] &= !Self::mask(num);
	}

	/// Checks if the odd number num is a prime.
	fn is_prime(&self, num: &usize) -> bool {
		// Checks if the bit coresponding to num in the bit array is 1 (prime)
		(self.bit_arr[num >> 4] & Self::mask(num)) != 0u8
	}

	/// Generates all prime numers < max_num
	fn generate_primes(&mut self) {
		self.cross_out(&1);

		// Loops through all odd numbers from 3 to √max_num
		let mut i = 3;
		while i * i < self.max_num {
			// If i is a prime, mark the odd multiples off i
			// Even multiples are skipped because they cannot be primes
			if self.is_prime(&i) {
				let mut prime = i * i;
				while prime < self.max_num {
					self.cross_out(&prime);
					prime += i * 2;
				}
			}
			i += 2;
		}
	}

	/// Finds a prime number which appears after num.
	pub fn next_prime(&self, num: &usize) -> Option<usize> {
		if self.max_num < 2 {
			return None;
		}
		if *num < 2 {
			return Some(2);
		}

		let mut p = if num % 2 == 0 { *num + 1} else { num + 2}; // odd or even?
		while p < self.max_num {
			if self.is_prime(&p) {
				return Some(p);
			}
			p += 2;
		}
		None
	}

	/// Finds the number of primes < max_num
	pub fn num_primes(&self) -> usize {
		self.iter().fold(0, |sum, _| sum + 1)
	}

	/// Collects all the prime numbers into a vector
	pub fn collect(&self) -> Vec<usize> {
		self.iter().collect::<Vec<usize>>()
	}

	/// Returns a borrowed iterator for primes
	pub fn iter(&self) -> Iter {
		Iter {primes: self, index: 1}
	}
}

pub struct Iter<'a> {
	primes: &'a Primes,
	index: usize,
}

impl<'a> Iterator for Iter<'a> {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.primes.next_prime(&self.index).map(|num| {
			self.index = num;
			num
		})
	}
}

impl<'a> IntoIterator for &'a Primes {
	type Item = usize;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
