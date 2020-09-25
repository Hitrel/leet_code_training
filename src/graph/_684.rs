struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_redundant_connection(mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = vec![edges[0][0]];
        while edges.len() > 1 {
            let mut has_retain_flag = false;
            edges.retain(|v| {
                if visited.contains(&v[0]) {
                    if visited.contains(&v[1]) {
                        true
                    } else {
                        if !has_retain_flag {
                            has_retain_flag = true;
                            visited.push(v[1]);
                            false
                        } else {
                            true
                        }
                    }
                } else {
                    if visited.contains(&v[1]) && !has_retain_flag {
                        has_retain_flag = true;
                        visited.push(v[0]);
                        false
                    } else {
                        true
                    }
                }
            });
        }
        edges.pop().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        Solution::find_redundant_connection(vec![vec![1,2],vec![1,3],vec![2,3]]);
    }
}