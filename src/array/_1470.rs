/// The Inplace Solution based on the constraints 1 <= nums[i] <= 10^3, which constraints the number <= 1000 which cost 10bits.
/// So when use type i32 we can store a number in 0\~9bit, and a number in 10\~19bit.
struct InplaceSolution;

impl InplaceSolution {
        pub fn shuffle(mut nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        
        let i = n - 1;
        let j = 2*n - 1;
        
        for shift in 0..n {
            nums[j - shift] = nums[j - shift] << 10; // move the yi to high 10 bits.
            nums[j - shift] |= nums[i - shift]; // move the xi to low 10 bits.
        }
        
        for idx in 0..n {
            nums[idx*2] = nums[idx+n] & 1023; // get xi by and with 00000000001111111111.
            nums[idx*2 + 1] = nums[idx+n] >> 10; // get yi by right shift 10bits.
        }

        nums
    }
}

struct SimpleSolution;

impl SimpleSolution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(2*n as usize);
        let (mut i, mut j) = (0_usize, n as usize);

        for idx in 0..2*n as usize {
            if idx % 2 == 0 { 
                result.push(nums[i]);
                i += 1;
            } else {
                result.push(nums[j]);
                j += 1;
            }
        }

        result
    }
}

struct FunctionalSolution;

impl FunctionalSolution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = nums.iter().enumerate().map(|p| (p.0 % n as usize, p.1)).collect::<Vec<_>>();
        ans.sort_by_key(|p| p.0);
        println!("{:?}", ans);
        ans.iter().map(|p| *p.1).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionalSolution;

    #[test]
    fn test() {
        FunctionalSolution::shuffle(vec![2, 5, 1, 3, 4, 7], 3);
    }
}