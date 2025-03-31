struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev = 1;
        let mut curr = 2;

        while curr < nums.len() {
            if nums[curr] == nums[prev] && nums[curr] == nums[prev - 1] {
                curr += 1;
            } else {
                prev += 1;
                nums[prev] = nums[curr];
                curr += 1;
            }
        }
        prev as i32 + 1
    }
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    Solution::remove_duplicates(&mut nums);
    println!("{:?}", nums);
}