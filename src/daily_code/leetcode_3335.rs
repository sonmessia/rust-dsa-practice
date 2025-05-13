struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut ans = 0;
        const MOD: i32 = 1_000_000_007;

        let mut dp = vec![0; 26];
        for c in s.chars() {
            dp[(c as u8 - b'a') as usize] += 1;
        }
        let mut t = t;
        while t > 0 {
            let mut next = vec![0; 26];
            
            for i in 0..26 {
                if i == 25 {
                    next[0] = dp[i] % MOD;
                    next[1] = (next[1] + dp[i]) % MOD;
                } else {
                    next[i + 1] = dp[i] % MOD;
                }
            }
            dp = next;
            t -= 1;
        }
        for i in 0..26 {
            ans = (ans + dp[i]) % MOD;
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_after_transformations() {
        assert_eq!(Solution::length_after_transformations("abcyy".to_string(), 2), 7);
        assert_eq!(Solution::length_after_transformations("azbk".to_string(), 1), 5);
    }
}