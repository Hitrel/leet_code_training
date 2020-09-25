struct Solution;
/// 使用动态规划的思想， result数组存储长度为index的BST的数量
/// 当长度为0时，bst的数量为1， 当长度为n时，对1..n的每一个数做根其左子树数量*右子树数量的和为该长度的bst的数量。
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut result: Vec<i32> = vec![0; n];
        if n == 0 { return result[0]; }
        result[0] = 1;
        for len in 0..=n {
            for root in 1..=len {
                let (left, right) = (root - 1, len - root);
                result[len] += result[left] * result[right]; 
            }
        }

        return result[n];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(5, super::Solution::num_trees(3));
    }
}