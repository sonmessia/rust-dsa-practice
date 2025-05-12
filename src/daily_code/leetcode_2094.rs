use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut nums = HashSet::new();
        let n = digits.len();

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if i == j || j == k || i == k {
                        continue;
                    }
                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                    if num >= 100 && num % 2 == 0 {
                        nums.insert(num);
                    }
                }
            }
        }

        let mut res: Vec<i32> = nums.into_iter().collect();
        res.sort();
        res
    }
}