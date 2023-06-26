use std::collections::BinaryHeap;

use crate::base::Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut heap = BinaryHeap::with_capacity(names.len());
        for i in 0..names.len() {
            heap.push((heights[i], names[i].to_owned()))
        }
        let mut sorted: Vec<String> = Vec::with_capacity(names.len());
        while !heap.is_empty() {
            sorted.push(heap.pop().unwrap().1)
        }
        sorted
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(
        vec!["Mary".to_owned(), "John".to_owned(), "Emma".to_owned()],
        vec![180, 165, 170],
        vec!["Mary".to_owned(), "Emma".to_owned(), "John".to_owned() ])]
    fn test_sort_people(
        #[case] names: Vec<String>,
        #[case] heights: Vec<i32>,
        #[case] expected: Vec<String>,
    ) {
        assert_eq!(crate::base::Solution::sort_people(names, heights), expected)
    }
}
