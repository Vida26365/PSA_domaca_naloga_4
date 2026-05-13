use crate::hash_tabela::PerfectHashSet;

pub type Key = u64;

/// Trait that defines operations needed for a structure to be used with Bentley-Saxe construction
/// Any data structure implementing this trait can be made semi-dynamic using BentleySaxe
pub trait DynamicStructure: Sized {
    /// Searches for a key in the structure
    fn contains(&self, key: Key) -> bool;
    
    /// Creates a new structure from a slice of keys
    /// This is used during merging to build new structures
    fn from_keys(keys: &[Key]) -> Self;
    
    /// Returns the number of elements in the structure
    /// Used for tracking purposes (optional, can just return a dummy value)
    fn len(&self) -> usize {
        0
    }
    
    /// Checks if the structure is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Extracts and returns all elements stored in the structure
    /// Returns a vector containing all keys in this structure
    fn get_elements(&self) -> Vec<Key>;
}

/// Implementation of DynamicStructure for PerfectHashSet
impl DynamicStructure for PerfectHashSet {
    fn contains(&self, key: Key) -> bool {
        PerfectHashSet::contains(self, key)
    }
    
    fn from_keys(keys: &[Key]) -> Self {
        PerfectHashSet::new(keys)
    }
    
    fn len(&self) -> usize {
        PerfectHashSet::len(self)
    }
    
    fn get_elements(&self) -> Vec<Key> {
        self.extract_keys()
    }
}

/// Generic Bentley-Saxe construction that works with any structure implementing DynamicStructure
/// 
/// Maintains multiple levels, where level i contains 2^i elements.
/// When inserting, merge levels as needed (similar to binary addition).
/// 
/// - Insertion: O(log n) amortized
/// - Search: O(log n) - searches through at most log n levels
/// - Space: O(n) same as storing all elements
#[derive(Debug)]
pub struct BentleySaxe<S: DynamicStructure> {
    /// levels[i] contains a structure with 2^i elements, or None if empty
    levels: Vec<Option<S>>,

    /// Total number of unique elements in the structure
    size: usize,
}

impl<S: DynamicStructure> BentleySaxe<S> {
    /// Creates a new empty Bentley-Saxe structure
    pub fn new() -> Self {
        BentleySaxe {
            levels: Vec::new(),
            size: 0,
        }
    }

    /// Inserts a new element into the structure
    /// Time complexity: O(log n) amortized (O(log n) dup check + O(n log n) merge)
    pub fn insert(&mut self, key: Key) {
        // Don't insert duplicates - use search() for O(log n) lookup
        if self.search(key) {
            return;
        }

        self.size += 1;

        // Promote through levels (similar to binary addition)
        self.promote_level(vec![key], 0);
    }

    /// Recursively promotes and merges structures through levels
    fn promote_level(&mut self, mut elements: Vec<Key>, level: usize) {
        // level ... v kater level ustavljamo strukturo

        if level == self.levels.len() {
            self.levels.push(None); // dodamo nov level za v prihodnost
        }

        // If current level is empty, place the structure there
        if self.levels[level].is_none() {
            let structure = S::from_keys(&elements);
            self.levels[level] = Some(structure);
        } else {
            let existing_structure = self.levels[level].take().unwrap();
            elements.extend(existing_structure.get_elements());
            self.promote_level(elements, level + 1);
        }
    }

    /// Searches for an element in the structure
    /// Time complexity: O(log n) - gre čez log n levlov v O(1) na level
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

    /// Returns the number of levels currently used
    pub fn num_levels(&self) -> usize {
        self.levels.iter().filter(|l| l.is_some()).count()
    }

    /// Returns the total number of elements in the structure
    /// Time complexity: O(1)
    pub fn len(&self) -> usize {
        self.size
    }

    /// Checks if the structure is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Rebuilds the entire structure to optimize space
    /// Useful after many insertions to consolidate levels
    /// Time complexity: O(n log n)
    pub fn rebuild(&mut self) {
        let elements = self.collect_all_elements();
        *self = Self::new();
        for elem in elements {
            self.insert(elem);
        }
    }

    /// Collects all elements from non-empty levels.
    fn collect_all_elements(&self) -> Vec<Key> {
        let mut elements = Vec::with_capacity(self.size);
        for level in &self.levels {
            if let Some(structure) = level {
                elements.extend(structure.get_elements());
            }
        }
        elements
    }
}

impl<S: DynamicStructure> Default for BentleySaxe<S> {
    fn default() -> Self {
        Self::new()
    }
}
