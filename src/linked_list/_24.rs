use super::ListNode;

struct SolutionComplex;

impl SolutionComplex {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut temp = &mut head;
        let _flag = false;
        while temp.is_some() {
            let (val, flag) = match temp.as_mut().unwrap() {
                v => match v.next.as_mut() {
                    Some(mut next) => {
                        let temp_number = next.val;
                        next.val = v.val;

                        (temp_number, true)
                    },
                    None => (v.val, false),
                },
            };

            if flag {
                temp.as_mut().unwrap().val = val;
                temp = &mut temp.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                break;
            }
            
        }


        head
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut cur| {
            match cur.next {
                Some(mut next) => {
                    cur.next = Solution::swap_pairs(next.next);
                    next.next = Some(cur);
                    Some(next)
                },
                None => Some(cur),
            }
        })
    }
}