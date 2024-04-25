/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
	pub fn insert_node(u: Option<Box<Self>>, value: T) -> Option<Box<Self>> {
		match u {
			None => {
				Some(Box::new(Self::new(value)))
			},
			Some(mut u) => {
				if value < u.value {
					u.left = Self::insert_node(u.left, value);
				} else {
					u.right = Self::insert_node(u.right, value);
				}
				Some(u)
			}
		}
	}
	pub fn search_node(u: &Option<Box<Self>>, value: T) -> bool {
		match u {
			None => {
				false
			},
			Some(u) => {
				if value == u.value {
					true
				} else if value < u.value {
					Self::search_node(&u.left, value)
				} else {
					Self::search_node(&u.right, value)
				}
			}
		}
	}
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
		self.root = TreeNode::insert_node(self.root, value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
		match self.root {
			None => {
				false
			},
			Some(u) => {
				TreeNode::search_node(&self.root, value)
			}
		}
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


