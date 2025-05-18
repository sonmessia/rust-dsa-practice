use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let mut ans = 0;
        let mut prefix_sum = vec![0; n + 1];
        let mut map = HashMap::new();
        map.insert(0, 1);

        for i in 0..n {
            prefix_sum[i + 1] = (prefix_sum[i] + (nums[i] % modulo == k) as i32) % modulo;
            let target = (prefix_sum[i + 1] - k + modulo) % modulo;
            ans += *map.get(&target).unwrap_or(&0);
            *map.entry(prefix_sum[i + 1]).or_insert(0) += 1;
        }
        ans
    }
}











