use rand::Rng;

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

    // Časovna zahtevnost: O(1)
    fn apply(&self, key: u64) -> usize {
        if self.m == 0 {
            return 0;
        }

        // h(x) = ((a * x + b) mod p) mod m
        let value = (self.a as u128 * key as u128 + self.b as u128) % PRIME as u128;
        (value % self.m as u128) as usize
        // Vmesni rezultati so lahko zelo veliki, zato jih računamo kot u128 (namesto u64)
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
    // Časovna zahtevnost: O(1)
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
    keys: Vec<u64>,
}



/// Ima vrhnji hash, ki najde vedo, nato pa še en hash, ki najde mesto v vedru
/// |1| [.]
/// |2| [. . . .]
/// |0| []
/// |0| []
/// |3| [. . . . . . . . .]
/// |0| []
/// 
/// n ... število vseh elementov
/// k_i ... število elementov v i-tem vedru
/// 
/// velikost vrhnje tabele je n, velikost vedr je enako k_i^2
impl PerfectHashSet {
    pub fn new(keys: &[u64]) -> Self {
        let mut unique: Vec<u64> = keys.to_vec();
        unique.sort_unstable();
        unique.dedup();

        let n = unique.len();
        let keys_copy = unique.clone();
        if n == 0 {
            return Self {
                size: 0,
                top_hash: UniversalHash::new(1, 0, 0),
                buckets: Vec::new(),
                keys: keys_copy,
            };
        }

        let m = n;

        for _ in 0..MAX_TOP_LEVEL_TRIES {
            let top_hash = random_hash(m); // O(1)
            let mut partitioned = vec![Vec::<u64>::new(); m]; // O(m)

            for &key in &unique {
                partitioned[top_hash.apply(key)].push(key);
                // za enkrat shranimo elemente, ki bodo skupaj v vedru samo v vektor. Kasneje to spremenimo v hash tabelo.
            }

            let sum_of_squares: usize = partitioned
                .iter()
                .map(|bucket| bucket.len() * bucket.len())
                .sum();

            if sum_of_squares > 4 * n {
                // Preverimo če je hash tabela dobra (da ni preveč elementov v istih vedrih). Če ni dovolj dobro, naredimo nov hash.
                continue;
            }

            let mut buckets = Vec::with_capacity(m);
            let mut success = true;

            for stvari_v_vedru in partitioned {
                // Preverimo da ni trkov
                match build_bucket(&stvari_v_vedru) {
                    // vrstni red vedrov je isti kot vrstni red v partitioned, ker se vedno izvede Some del, ker drugače probamo še enkrat. Ampak to se nikoli ne zgodi, ker hashi v vedrih poskrbijo za to.
                    Some(bucket) => buckets.push(bucket),
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
                    keys: keys_copy,
                };
            }
        }

        panic!("failed to build perfect hash table after multiple attempts");
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // O(1)
    pub fn contains(&self, key: u64) -> bool {
        if self.buckets.is_empty() {
            return false;
        }

        let bucket_index = self.top_hash.apply(key);
        self.buckets[bucket_index].contains(key)
    }

    pub fn extract_keys(&self) -> Vec<u64> {
        self.keys.clone()
    }
}

fn build_bucket(keys: &[u64]) -> Option<Bucket> {
    match keys.len() {
        0 => Some(Bucket::Empty),
        1 => Some(Bucket::Single(keys[0])),
        size => {
            let m = size * size;

            for _ in 0..MAX_BUCKET_TRIES {
                let hash = random_hash(m);
                let mut slots = vec![None; m];
                let mut collision = false;

                for &key in keys {
                    // O(len(vedro_i))
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

// O(1)
fn random_hash(m: usize) -> UniversalHash {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..PRIME); // 0 < a < p
    let b = rng.gen_range(0..PRIME); // 0 <= b < p
    UniversalHash::new(a, b, m)
}
