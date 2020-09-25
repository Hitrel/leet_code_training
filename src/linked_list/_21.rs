// Definition for singly-linked list.

use super::ListNode;
pub struct Solution;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (l1, None) => l1,
            (None, l2) => l2,
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Solution::merge_two_lists(l1.next.take(), Some(l2));
                    return Some(l1);
                } else {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next.take());
                    return Some(l2);
                }
            }
        }
    }

}
