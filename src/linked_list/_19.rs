//! recursion travel the list
//! recursion function: remove_helper(head, n) ->　
//! 
use super::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        match Solution::remove_helper(head, n) {
            (Some(node), i) if i == n => node.next,
            (node, _) => node,
        }
    }
    
    fn remove_helper(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        match head {
            Some(mut i) =>  match Solution::remove_helper(i.next.take(), n) {
                (None, 0) => {
                    if n == 1 {
                        return (None, 1);
                    } else {
                        return (Some(i), 1);
                    }
                }, 
                (Some(next_node), number) if number == n => {
                    i.next = next_node.next;
                    return (Some(i), number + 1);
                },
                (next_node_wrapped, number) => {
                    i.next = next_node_wrapped;
                    return (Some(i), number + 1);
                },
                
            },
            None => return (None, 0),
        }
    }
}

