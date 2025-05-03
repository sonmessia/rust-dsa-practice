struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        fn check(tops: &Vec<i32>, bottoms: &Vec<i32>, x: i32) -> i32 {
            let mut top_rotations = 0;
            let mut bottom_rotations = 0;
            for i in 0..tops.len() {
                if tops[i] != x && bottoms[i] != x {
                    return -1;
                } else if tops[i] != x {
                    top_rotations += 1;
                } else if bottoms[i] != x {
                    bottom_rotations += 1;
                }
            }
            top_rotations.min(bottom_rotations)
        }
        let candidates = [tops[0], bottoms[0]];
        let mut min_rotations = i32::MAX;
        for &x in &candidates {
            let rotations = check(&tops, &bottoms, x);
            if rotations != -1 {
                min_rotations = min_rotations.min(rotations);
            }
        }
        if min_rotations == i32::MAX {
            -1
        } else {
            min_rotations
        }
    }
}

fn main() {
    let tops = vec![2, 1, 2, 4, 2, 2];
    let bottoms = vec![5, 2, 6, 2, 3, 2];
    let result = Solution::min_domino_rotations(tops, bottoms);
    println!("Minimum rotations: {}", result); 
}