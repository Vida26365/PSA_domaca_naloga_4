use crate::hash_tabela::PerfectHashSet;
pub type BentleySaxeDynamicHashSet = BentleySaxe<PerfectHashSet>;

pub type Key = u64;

/// Lastnosti, ki morajo veljati za implementacijo statične strukture v dinamično z Bently-Saxovo transformacijo
pub trait DynamicStructure: Sized {
    /// Ali je element vsebovan
    fn contains(&self, key: Key) -> bool;
    
    /// Ustvari novo strukturo iz elementov
    fn from_keys(keys: &[Key]) -> Self;
    
    
    /// Preveri če je struktura prazna
    fn is_empty(&self) -> bool;
    
    /// Vrne vse elemente v strukturi
    fn get_elements(&self) -> Vec<Key>;
}

/// DynamicStructure za PerfectHashSet
impl DynamicStructure for PerfectHashSet {
    fn contains(&self, key: Key) -> bool {
        PerfectHashSet::contains(self, key)
    }
    
    fn from_keys(keys: &[Key]) -> Self {
        PerfectHashSet::new(keys)
    }
    
    fn is_empty(&self) -> bool {
        PerfectHashSet::is_empty(&self)
    }
    
    fn get_elements(&self) -> Vec<Key> {
        self.extract_keys()
    }
}


/// Bantly-Saxe struktura:
/// vsak nivo strukture je lahko prazen, ali pa je tam zgoščena tabela velikosti 2^i
/// [ . ]
/// [ . . ]
/// [ . . . .]
/// [ . . . . . . . .]
/// Implementacija podpira transformacijo poljubne strukture, ki implementira DynamicStructure. 
/// V nadaljevanju bo ime tabela označevalo statično strukturo, struktura pa Bently-Saxovo transformacijo te statične strukture
#[derive(Debug)]
pub struct BentleySaxe<S: DynamicStructure> {
    levels: Vec<Option<S>>, // vektor tabel
    size: usize, // število vseh elementov
}

impl<S: DynamicStructure> BentleySaxe<S> {
    pub fn new() -> Self {
        BentleySaxe {
            levels: Vec::new(),
            size: 0,
        }
    }

    
    pub fn insert(&mut self, key: Key) {

        // Pregled, če je ključ že v strukturi
        if self.search(key) {
            return;
        }

        self.size += 1;

        // Poskrbi za nivojanje
        self.promote_level(vec![key], 0);
    }

    /// Gre čez nivoje in jih sporti prazni, dokler ne naleti na prazen nivo. 
    /// Tja postavi tabelo z vsemi elementi iz nižjih nivojev
    fn promote_level(&mut self, mut elements: Vec<Key>, level: usize) {
        // level ... v kater nivo ustavljamo strukturo

        if level == self.levels.len() {
            self.levels.push(None); // dodamo nov level za v prihodnost
        }

        // Ko najdemo prazen nivo, naredimo tabelo iz elementov, ki smo jih sproti nabrali 
        if self.levels[level].is_none() {
            let structure = S::from_keys(&elements);
            self.levels[level] = Some(structure);
        } else {
            // Če je nivo poln, ga izpraznimo
            let existing_structure = self.levels[level].take().unwrap();
            elements.extend(existing_structure.get_elements());
            self.promote_level(elements, level + 1);
        }
    }

    /// Iskanje elementa v strukturi
    pub fn search(&self, key: Key) -> bool {
        for level_structure in &self.levels {
            if let Some(structure) = level_structure {
                if structure.contains(key) {
                    return true;
                }
            }
        }
        false
    }

}


