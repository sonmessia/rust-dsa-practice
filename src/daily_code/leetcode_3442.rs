struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut map = vec![0; 26];
        for c in s.chars() {
            map[(c as u8 - b'a') as usize] += 1;
        }

        let mut maxOdd = 1;
        let mut minEven = s.len() as i32;

        for &count in map.iter() {
            if count % 2 == 1 {
                maxOdd = maxOdd.max(count);
            } else if count > 0 {
                minEven = minEven.min(count as i32);
            }
        }

        maxOdd - minEven

    }
}

fn main() {
    let s = "aaaaabbc".to_string();
    let result = Solution::max_diffirence(s);
    println!("Maximum difference: {}", result);
}