pub mod _19;
pub mod _21;
pub mod _23;
pub mod _24;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self { Self { val, next } }
}

