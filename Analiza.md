# Podrobna analiza časovne kompleksnosti
To je analiza časovne zahtevnosti. Za ogled primerjave rezultatov si poglejte datoteko [rezultati.ipynb](rezultati.ipynb).


## Hash tabela
Pri Bently-Saxovi konstrukciji se uporabi le funkcije, ki jih implementira lastnost `DynamicStructure` 

### `contains` (Ali je element vsebovan v strukturi)

Ker uporabljamo hash funkcijo, ki deluje v O(1), da najdemo pravo vedro in potem še eno hash funkcijo, da najdemo mesto kjer bi element moral bitit, lahko v O(1) Preverimo, če je element vsebovan v strukturi ali ne.

``` rs
pub fn contains(&self, key: u64) -> bool {
	if self.buckets.is_empty() { // O(1)
		return false;
	}

	let bucket_index = self.top_hash.apply(key); // O(1)
	self.buckets[bucket_index].contains(key) // O(1)
}
```
``` rs
impl Bucket {
    fn contains(&self, key: u64) -> bool {
		match self {
			Self::Empty => false, 
			Self::Single(existing) => *existing == key, 
			Self::Table { hash, slots } => {
				if slots.is_empty() {  // O(1)
					return false;
				}

				let index = hash.apply(key); // O(1)
				slots[index] == Some(key) 
			}
		}
	}
}
```

### `from_keys` (Sestavimo strukturo iz elementov)

`PrfectHashSet` implementira funkcijo `from_keys` s funkcijo `new: &[u64] -> PrfectHashSet`.
V funkciji `new` najprej preverimo, da se elementi iz katerih želimo sestaviti tabelo ne ponavljajo. To opravimo v $ O(n \cdot \log (n)) $ časa
``` rs
pub fn new(keys: &[u64]) -> Self {
		let mut unique: Vec<u64> = keys.to_vec();
		unique.sort_unstable();  // O(n * log n)
		unique.dedup(); // O(n)
    ...
}
```
Nato poskušamo najti hash funkcijo, ki bo razporedila elemente v vedra tako, da ne bo preveč elementov v istih vedrih. Pogoj za to da je tabela dovolj dobra izberemo $ \sum_{i}^{m} \text{len}(vedro_i)^2 > 4 * n $, kjer je $m$ število prostorov v vrhnji tabeli in $n$ število vseh elementov. V našem primeru je $ n = m $
``` rs
for _ in 0..MAX_TOP_LEVEL_TRIES {
	let top_hash = random_hash(m); // O(1)
	let mut partitioned = vec![Vec::<u64>::new(); m]; // O(m)

	for &key in &unique {
		partitioned[top_hash.apply(key)].push(key); 
	}

	// O(Σ len(vedro_i)) = O(n)
	let sum_of_squares: usize = partitioned 
		.iter()
		.map(|bucket| bucket.len() * bucket.len())
		.sum();

	if sum_of_squares > 4 * n { 
		continue;
	}
	...
}
```
Ko najdemo vrhno hash funkcijo, lahko gremo iskati hash funkcije za vedra, ki nimajo trkov. Funkcija `build_bucket` sestavi vedro z hash funkcijo brez trkov. Če ne uspe sestaviti take funkcije, vrne `None`, vendar je vrjetnost da se to zgodi zanemarljiva. 
``` rs
let mut buckets = Vec::with_capacity(m);
let mut success = true;

for stvari_v_vedru in partitioned { // Preverimo da ni trkov
	match build_bucket(&stvari_v_vedru) { // E * O(len(vedro_i))
		Some(bucket) => buckets.push(bucket), 
		None => {
			success = false;
			break;
		}
	}
}
```
Ker je pričakovana vrednost ustavitve zanke $ O(1) $ je časovna zahtevost funkcije `new` enaka $ O( n \cdot \log(n) + n + m + m \cdot \text{len}({vedro_i})) = O( n \cdot log n) $


### `len` (Koliko elementov je v strukturi)
V strukturi `PrfectHashSet` je vrednost `size` shranjen kot polje. Zato funkcija `len` deluje v $ O(1) $
``` rs
pub struct PerfectHashSet {
	size: usize,
	top_hash: UniversalHash,
	buckets: Vec<Bucket>,
}
```

### `extraxt_keys` (Vrne vektor elementov, ki so v strukturi)
Eno izmed polj v strukturi `PerfectHashSet` je `keys`, ki shranjuje vektor vseh elementov v strukturi. Torej da vrnemo ključe je čas $ O(n) $, saj kopiramo vektor.




## Bently-Saxova transformacija hash tabele
Implementirana Bently-Sax struktura `BentleySaxe` vsebuja dva polja: 
	- nivoje (`levels:  Vec<Option<S>>`)
	- vse elemente: (all_elements: Vec<Key>)
Polja za vse elemente ne potrebujemo. Če bi lastnost `DynamicStructure` zahtevala še fukcijo `get_elements`, ki vrne nek iterator elementov strukture. 

Poglejmo si časovno zahtevbost iskanja in odajanja elementov.

### `search` (Najdi element)
Število nivojev je $ O( \log(n)) $, ker $i$-ti nivo vsebuje strukturo, ki vsebuje $2^i$ elementov, ali pa jih ne vsebuje. V vseh nivojih je skupaj $n$ elementov, torej bo največ nivojev, če bodo vsi elementi v najvišjem nivoju. Torej, či bi bilo $n = 2^{\text{št nivojev}} $. Sledi da je število nivojev največ $ \log(n) $. 
``` rs
pub fn search(&self, key: Key) -> bool {
	for level_structure in &self.levels { // O(log(n))
		if let Some(structure) = level_structure {
			if structure.contains(key) { // O(1)
				return true;
			}
		}
	}
	false
}
```
Za funckcijo `containes` smo že zgoraj pokazali, da ima časovno zahtevnost $O(1)$ uporabimo pa jo $ \log(n) $-krat, torej je časovna zahtevnost za `search` $ O(\log(n)) $

### insert (Dodaj element)

Ko vstavljamo element v Bently-Saxovo strukturo, ga poskusimo njaprej vstaviti na najnižji nivo. Če je ta nivo že zaseden, vzamemo element in z+nivoja in element, ki ga želimo vstaviti, in pogledamo, če ju lahko skupaj damo v strukturo in postavimo v naslednji nivo (s funkcijo `promote_level`). 
``` rs
pub fn insert(&mut self, key: Key) {
	if self.search(key) {  // O(log n)
		return; // da nimamo ponovitev
	}

	self.all_elements.push(key);

	let new_structure = S::from_keys(&[key]);

	self.promote_level(new_structure, 0);
}
```
Če drugi nivo ni praznen, poberemo elementa iz drugega nivoja, in pogledamo, če je tretji nivo prazen, in če je, lahko vse elemente prvega in drugega nivoja združimo v hash tabelo in jo postavimo kot tretji nivo strukture. Tako nadaljujemo gor po nivojih dokler ne najdemo nivoja, ki je prazen. Vsi nivoji pod tem nivojem, pa bodo po vstavljanju postali prazni.

``` rs
fn promote_level(&mut self, mut elements: Vec<Key>, level: usize) {
	// level ... v kater level ustavljamo strukturo

	if level == self.levels.len() {
		self.levels.push(None); // dodamo nov level za v prihodnost
	}

	// Če je level prazen, smo našli mesto za vstavitit strukturo
	if self.levels[level].is_none() {
		let structure = S::from_keys(&elements); // O(2^{level} * log (2 * 2^{level})) (zaradi urejanja elementov)
		self.levels[level] = Some(structure);
	} else {
		let existing_structure = self.levels[level].take().unwrap();
		elements.extend(existing_structure.get_elements()); // O(2^{level})
		self.promote_level(elements, level + 1);
	}
}
```
`promote_level` je rekurzivna funkcija. Največ časa bi potrebovala, če bi bili vsi nivoji zasedeni. Torej 
$$ 
O(2^{\log(n)} \cdot \log( 2 \cdot 2^{\log(n)})) + \sum_{i=1}^{\log(n)}2^i = O(n \cdot \log(n) + n) = O(n \cdot \log(n)).
$$
Ampak amortiziano po skoraj enakem postopku kot iz predavanj dobimo:
$$
C \cdot n =n + \sum_{i = 1}^{\lfloor \log(n) \rfloor} 2^i \cdot {\log(2^i)} = 
n + \sum_{i = 1}^{\lfloor \log(n) \rfloor} 2^i \cdot i = 
2 (1 - 2^{\log(n)} + 2^{\log(n)} \cdot \log(n)) \leq 2 + 2 n \log(n)
$$
Zato je amortiziran čas vstavljanja $ O(\log(n)) $


## AVL drevo

### `search` (Iskanje node v drevesu)
Drevo je urejeno, tako da je levi otrok vedno manjši od starša in desni otrok večji. Zato iskanje poteka v $ O(\log(n)) $, kr je globina drevesa $\log(n)$
``` rs
pub fn search(&self, key: Key) -> bool {
	let mut current = self.root.as_ref();

	while let Some(node) = current {
		if key == node.key {
			return true;
		}

		current = if key < node.key {
			node.left.as_ref()
		} else {
			node.right.as_ref()
		};
	}

	false
}
```

### `insert` (Vstavi novo nodo v drevo)

Vstaljanje deluje tako, da najprej najdemo mesto, kamor bomo vstavili nodo. Torej se sprehodimo do dna drevesa, kar vzame $ O(\log(n)) $ časa. Nato pa moramo še poskrbeti, da drevo ostane uravnoteženo, kar naredimo z funkcijo `rebalance` za vsakega prednika vstavljene node.

Najprej preverimo, če je drevo iz node neuravnoteeno in v katero smer je neuravnoteženo. Vsaka noda hrani velikost drevesa, ki se začne iz te node, zato lahko v $ O(1) $ Najdemo ralliko v višini levega in desnega otroka node
``` rs
fn razlika_leva_desna(node: &Node) -> i32 {
	Self::height(&node.left) - Self::height(&node.right)
}
```
Če je potrebna rotacija, ločimo primera, če rotiramo levo ali desno.
``` rs
	fn rotate_left(mut z: Box<Node>) -> Box<Node> {
//	z < t2 < y	
//
//		z				y
//	   / \			   / \
//		  y	   -->	  z	  
//		 / \		 / \	
//	   t2				t2
//
		let mut y = z.right.take().expect("right child must exist");
		let t2 = y.left.take();

		z.right = t2;
		Self::update_height(&mut z); 

		y.left = Some(z);
		Self::update_height(&mut y);

		y
	}

	fn rotate_right(mut z: Box<Node>) -> Box<Node> {
//	y < t3 < z	
//
//		z				y
//	   / \			   / \
//	  y   	   -->	      z	  
//	 / \   			 	 / \	
//	   t3			    t3
//
		let mut y = z.left.take().expect("left child must exist");
		let t3 = y.right.take();

		z.left = t3;
		Self::update_height(&mut z);

		y.right = Some(z);
		Self::update_height(&mut y);

		y
	}
```
Vidimo, da rotacije zahtevajo le spreminjanje velikosti in spreminjanje levega in desnega otroka. Vse to poteka v $ O(1) $, saj si vsaka noda hrani višino.
``` rs
fn update_height(node: &mut Box<Node>) {
	node.height = 1 + Self::height(&node.left).max(Self::height(&node.right));
}
```
Zato je časovna zahtevnost vstavljanja $ \log(n) \cdot O(1) = O(\log(n)) $

