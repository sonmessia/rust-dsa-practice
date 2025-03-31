use std::collections::HashMap;
use rand::thread_rng;

struct RandomizedSet {
    nums: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            nums: Vec::new(),
            indices: HashMap::new()
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            return false;
        }
        self.nums.push(val);
        self.indices.insert(val, self.nums.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.indices.contains_key(&val) {
            return false;
        }
        let index = self.indices[&val];
        let last_val = self.nums.pop().unwrap();
        self.nums[index] = last_val;
        self.indices.insert(last_val, index);
        self.indices.remove(&val);
        true
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.nums.len());
        self.nums[random_index]
    }
}

fn main() {
    let mut randomized_set = RandomizedSet::new();
    println!("{}", randomized_set.insert(1));
    println!("{}", randomized_set.insert(2));
    println!("{}", randomized_set.insert(3));
    println!("{}", randomized_set.get_random());
    println!("{}", randomized_set.remove(2));
    println!("{}", randomized_set.get_random());
}

