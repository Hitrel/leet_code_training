use super::prelude::*;

use super::TreeNode;

struct Solution {

}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Solution::helper(root.as_ref())
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let mut left_vec = Solution::helper(node.borrow().left.as_ref());
                left_vec.push(node.borrow().val);
                let mut right_vec = Solution::helper(node.borrow().right.as_ref());
                left_vec.append(&mut right_vec);

                left_vec
            },
            None => vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use super::super::{TreeNode, prelude::*};
    #[test]
    fn test_empty() {
        assert_eq!(Vec::<i32>::new(), Solution::inorder_traversal(None));
    }

    #[test]
    fn test_two_layer() {
        let tree = Rc::new(RefCell::new(TreeNode::new(1)));
        tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(vec![1, 2], Solution::inorder_traversal(Some(tree)));
    }
}