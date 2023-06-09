use std::collections::{BinaryHeap, HashMap};

use crate::base::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(nums.len());
        let mut counter: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            match counter.get(&num) {
                None => {
                    counter.insert(num, 1);
                }
                Some(curr) => {
                    counter.insert(num, curr + 1);
                }
            }
        }
        for (num, freq) in counter {
            freq_heap.push((freq, num))
        }
        let mut result: Vec<i32> = Vec::with_capacity(k as usize);
        for _ in 0..k {
            result.push(freq_heap.pop().unwrap().1);
        }
        result
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![1,1,1,2,2,3], 2, vec![1,2]  )]
    #[case(vec![1], 1, vec![1]  )]
    fn test_top_k_frequent(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        assert_eq!(crate::base::Solution::top_k_frequent(nums, k), expected)
    }
}
