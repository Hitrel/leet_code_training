struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_redundant_directed_connection(mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent = vec![0; edges.len() + 1];

        let (mut temp_a, mut temp_b) = (Vec::new(), Vec::new());

        for edge in edges.iter_mut() {
            if parent[edge[1] as usize] == 0 { 
                parent[edge[1] as usize] = edge[0];
            } else {
                temp_a = vec![parent[edge[1] as usize], edge[1]];
                temp_b = edge.clone();

                edge[1] = 0;
            }
        }

        let mut parent = parent.iter().enumerate().map(|i| i.0 as i32).collect();

        for edge in edges {
            if edge[1] == 0 { continue; }
            let (u, v) = (edge[0], edge[1]);
            let pu = Solution::root(&mut parent, u);

            if pu == v { 
                if temp_a.is_empty() { return edge.clone(); }
                else { return temp_a; }
            }

            parent[v as usize] = pu;
        }

        temp_b
    }

    fn root(parent: &mut Vec<i32>, u: i32) -> i32 {
        match parent[u as usize] {
            v if v == u => v,
            v => {
                parent[u as usize] = Solution::root(parent, v);

                return parent[u as usize];
            }
        }
    }
}