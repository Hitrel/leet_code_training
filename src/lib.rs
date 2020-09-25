#![feature(allow_internal_unstable)]
mod array;
mod linked_list;
mod daily_challenge;
mod tree;
mod graph;

#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

