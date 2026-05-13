#[path = "Hash_tabela.rs"]
pub mod hash_tabela;

#[path = "Bently-Saxe.rs"]
pub mod bentley_saxe;

#[path = "AVL_drevo.rs"]
pub mod avl_drevo;


use bentley_saxe::BentleySaxeDynamicHashSet;
use avl_drevo::AvlTree;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Write;
use rand::seq::SliceRandom;



fn time_function<F, T>(function: F) -> (T, Duration)
where
	F: FnOnce() -> T,
{
	let start = Instant::now();
	let result = function();
	(result, start.elapsed())
}

fn generate_random_numbers(n: usize) -> Vec<usize> {
	let mut numbers: Vec<usize> = (1..=n).collect();
	let mut rng = rand::thread_rng();
	numbers.shuffle(&mut rng);
	numbers
}

fn generate_csv(path: &str, max_n: usize) -> std::io::Result<()> {
	let mut file = File::create(path)?;
	writeln!(
		file,
		"n,avl_insert_ns,avl_search_ns,bentley_insert_ns,bentley_search_ns,speed_insert_ns,speed_search_ns"
	)?;

	for n in (100..=max_n).step_by(100) { 
        let m = n/2;
		let random_numbers = generate_random_numbers(n);
		let search_numbers = generate_random_numbers(m);
		
		// AVL tree
		let (avl_tree, avl_insert_elapsed) = time_function(|| {
			let mut avl_tree = AvlTree::new();
			for &key in &random_numbers {
				avl_tree.insert(key);
			}
			avl_tree
		});
		
		let (_, avl_search_elapsed) = time_function(|| {
			for &key in &search_numbers {
				let _ = avl_tree.search(key);
			}
		});
		
		// Bentley-Saxe
		let (bentley_saxe_set, bentley_insert_elapsed) = time_function(|| {
			let mut dynamic_hash_set = BentleySaxeDynamicHashSet::new();
			for &key in &random_numbers {
				dynamic_hash_set.insert(key as u64);
			}
			dynamic_hash_set
		});
		
		let (_, bentley_search_elapsed) = time_function(|| {
			for &key in &search_numbers {
				let _ = bentley_saxe_set.search(key as u64);
			}
		});
		
		// SpeedPerfectHashSet
		// (removed - file doesn't exist yet)
		
		writeln!(
			file,
			"{},{},{},{},{}",
			n,
			avl_insert_elapsed.as_nanos(),
			avl_search_elapsed.as_nanos(),
			bentley_insert_elapsed.as_nanos(),
			bentley_search_elapsed.as_nanos(),
		)?;
	}
	
	Ok(())
}

fn main() {
	if let Err(error) = generate_csv("data.csv", 100000) {
		eprintln!("failed to write benchmark csv: {error}");
		std::process::exit(1);
	}
}
