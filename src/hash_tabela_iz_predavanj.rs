use std::time::Instant;

/// Enostavna zgoščevalna funkcija: h(x) = x mod m
fn hash(x: u64, bins: usize) -> usize {
    (x as usize) % bins
}

// ---------------------------------------------------------------------------
// Trait HashTable
// ---------------------------------------------------------------------------

trait HashTable: std::fmt::Display {
    fn new(bins: usize) -> Self;
    fn insert(&mut self, x: u64);
    fn lookup(&self, x: u64) -> bool;
    fn delete(&mut self, x: u64);
    fn load_factor(&self) -> f64;
    fn name() -> &'static str;
}

// ---------------------------------------------------------------------------
// 1. Veriženje (chaining)
// ---------------------------------------------------------------------------

struct ChainingTable {
    buckets: Vec<Vec<u64>>,
    bins: usize,
    elements: usize,
}

impl HashTable for ChainingTable {
    fn new(bins: usize) -> Self {
        Self {
            buckets: (0..bins).map(|_| Vec::new()).collect(),
            bins,
            elements: 0,
        }
    }

    fn insert(&mut self, x: u64) {
        if self.lookup(x) {
            return;
        }
        let idx = hash(x, self.bins);
        self.buckets[idx].push(x);
        self.elements += 1;
    }

    fn lookup(&self, x: u64) -> bool {
        let idx = hash(x, self.bins);
        self.buckets[idx].contains(&x)
    }

    fn delete(&mut self, x: u64) {
        let idx = hash(x, self.bins);
        if let Some(pos) = self.buckets[idx].iter().position(|&v| v == x) {
            self.buckets[idx].swap_remove(pos);
            self.elements -= 1;
        }
    }

    fn load_factor(&self) -> f64 {
        self.elements as f64 / self.bins as f64
    }

    fn name() -> &'static str {
        "Veriženje"
    }
}

impl std::fmt::Display for ChainingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, bucket) in self.buckets.iter().enumerate() {
            write!(f, "  T[{i}]:")?;
            for &v in bucket {
                write!(f, " → {v}")?;
            }
            writeln!(f)?;
        }
        write!(f, "  α = {:.2}", self.load_factor())
    }
}

// ---------------------------------------------------------------------------
// 2. Odprto naslavljanje – linearno poizvedovanje
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Empty,
    Deleted,
    Occupied(u64),
}

struct LinearProbingTable {
    slots: Vec<Slot>,
    bins: usize,
    elements: usize,
}

impl LinearProbingTable {
    /// p(x, i) = (h(x) + i) mod m
    fn probe(&self, x: u64, i: usize) -> usize {
        (hash(x, self.bins) + i) % self.bins
    }
}

impl HashTable for LinearProbingTable {
    fn new(bins: usize) -> Self {
        Self {
            slots: vec![Slot::Empty; bins],
            bins,
            elements: 0,
        }
    }

    fn insert(&mut self, x: u64) {
        if self.elements == self.bins {
            return; // tabela polna
        }
        for i in 0..self.bins {
            let idx = self.probe(x, i);
            match self.slots[idx] {
                Slot::Empty | Slot::Deleted => {
                    self.slots[idx] = Slot::Occupied(x);
                    self.elements += 1;
                    return;
                }
                Slot::Occupied(v) if v == x => return, // že prisoten
                _ => {}
            }
        }
    }

    fn lookup(&self, x: u64) -> bool {
        for i in 0..self.bins {
            let idx = self.probe(x, i);
            match self.slots[idx] {
                Slot::Empty => return false,
                Slot::Occupied(v) if v == x => return true,
                _ => {}
            }
        }
        false
    }

    fn delete(&mut self, x: u64) {
        for i in 0..self.bins {
            let idx = self.probe(x, i);
            match self.slots[idx] {
                Slot::Empty => return,
                Slot::Occupied(v) if v == x => {
                    self.slots[idx] = Slot::Deleted;
                    self.elements -= 1;
                    return;
                }
                _ => {}
            }
        }
    }

    fn load_factor(&self) -> f64 {
        self.elements as f64 / self.bins as f64
    }

    fn name() -> &'static str {
        "Linearno poizvedovanje"
    }
}

impl std::fmt::Display for LinearProbingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, slot) in self.slots.iter().enumerate() {
            match slot {
                Slot::Empty => writeln!(f, "  T[{i}]: _")?,
                Slot::Deleted => writeln!(f, "  T[{i}]: ✗")?,
                Slot::Occupied(v) => writeln!(f, "  T[{i}]: {v}")?,
            }
        }
        write!(f, "  α = {:.2}", self.load_factor())
    }
}

// ---------------------------------------------------------------------------
// 3. Odprto naslavljanje – dvojno zgoščevanje
// ---------------------------------------------------------------------------

struct DoubleHashingTable {
    slots: Vec<Slot>,
    bins: usize,
    elements: usize,
}

impl DoubleHashingTable {
    /// h1(x) = x mod m
    fn h1(&self, x: u64) -> usize {
        (x as usize) % self.bins
    }

    /// h2(x) = 1 + (x mod (m - 1))   (zagotovi h2 ≥ 1 in gcd(h2, m) = 1 za m praštevilo)
    fn h2(&self, x: u64) -> usize {
        1 + (x as usize) % (self.bins - 1)
    }

    /// h(x, i) = (h1(x) + i · h2(x)) mod m
    fn probe(&self, x: u64, i: usize) -> usize {
        (self.h1(x) + i * self.h2(x)) % self.bins
    }
}

impl HashTable for DoubleHashingTable {
    /// bins mora biti praštevilo
    fn new(bins: usize) -> Self {
        Self {
            slots: vec![Slot::Empty; bins],
            bins,
            elements: 0,
        }
    }

    fn insert(&mut self, x: u64) {
        if self.elements == self.bins {
            return;
        }
        for i in 0..self.bins {
            let idx = self.probe(x, i);
            match self.slots[idx] {
                Slot::Empty | Slot::Deleted => {
                    self.slots[idx] = Slot::Occupied(x);
                    self.elements += 1;
                    return;
                }
                Slot::Occupied(v) if v == x => return,
                _ => {}
            }
        }
    }

    fn lookup(&self, x: u64) -> bool {
        for i in 0..self.bins {
            let idx = self.probe(x, i);
            match self.slots[idx] {
                Slot::Empty => return false,
                Slot::Occupied(v) if v == x => return true,
                _ => {}
            }
        }
        false
    }

    fn delete(&mut self, x: u64) {
        for i in 0..self.bins {
            let idx = self.probe(x, i);
            match self.slots[idx] {
                Slot::Empty => return,
                Slot::Occupied(v) if v == x => {
                    self.slots[idx] = Slot::Deleted;
                    self.elements -= 1;
                    return;
                }
                _ => {}
            }
        }
    }

    fn load_factor(&self) -> f64 {
        self.elements as f64 / self.bins as f64
    }

    fn name() -> &'static str {
        "Dvojno zgoščevanje"
    }
}

impl std::fmt::Display for DoubleHashingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, slot) in self.slots.iter().enumerate() {
            match slot {
                Slot::Empty => writeln!(f, "  T[{i}]: _")?,
                Slot::Deleted => writeln!(f, "  T[{i}]: ✗")?,
                Slot::Occupied(v) => writeln!(f, "  T[{i}]: {v}")?,
            }
        }
        write!(f, "  α = {:.2}", self.load_factor())
    }
}

// ---------------------------------------------------------------------------
// Enostaven LCG generator (da ne potrebujemo zunanjih odvisnosti)
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        // Knuth LCG
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
}

// ---------------------------------------------------------------------------
// Benchmarking
// ---------------------------------------------------------------------------

const ELEMENTS: usize = 100_000;

fn bench_insert<T: HashTable>(bins: usize, keys: &[u64]) {
    let mut table = T::new(bins);
    let start = Instant::now();
    for &k in keys {
        table.insert(k);
    }
    let insert_time = start.elapsed();

    let start = Instant::now();
    let mut found = 0usize;
    for &k in keys {
        if table.lookup(k) {
            found += 1;
        }
    }
    let lookup_time = start.elapsed();

    println!(
        "  {:<25} | insert: {:>8.2?} | lookup: {:>8.2?} | α = {:.2} | najdenih: {}/{}",
        T::name(),
        insert_time,
        lookup_time,
        table.load_factor(),
        found,
        keys.len(),
    );
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Benchmark: {ELEMENTS} naključnih ključev ===");

    let mut rng = Rng::new(42);
    let random_keys: Vec<u64> = (0..ELEMENTS).map(|_| rng.next_u64()).collect();

    // Veriženje: bins ≈ elements (α ≈ 1)
    bench_insert::<ChainingTable>(ELEMENTS, &random_keys);
    // Veriženje: bins ≈ elements/2 (α ≈ 2)
    bench_insert::<ChainingTable>(ELEMENTS / 2, &random_keys);

    // Odprto naslavljanje: bins ≈ 2·elements (α ≈ 0.5) — 200003 je praštevilo
    bench_insert::<LinearProbingTable>(200_003, &random_keys);
    bench_insert::<DoubleHashingTable>(200_003, &random_keys);

    // Odprto naslavljanje: bins ≈ 4·elements/3 (α ≈ 0.75) — 133_003 je praštevilo
    bench_insert::<LinearProbingTable>(133_003, &random_keys);
    bench_insert::<DoubleHashingTable>(133_003, &random_keys);
}