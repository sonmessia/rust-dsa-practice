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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_visit_all_rooms() {
        assert_eq!(Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]), true);
        assert_eq!(Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![]]), false);
        assert_eq!(Solution::can_visit_all_rooms(vec![vec![]]), true);
    }
}

fn main() {

}