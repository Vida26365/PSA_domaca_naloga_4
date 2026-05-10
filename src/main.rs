use avl_drevo::AvlTree;
use std::time::{Duration, Instant};

fn time_function<F, T>(function: F) -> (T, Duration)
where
	F: FnOnce() -> T,
{
	let start = Instant::now();
	let result = function();
	(result, start.elapsed())
}

fn main() {
	const N: usize = 1000;
	let ((), elapsed) = time_function(|| {
		let mut tree = AvlTree::new();

		for key in 0..N {
			tree.insert(key);
		}
	});

	println!("Inserted {N} nodes in {:?}", elapsed);
}
