
pub mod prelude;
pub mod _94;
pub mod _95;
pub mod _96;
pub mod _98;
use std::ops::Deref;
use self::prelude::*;
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
            val: val,
            left: None,
            right: None,
        }
    }
}

