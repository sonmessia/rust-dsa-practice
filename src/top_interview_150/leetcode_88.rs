struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut i = m + n;
        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[i - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[i - 1] = nums2[n - 1];
                n -= 1;
            }
            i -= 1;
        }
        while n > 0 {
            nums1[i - 1] = nums2[n - 1];
            n -= 1;
            i -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let nums2 = vec![2, 5, 6];
    let n = 3;
    Solution::merge(&mut nums1, m, nums2, n);
    println!("{:?}", nums1);

}