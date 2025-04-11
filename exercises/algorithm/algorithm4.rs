/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/
// Implements the static search_node method for BinarySearchTree

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
        match &mut self.root {
            None => {
            // If the tree is empty, create a new root node
            self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(node) => {
            // Otherwise, insert the value into the existing tree
            node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => false,
            Some(node) => Self::search_node(node, &value)
        }
    }

    // Helper function to search in a node and its subtrees
    fn search_node(node: &TreeNode<T>, value: &T) -> bool {
        match value.cmp(&node.value) {
            Ordering::Equal => true,
            Ordering::Less => {
                match &node.left {
                    None => false,
                    Some(left) => Self::search_node(left, value)
                }
            },
            Ordering::Greater => {
                match &node.right {
                    None => false,
                    Some(right) => Self::search_node(right, value)
                }
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
        match self.value.cmp(&value) {
            Ordering::Greater => {
                // If value is less than current node's value, go left
                match self.left {
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                    Some(ref mut left) => left.insert(value),
                }
            }
            Ordering::Less => {
                // If value is greater than current node's value, go right
                match self.right {
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                    Some(ref mut right) => right.insert(value),
                }
            }
            Ordering::Equal => {
                // If value is equal, do nothing (no duplicates)
            }
        }
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


