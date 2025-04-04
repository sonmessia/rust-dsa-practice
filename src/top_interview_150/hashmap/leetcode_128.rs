use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut l = 0;
                let mut curr = num;
                while set.contains(&(curr)) {
                    l += 1;
                    curr += 1;
                }
                longest = longest.max(l);
            }
        }
        longest
    }
}

fn main() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let result = Solution::longest_consecutive(nums);
    println!("The length of the longest consecutive sequence is: {}", result);
}