use avl_drevo::AvlTree;

fn main() {
	const N: usize = 1000;
	let mut tree = AvlTree::new();

	for key in 0..N {
		tree.insert(key);
	}
}
