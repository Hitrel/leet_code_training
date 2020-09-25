use super::ListNode;
use std::collections::BinaryHeap;
struct SolutionBad;

impl SolutionBad {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        match SolutionBad::get_min(&mut lists) {
            None => None,
            Some(mut item) => {
                item.next = SolutionBad::merge_k_lists(lists);
                Some(item)
            }
        }

    }
    
    fn get_min(lists: &mut Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if let mut min @ (_, Some(_))  = (0, lists[0].clone()) {
            for item in lists.iter().enumerate() {
                min =match (item, min) {
                    ((_, None), min_value) => min_value,
                    ((i, Some(this)), (_, Some(min_value))) if this.val < min_value.val => {
                        (i, Some(this.clone()))
                    },
                    (_, min_value) => min_value,
                }
            }

            return if let (i, Some(mut this)) = min {
                if this.next.is_some() {
                    lists[i] = this.next.take();
                } else {
                    lists.remove(i);
                }

                Some(this)
            } else {
                None
            }
        } else {
            return None;
        }
    }
}

struct SolutionNormal;

impl SolutionNormal {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        match lists.len() {
            1 => lists[0].take(),
            0 => None,
            len => SolutionNormal::merge_helper(&mut lists, 0, len - 1)
        }
    }

    fn merge_helper(lists: &mut Vec<Option<Box<ListNode>>>, start: usize, end: usize) -> Option<Box<ListNode>> {
        if start == end { return lists[start].take(); }
        if start == end - 1 { return super::_21::Solution::merge_two_lists(lists[start].take(), lists[end].take()); }
        
        super::_21::Solution::merge_two_lists(
            SolutionNormal::merge_helper(lists, start, start + (end - start) / 2), 
            SolutionNormal::merge_helper(lists, start + (end - start) / 2 + 1, end))
    }
}

struct Solution;

impl Solution {
    
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Solution::list_from_heap(
        BinaryHeap::from(lists.iter().fold(Vec::<i32>::new(), |mut acc, mut cur| {
            while cur.is_some() {
                acc.push(cur.as_ref().unwrap().val);
                cur = &cur.as_ref().unwrap().next;
            }

            acc
        })))
    }

    fn list_from_heap(mut heap: BinaryHeap<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        while !heap.is_empty() {
            head = Some(Box::new(ListNode {
                val: heap.pop().unwrap(),
                next: head,
            }));
        }

        head
    }
}