struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut track = 0;
        for num in nums.iter() {
            if count == 0 {
                track = *num;
                count += 1;
            }
            else if track == *num {
                count += 1;
            }
            else {
                count -= 1;
            }
        }
        if count > (nums.len() / 2) as i32 {
            track as i32;
        }
        -1
    }
}

fn main() {
    let nums = vec![3, 2, 3];
    let result = Solution::majority_element(nums);
    println!("{}", result);
}