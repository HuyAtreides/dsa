use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    let mut result = 0;

    while !queue.is_empty() {
        let level_count = queue.len();
        let mut level_ops = 0;
        let mut sorted: Vec<i32> = Vec::with_capacity(level_count);
        let mut original: Vec<i32> = Vec::with_capacity(level_count);
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..level_count {
            if let Some(node) = queue.pop_front() {
                sorted.push(node.borrow().val);
                original.push(node.borrow().val);
                map.insert(node.borrow().val, i);

                if let Some(left_node) = node.borrow().left.clone() {
                    queue.push_back(left_node);
                }

                if let Some(right_node) = node.borrow().right.clone() {
                    queue.push_back(right_node);
                }
            }
        }

        sorted.sort();

        for i in 0..original.len() {
            if original[i] != sorted[i] {
                level_ops += 1;
                let v = sorted[i];
                let index = *map.get(&v).unwrap();
                map.insert(original[i], index);
                map.insert(sorted[i], i);
                original.swap(i, index);
            }
        }

        result += max(0, level_ops);
    }

    result
}
