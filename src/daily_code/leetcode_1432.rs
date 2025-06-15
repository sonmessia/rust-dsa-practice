struct Solution;

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let num_str = num.to_string();
        let mut min_num = num;
        let mut max_num = num;

        for x in b'0'..=b'9' {
            for y in b'0'..=b'9' {
                // Map chars, not bytes!
                let res: String = num_str
                    .chars()
                    .map(|d| if d as u8 == x { y as char } else { d })
                    .collect();

                // Skip if leading zero or is zero
                if res.chars().next().unwrap() != '0' {
                    if let Ok(res_num) = res.parse::<i32>() {
                        if res_num != 0 {
                            min_num = min_num.min(res_num);
                            max_num = max_num.max(res_num);
                        }
                    }
                }
            }
        }
        max_num - min_num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_diff() {
        assert_eq!(Solution::max_diff(555), 888);
        assert_eq!(Solution::max_diff(9), 8);
        assert_eq!(Solution::max_diff(123456), 820000);
        assert_eq!(Solution::max_diff(1001), 999);
        assert_eq!(Solution::max_diff(0), 0);
    }
}