struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;

        let mut G = vec![Vec::<i32>::new(); num_courses];
        let mut degree = vec![0; num_courses];
        let mut bfs = Vec::new();

        for edge in prerequisites.iter() {
            G[edge[1] as usize].push(edge[0]);
            degree[edge[0] as usize] += 1;
        }

        for i in 0_usize..degree.len() {
            if degree[i] == 0 { bfs.push(i); }
        }

        let mut idx = 0_usize;
        while idx < bfs.len() {
            for &node in G[bfs[idx]].iter() {
                degree[node as usize] -= 1;
                if degree[node as usize] == 0 { bfs.push(node as usize); } 
            }

            idx += 1;
        }

        return bfs.len() == num_courses;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_two_layer() {
        assert!(Solution::can_finish(2, vec![vec![0, 1]]));
    }

    #[test]
    fn test_three_layer() {
        assert!(Solution::can_finish(3, vec![vec![1, 0], vec![2, 1]]));
    }

    #[test]
    fn test_loop() {
        assert!(!Solution::can_finish(3, vec![vec![1, 0], vec![0, 1]]));
    }
}