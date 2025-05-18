use std::collections::VecDeque;
struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut seen = vec![false; rooms.len()];
        seen[0] = true;
        let mut stack = VecDeque::new();
        stack.push_back(0);
        while let Some(node) = stack.pop_back() { 
            for &nei in &rooms[node] {
                if !seen[nei as usize] { 
                    seen[nei as usize] = true; 
                    stack.push_back(nei as usize); 
                }
            }
        }

        seen.into_iter().all(|v| v)
    }
}