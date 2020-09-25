
struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut g:Vec<Vec<(i32, i32)>> = vec![Vec::new(); n as usize + 1];
        let mut visited: Vec<bool> = vec![false; n as usize + 1];

        for time in times {
            g[time[0] as usize].push((time[1], time[2]));
        }
        Solution::dfs(&g, &mut visited, k)
    }

    fn dfs(g: &Vec<Vec<(i32, i32)>>, visited: &mut Vec<bool>, k: i32) -> i32 {

        if g[k as usize].len() == 0 { return 0; }

        g[k as usize].iter().map(|v| {
            if visited[v.0 as usize] == false {
                visited[v.0 as usize] = true;
                match Solution::dfs(g, visited, v.0) {
                    -1 => -1,
                    i => v.1 + i,
                }
            } else {
                -1
            }
        }).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test() {
        print!("{:?} ", Solution::network_delay_time(vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]], 4, 2))
    }
}
