use super::{TreeNode, prelude::* };

struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let n = n as usize;
        let result: Vec<RefCell<Vec<Option<Rc<RefCell<TreeNode>>>>>> = vec![RefCell::new(Vec::new()); n + 1]; 

        if n == 0 {
            return result[0].borrow().clone();
        }

        result[0].borrow_mut().push(None);
        for len in 0_usize..=n {
            for root in 1..=len {
                let (left, right) = (root - 1, len - root);
                for left_node in result[left].borrow().iter() {
                    for right_node in result[right].borrow().iter() {
                        let new_node = Rc::new(RefCell::new(TreeNode::new(root as i32)));
                        new_node.borrow_mut().left = left_node.clone();
                        new_node.borrow_mut().right = Solution::clone(right_node.as_ref(), root as i32);
                        result[len].borrow_mut().push(Some(new_node));
                    }
                }
            }
        }

        return result[n].clone().borrow().clone();
    }

    fn clone(node: Option<&Rc<RefCell<TreeNode>>>, offset: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(node) => {
                let new_node = Rc::new(RefCell::new(TreeNode::new(node.borrow().val + offset)));
                new_node.borrow_mut().left = Solution::clone(node.borrow().left.as_ref(), offset);
                new_node.borrow_mut().right = Solution::clone(node.borrow().right.as_ref(), offset);

                Some(new_node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::Solution;

    #[test]
    fn test() {
        Solution::generate_trees(3);
    }
}