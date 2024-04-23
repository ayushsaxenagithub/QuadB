// Given a binary tree, implement a function that returns the maximum depth of the tree.



use std::io;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            1 + std::cmp::max(
                max_depth(node.borrow().left.clone()),
                max_depth(node.borrow().right.clone()),
            )
        }
    }
}

fn main() {
    // Create a simple example tree: [3,9,20,null,null,15,7]
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));

    println!("The maximum depth of the tree is {}.", max_depth(root));
}

