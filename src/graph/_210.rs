struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut G = vec![Vec::<i32>::new(); num_courses];
        let mut degree = vec![0; num_courses];
        let mut dfs = Vec::<i32>::new();

        for edge in prerequisites.iter() {
            G[edge[1] as usize].push(edge[0]);
            degree[edge[0] as usize] += 1;
        }

        for i in 0..degree.len() {
            if degree[i] == 0 { dfs.push(i as i32)}
        }        

        let mut idx = 0_usize;
        while idx < dfs.len() {
            for &node in G[dfs[idx] as usize].iter() {
                degree[node as usize] -= 1;
                if degree[node as usize] == 0 {
                    dfs.push(node);
                }
            }

            idx += 1;
        }
        
        if dfs.len() == num_courses {
            dfs
        } else {
            vec![]
        }
    }
}