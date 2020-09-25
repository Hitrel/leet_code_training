struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut sum = 0;
        for c in word.chars() {
            if c.is_uppercase() {
                sum += 1;
            }
        }

        match sum {
            0 => true,
            1 if word.chars().next().unwrap().is_uppercase() => true,
            1 => false,
            l if l == word.len() => true,
            _ => false,
        }
        
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_full_uppercase() {
       assert!(Solution::detect_capital_use("USA".to_string()))
    }

     #[test]
    fn test_first_uppercase() {
       assert!(Solution::detect_capital_use("Usa".to_string()))
    }
    
     #[test]
    fn test_non_uppercase() {
       assert!(Solution::detect_capital_use("usa".to_string()))
    }

     #[test]
    fn test_other_uppercase() {
       assert!(!Solution::detect_capital_use("usA".to_string()));
       assert!(!Solution::detect_capital_use("uAs".to_string())); 
    }
}