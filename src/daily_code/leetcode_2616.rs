struct Solution;

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
      
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];

        while left < right {
            let mid = (left + right) / 2;
            if Self::can_form_pairs(&nums, p, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left   
    }
    fn can_form_pairs(nums: &Vec<i32>, p: i32, max_diff: i32) -> bool {
        let mut pairs = 0;
        let mut i = 0;
        while i < nums.len() - 1 {
            if nums[i + 1] - nums[i] <= max_diff {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        pairs >= p
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimize_max() {
        let nums = vec![10, 1, 2, 7, 1, 3];
        let p = 2;
        assert_eq!(Solution::minimize_max(nums, p), 1);

        let nums = vec![4, 2, 1, 3];
        let p = 1;
        assert_eq!(Solution::minimize_max(nums, p), 0);

        let nums = vec![1, 3, 6, 10, 15];
        let p = 2;
        assert_eq!(Solution::minimize_max(nums, p), 3);
    }
}