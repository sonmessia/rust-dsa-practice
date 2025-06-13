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
        for (left, right) in [(0, 3), (1, 2), (2, 1), (3, 0)] {
            let diff = nums[n - 1 - right] - nums[left];
            min_diff = min_diff.min(diff);
        }
        min_diff
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_difference() {
        let nums = vec![1, 5, 0, 10, 14];
        assert_eq!(Solution::min_difference(nums), 1);

        let nums = vec![6, 6, 0, 1, 1];
        assert_eq!(Solution::min_difference(nums), 0);

        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::min_difference(nums), 0);

        let nums = vec![5, 3, 2, 4];
        assert_eq!(Solution::min_difference(nums), 0);
    }
}