use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(&m) = map.get(&num) {
                if (m as i32 - i as i32).abs() <= k {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}
    
fn main() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    let result = Solution::contains_nearby_duplicate(nums, k);
    println!("{}", result);
}