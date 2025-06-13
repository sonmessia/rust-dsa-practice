struct Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 4 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut min_diff = i32::MAX;
        for i in 0..4 {
            let diff = nums[n - 4 + i] - nums[i];
            min_diff = min_diff.min(diff);
        }
        min_diff
    }
}

fn main() {
    let nums = vec![10, 1, 2, 7, 1, 3];
    println!("Minimum difference: {}", Solution::min_difference(nums));
}