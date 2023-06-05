fn main() {
    println!("Hello, world!");
}

struct Solution;

use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) -= 1;
        }
        let mut heap = BinaryHeap::with_capacity(k as usize + 1);
        for item in map.into_iter() {
            heap.push((item.1, item.0));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.into_iter().map(|val| val.1).collect()
    }
}