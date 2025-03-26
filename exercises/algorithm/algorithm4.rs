/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug,Clone)]
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
    fn search_node(&self, value: T) -> bool {
    	if self.value==value {
    		return true;
    	}else if self.value<value {
            return self.right.is_some()&&self.right.as_ref().unwrap().search_node(value);
        }else{
            return self.left.is_some()&&self.left.as_ref().unwrap().search_node(value);  
        }
    	}
    fn insert_node(&mut self, newNode: Box<TreeNode<T>>) {
        if newNode.value < self.value {
            if let Some(ref mut left) = self.left {
                left.insert_node(newNode);
            } else {
                self.left = Some(newNode);
            }
        } else if newNode.value > self.value {
            if let Some(ref mut right) = self.right {
                right.insert_node(newNode);
            } else {
                self.right = Some(newNode);
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
        let node = Box::new(TreeNode::new(value));
        if let Some(ref mut root) = self.root {
            root.insert_node(node);
        } else {
            self.root = Some(node);
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if self.root.is_none(){
            return false;
        }
         let root=self.root.as_ref().unwrap();
         return root.search_node(value);
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
// fn main() {
//     println!("Hello, world!");
// }


