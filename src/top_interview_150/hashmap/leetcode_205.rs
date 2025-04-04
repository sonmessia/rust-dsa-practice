struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map1 = vec![0; 128];
        let mut map2 = vec![0; 128];

        for (s_ch, t_ch) in s.chars().zip(t.chars()) {
            if map1[s_ch as usize] == 0 && map2[t_ch as usize] == 0 {
                map1[s_ch as usize] = t_ch as i32;
                map2[t_ch as usize] = s_ch as i32;
            } else if map1[s_ch as usize] != t_ch as i32 ||
                        map2[t_ch as usize] != s_ch as i32 {
                            return false;
                        }
        }
        true
    }
}

fn main() {
    let s = "egg".to_string();
    let t = "add".to_string();
    println!("{}", Solution::is_isomorphic(s, t)); // true

    let s = "foo".to_string();
    let t = "bar".to_string();
    println!("{}", Solution::is_isomorphic(s, t)); // false

    let s = "paper".to_string();
    let t = "title".to_string();
    println!("{}", Solution::is_isomorphic(s, t)); // true
}