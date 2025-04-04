use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_map = HashMap::with_capacity(pattern.len());
        let mut word_set = HashSet::with_capacity(pattern.len());
        
        let words = s.split_whitespace().collect::<Vec<_>>();
        let pattern_byte = pattern.as_bytes();

        if pattern_byte.len() != words.len() {
            return false;
        }

        for i in 0..pattern_byte.len() {
            if let Some(mapped_word) = pattern_map.get(&pattern_byte[i]) {
                if *mapped_word != words[i] {
                    return false;
                }
            } else {
                if word_set.contains(words[i]) {
                    return false;
                }
                pattern_map.insert(pattern_byte[i], words[i]);
                word_set.insert(words[i]);
            }
        }
        true
    }
}

fn main() {
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    let result = Solution::word_pattern(pattern, s);
    println!("Does the pattern match? {}", result);
}