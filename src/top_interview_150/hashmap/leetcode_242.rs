use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            if map.contains_key(&c) {
                if map.get(&c) > Some(&0) {
                    *map.get_mut(&c).unwrap() -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        let sum = map.values().fold(0, |acc, value| acc + *value);

        sum <= 0
    }
}