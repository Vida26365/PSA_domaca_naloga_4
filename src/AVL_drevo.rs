#[path = "Hash_tabela.rs"]
pub mod hash_tabela;

#[path = "Bently-Saxe.rs"]
pub mod bentley_saxe;

pub type Key = usize;

#[derive(Debug)]
struct Node {
	key: Key,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>,
	height: i32,
}

impl Node {
	fn new(key: Key) -> Self {
		Self {
			key,
			left: None,
			right: None,
			height: 1,
		}
	}
}

#[derive(Debug, Default)]
pub struct AvlTree {
	root: Option<Box<Node>>,
}

impl AvlTree {
	pub fn new() -> Self {
        AvlTree { root: None }
    }

	pub fn insert(&mut self, key: Key) {
		self.root = Self::insert_node(self.root.take(), key);
	}

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

	fn insert_node(node: Option<Box<Node>>, key: Key) -> Option<Box<Node>> {
		match node {
			None => Some(Box::new(Node::new(key))),
			Some(mut current) => {
				if key < current.key {
					current.left = Self::insert_node(current.left.take(), key);
				} else if key > current.key {
					current.right = Self::insert_node(current.right.take(), key);
				} else {
					return Some(current);
				}

				Self::update_height(&mut current);
				Some(Self::rebalance(current))
			}
		}
	}

	fn rebalance(mut node: Box<Node>) -> Box<Node> {
		let balance = Self::razlika_leva_desna(&node);

		if balance > 1 {
			let left_balance = node
				.left
				.as_ref()
				.map_or(0, |left| Self::razlika_leva_desna(left));

			if left_balance < 0 {
				let left = node.left.take().expect("left child must exist");
				node.left = Some(Self::rotate_left(left));
			}

			return Self::rotate_right(node);
		}

		if balance < -1 {
			let right_balance = node
				.right
				.as_ref()
				.map_or(0, |right| Self::razlika_leva_desna(right));

			if right_balance > 0 {
				let right = node.right.take().expect("right child must exist");
				node.right = Some(Self::rotate_right(right));
			}

			return Self::rotate_left(node);
		}

		node
	}

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

	fn update_height(node: &mut Box<Node>) {
		node.height = 1 + Self::height(&node.left).max(Self::height(&node.right));
	}

	fn height(node: &Option<Box<Node>>) -> i32 {
		node.as_ref().map_or(0, |n| n.height)
	}

	fn razlika_leva_desna(node: &Node) -> i32 {
		Self::height(&node.left) - Self::height(&node.right)
	}
}

#[cfg(test)]
mod tests {
	use super::AvlTree;

	#[test]
	fn insert_and_search_work() {
		let mut tree = AvlTree::new();
		let values = [10, 20, 30, 40, 50, 25];

		for value in values {
			tree.insert(value);
		}

		assert!(tree.search(25));
		assert!(tree.search(10));
		assert!(!tree.search(99));
	}

	#[test]
	fn duplicate_insert_is_ignored() {
		let mut tree = AvlTree::new();
		tree.insert(7);
		tree.insert(7);

		assert!(tree.search(7));
		assert!(!tree.search(8));
	}
}

