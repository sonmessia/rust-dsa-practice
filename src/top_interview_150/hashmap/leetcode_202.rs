struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut sum = 0;

        loop {
            while n != 0 {
                let tmp = n % 10;
                sum += tmp*tmp;
                n /= 10;
            }
            n = sum;
            if sum == 1 {
                return true;
            }
            if (sum == 116 || sum == 145 || sum == 5 || sum == 80 || sum == 20 || sum == 2) {
                return false;
            }
            sum = 0;
        }
    }
}

fn main() {
    let n = 19;
    println!("Is {} a happy number? {}", n, Solution::is_happy(n));
}