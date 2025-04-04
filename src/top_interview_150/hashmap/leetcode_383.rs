use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        for c in magazine.chars() {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        
        for c in ransom_note.chars() {
            match map.get_mut(&c) {
                Some(n) if *n > 0 => { *n -= 1; }
                _ => { return false; }
            }
        }
        true
    }
}

fn main() {
    let ransom_note = "a".to_string();
    let magazine = "b".to_string();
    let result = Solution::can_construct(ransom_note, magazine);
    println!("Can construct: {}", result);
}