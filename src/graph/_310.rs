struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;

        let mut G = vec![HashSet::<i32>::new(); n];
        let mut degree = vec![0; n];

        for edge in edges.iter() {
            G[edge[0] as usize].insert(edge[1]);
            G[edge[1] as usize].insert(edge[0]);
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }

        let mut leaves = Box::new(Vec::<i32>::new());

        for i in degree.iter().enumerate() {
            if *i.1 == 1 {
                leaves.push(i.0 as i32);
            }
        }

        let mut size = n;

        while size > 2 {
            size -= leaves.len();
            let mut newLeaves = Vec::<i32>::new();
            for &node in leaves.iter() {
                let &next_node = G[node as usize].iter().next().unwrap();
                G[next_node as usize].remove(&node);
                if G[next_node as usize].len() == 1 { newLeaves.push(next_node)}
            }

            leaves = Box::new(newLeaves);
        }
        if leaves.len() != 0 {
            *leaves
        } else {
            vec![0]
        }
    }
}

#[cfg(test)]
mod tests {
    fn test() {
        
    }
}