struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let n = nums.len();
        if n == 0 {
            return ans;
        }

        let (mut start, mut end) = (nums[0], nums[0]);

        for i in 1..n {
            if nums[i] == nums[i-1]+1 {
                end = nums[i];
            } else {
                if (start != end) {
                    ans.push(format!("{}->{}", start, end));
                } else {
                    ans.push(format!("{}", start));
                }
                (start, end) = (nums[i], nums[i]);
            }
        }
        if (start != end) {
            ans.push(format!("{}->{}", start, end));
        } else {
            ans.push(format!("{}", start));
        }
        ans
    }
}


fn main() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    let result = Solution::summary_ranges(nums);
    println!("{:?}", result);
}