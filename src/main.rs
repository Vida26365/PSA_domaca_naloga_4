#[path = "Hash_tabela.rs"]
pub mod hash_tabela;

#[path = "Bently-Saxe.rs"]
pub mod bentley_saxe;

#[path = "AVL_drevo.rs"]
pub mod avl_drevo;

use bentley_saxe::BentleySaxeDynamicHashSet;
use avl_drevo::AvlTree;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;

fn time_function<F, T>(function: F) -> (T, Duration)
where
	F: FnOnce() -> T,
{
	let start = Instant::now();
	let result = function();
	(result, start.elapsed())
}

/// Generates N random non-repeating numbers
fn generate_random_numbers(n: usize) -> Vec<usize> {
	let mut numbers: Vec<usize> = (0..n).collect();
	let mut rng = rand::thread_rng();
	numbers.shuffle(&mut rng);
	numbers
}

/// Compares insertion performance between AVL_tree and Bentley-Saxe structures
fn compare_structures(n: usize) {
	println!("=== Comparing insertion performance with N = {} ===\n", n);
	
	// Generate random non-repeating numbers
	let random_numbers = generate_random_numbers(n);
	
	// Test AVL_tree insertions
	let (_, avl_elapsed) = time_function(|| {
		let mut avl_tree = AvlTree::new();
		for &key in &random_numbers {
			avl_tree.insert(key);
		}
	});
	println!("AVL_tree: Inserted {} elements in {:?}", n, avl_elapsed);
	
	// Test Bentley-Saxe with HashTableSet insertions
	let (_, bentley_saxe_elapsed) = time_function(|| {
		let mut dynamic_hash_set = BentleySaxeDynamicHashSet::new();
		for &key in &random_numbers {
			dynamic_hash_set.insert(key as u64);
		}
	});
	println!("Bentley-Saxe Dynamic HashSet: Inserted {} elements in {:?}", n, bentley_saxe_elapsed);
	
	// Show comparison
	println!("\nComparison:");
	if avl_elapsed > bentley_saxe_elapsed {
		let ratio = avl_elapsed.as_secs_f64() / bentley_saxe_elapsed.as_secs_f64();
		println!("Bentley-Saxe is {:.2}x faster", ratio);
	} else {
		let ratio = bentley_saxe_elapsed.as_secs_f64() / avl_elapsed.as_secs_f64();
		println!("AVL_tree is {:.2}x faster", ratio);
	}
}

fn main() {
	const N: usize = 1000;
	compare_structures(N);
}
