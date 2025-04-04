struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return true;
        }

        let (mut l, mut r) = (0, chars.len() - 1);
        while l < r {
            while l < r && !chars[l].is_alphanumeric() {
                l += 1;
            }
            while l < r && !chars[r].is_alphanumeric() {
                r -= 1;
            }
            if l < r {
                if chars[l].to_ascii_lowercase() != chars[r].to_ascii_lowercase() {
                    return false;
                }
                l += 1;
                r -= 1;
            }
        }
        true   
    }
}

fn main() {

    let test_string = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome(test_string);
    println!("Is palindrome: {}", result);
}