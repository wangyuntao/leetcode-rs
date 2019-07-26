use std::cell::RefCell;
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

pub struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type Rn = Rc<RefCell<TreeNode>>;

        fn f(p: Option<&Rn>, q: Option<&Rn>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    p.val == q.val
                        && f(p.left.as_ref(), q.right.as_ref())
                        && f(p.right.as_ref(), q.left.as_ref())
                }
                _ => false,
            }
        }

        match root {
            None => true,
            Some(n) => {
                let n = n.borrow();
                f(n.left.as_ref(), n.right.as_ref())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_symmetric(None));
    }
}
