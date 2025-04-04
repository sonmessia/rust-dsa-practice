use std::collections::HashMap;
struct Solution;    

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let j = target - num;
            if let Some(&m) = map.get(&j) {
                return vec![m as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}