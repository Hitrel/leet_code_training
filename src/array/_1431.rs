struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candie = candies.iter().max().unwrap();
        candies.iter().map(|candidate|  candidate + extra_candies >= *max_candie ).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn test() {
        assert_eq!(vec![true,true,true,false,true], Solution::kids_with_candies(vec![2,3,5,1,3], 3))
    }
}