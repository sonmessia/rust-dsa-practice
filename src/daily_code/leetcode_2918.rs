struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let cnt1 = nums1.iter().fold(0, |acc, x| if *x == 0 { acc + 1 } else { acc });
        let cnt2 = nums2.iter().fold(0, |acc, x| if *x == 0 { acc + 1 } else { acc });
        let mut sum1 = nums1.iter().fold(0, |acc, x| acc + x);
        let mut sum2 = nums2.iter().fold(0, |acc, x| acc + x);

        if (cnt1 == 0 && sum2 > sum1) || (cnt2 == 0 && sum1 > sum2) {
            return -1;
        }
    
        sum1 += cnt1;
        sum2 += cnt2;
        sum1.max(sum2) as i64
    }
}


fn main() {
    println!("Hello, world!");
    let nums1 = vec![3, 2, 0, 1, 0];
    let nums2 = vec![6, 5, 0];
    let result = Solution::min_sum(nums1, nums2);
    println!("Minimum sum: {}", result);
}
