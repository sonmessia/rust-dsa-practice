struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    ans.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
        return ans;
    }
}


fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(nums);
    println!("{:?}", result);
}