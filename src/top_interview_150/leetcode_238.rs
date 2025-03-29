struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer: Vec<i32> = vec![0; n];
        let mut product = 1;
        let mut zeros = 0;

        for num in &nums {
            if *num == 0 {
                zeros += 1;
                continue;
            }
            product *= num;
        }

        if zeros == 1 {
            for i in 0..n {
                answer[i] = if nums[i] == 0 {product} else {0}; 
            }
        }
        else if zeros == 0 {
            for i in 0..n {
                answer[i] = product / nums[i];
            }
        }
        answer
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let answer = Solution::product_except_self(nums);
    println!("{:?}", answer);
}


