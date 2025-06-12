struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let n = nums.len();
        for i in 0..n {
            ans = ans.max((nums[i] - nums[(i + 1) % n]).abs());
        }
        ans
    }
}

fn main() {
    let nums = vec![1, 3, 6, 10, 15];
    println!("Maximum difference: {}", Solution::max_adjacent_distance(nums));
}