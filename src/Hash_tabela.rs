// use rand::Rng;

const PRIME: u64 = 2_305_843_009_213_693_951;
const MAX_TOP_LEVEL_TRIES: usize = 128;
const MAX_BUCKET_TRIES: usize = 512;

#[derive(Clone, Copy, Debug)]
struct UniversalHash {
	a: u64,
	b: u64,
	m: usize,
}

// https://en.wikipedia.org/wiki/Universal_hashing 
// hash funkcija je odvisna od a, b, p(=PRIME), m
impl UniversalHash {
	fn new(a: u64, b: u64, m: usize) -> Self {
		Self { a, b, m }
		// m ... velikost tabele
		// a, b ... parametra za hash funkcijo
	}

	fn apply(&self, key: u64) -> usize {
		if self.m == 0 {
			return 0;
		}

		// h(x) = ((a * x + b) mod p) mod m
		let value = (self.a as u128 * key as u128 + self.b as u128) % PRIME as u128;
		(value % self.m as u128) as usize
	}
}

#[derive(Debug)]
enum Bucket {
	Empty,
	Single(u64),
	Table {
		hash: UniversalHash,
		slots: Vec<Option<u64>>,
	},
}

impl Bucket {
	fn contains(&self, key: u64) -> bool {
		match self {
			Self::Empty => false,
			Self::Single(existing) => *existing == key,
			Self::Table { hash, slots } => {
				if slots.is_empty() {
					return false;
				}

				let index = hash.apply(key);
				slots[index] == Some(key)
			}
		}
	}
}

#[derive(Debug)]
pub struct PerfectHashSet {
	size: usize,
	top_hash: UniversalHash,
	buckets: Vec<Bucket>,
}

impl PerfectHashSet {
	pub fn new(keys: &[u64]) -> Self {
		let mut unique: Vec<u64> = keys.to_vec();
		unique.sort_unstable();
		unique.dedup();

		let n = unique.len();
		if n == 0 {
			return Self {
				size: 0,
				top_hash: UniversalHash::new(1, 0, 0),
				buckets: Vec::new(),
			};
		}

		// let mut rng = SimpleRng::new(/* system randomness */);
		let mut rng = SimpleRng::new(seed_from_keys(&unique));
		let m = n;

		for _ in 0..MAX_TOP_LEVEL_TRIES {
			let top_hash = random_hash(m, &mut rng);
			let mut partitioned = vec![Vec::<u64>::new(); m];

			for &key in &unique {
				partitioned[top_hash.apply(key)].push(key); 
				// za enkrat shranimo elemente, ki bodo skupaj v vedru samo v vektor. Kasneje to spremenimo v hash tabelo.
			}

			let sum_of_squares: usize = partitioned
				.iter()
				.map(|bucket| bucket.len() * bucket.len())
				.sum();

			if sum_of_squares > 4 * n { // Preverimo če je hash tabela dobra (da ni preveč elementov v istih vedrih). Če ni dovolj dobro, naredimo nov hash.
				continue;
			}

			let mut buckets = Vec::with_capacity(m);
			let mut success = true;

			for stvari_v_vedru in partitioned { // Preverimo da ni trkov
				match build_bucket(&stvari_v_vedru, &mut rng) {
					Some(bucket) => buckets.push(bucket), // vrstni red vedrov je isti kot vrstni red v partitioned, ker se vedno izvede Some del, ker drugače probamo še enkrat. Ampak to se nikoli ne zgodi, ker hashi v vedrih poskrbijo za to.
					None => {
						success = false;
						break;
					}
				}
			}

			if success {
				return Self {
					size: n,
					top_hash,
					buckets,
				};
			}
		}

		panic!("failed to build perfect hash table after multiple attempts");
	}

	pub fn len(&self) -> usize {
		self.size
	}

	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	pub fn contains(&self, key: u64) -> bool {
		if self.buckets.is_empty() {
			return false;
		}

		let bucket_index = self.top_hash.apply(key);
		self.buckets[bucket_index].contains(key)
	}
}

fn build_bucket(keys: &[u64], rng: &mut SimpleRng) -> Option<Bucket> {
	match keys.len() {
		0 => Some(Bucket::Empty),
		1 => Some(Bucket::Single(keys[0])),
		size => {
			let m = size * size;

			for _ in 0..MAX_BUCKET_TRIES {
				let hash = random_hash(m, rng);
				let mut slots = vec![None; m];
				let mut collision = false;

				for &key in keys {
					let idx = hash.apply(key);
					if slots[idx].is_some() {
						collision = true;
						break;
					}
					slots[idx] = Some(key);
				}

				if !collision {
					return Some(Bucket::Table { hash, slots });
				}
			}

			None
		}
	}
}

fn random_hash(m: usize, rng: &mut SimpleRng) -> UniversalHash {
	// let a = rand.thread_rng().gen_range(1..(PRIME-1));
	// let b = rand.thread_rng().gen_range(0..(PRIME-1));
	let a = 1 + (rng.next_u64() % (PRIME - 1));  // 0 < a < p
	let b = rng.next_u64() % PRIME; // 0 <= b < p
	UniversalHash::new(a, b, m)
}

fn seed_from_keys(keys: &[u64]) -> u64 {
	let mut seed = 0x9E37_79B9_7F4A_7C15_u64 ^ keys.len() as u64;
	for &k in keys {
		seed ^= k.wrapping_mul(0xBF58_476D_1CE4_E5B9);
		seed = seed.rotate_left(13); // 0110001.rotate_left(2) = 1000101
	}
	seed
}


// To je kao mal bl advanced, ampak lahko sam uporabmo random na a in b
#[derive(Debug)]
struct SimpleRng {
	state: u64,
}

impl SimpleRng {
	fn new(seed: u64) -> Self {
		let fixed_seed = if seed == 0 { 0xA076_1D64_78BD_642F } else { seed };
		Self { state: fixed_seed }
	}

	fn next_u64(&mut self) -> u64 {
		self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
		let mut z = self.state;
		z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
		z ^ (z >> 31)
	}
}

#[cfg(test)]
mod tests {
	use super::PerfectHashSet;

	#[test]
	fn builds_and_answers_membership() {
		let keys = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
		let table = PerfectHashSet::new(&keys);

		for key in keys {
			assert!(table.contains(key));
		}

		assert!(!table.contains(4));
		assert!(!table.contains(31));
	}

	#[test]
	fn handles_duplicates_and_empty_input() {
		let table = PerfectHashSet::new(&[10, 10, 10, 42, 42]);
		assert_eq!(table.len(), 2);
		assert!(table.contains(10));
		assert!(table.contains(42));
		assert!(!table.contains(11));

		let empty = PerfectHashSet::new(&[]);
		assert!(empty.is_empty());
		assert!(!empty.contains(1));
	}
}
