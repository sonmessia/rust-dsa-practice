use std::collections::BinaryHeap;
struct Solution;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = nums.len();
        let mut heap = BinaryHeap::new();
        let mut diff_array = vec![0; n + 1];
        let mut op = 0;
        let mut j = 0;

        for i in 0..n {
            op += diff_array[i];
            while j < queries.len() && queries[j][0] == i as i32 {
                heap.push(queries[j][1]);
                j += 1;
            }

            while op < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
                let x = heap.pop().unwrap();
                op += 1;
                diff_array[x as usize + 1] -= 1;
            }

            if op < nums[i] {
                return -1;
            }
        }

        heap.len() as i32
    } 
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let queries = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    let result = Solution::max_removal(nums, queries);
    println!("Result: {}", result);
}