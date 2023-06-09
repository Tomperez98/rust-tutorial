use std::collections::HashSet;

use crate::base::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen_numbers: HashSet<i32> = HashSet::with_capacity(nums.len());
        for num in nums {
            if seen_numbers.contains(&num) {
                return true;
            }
            seen_numbers.insert(num);
        }
        false
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![1,2,3,1], true)]
    #[case(vec![1,2,3,4], false)]
    #[case(vec![1,1,1,3,3,4,3,2,4,2], true)]
    fn test_contains_duplicate(#[case] nums: Vec<i32>, #[case] expected: bool) {
        assert_eq!(crate::base::Solution::contains_duplicate(nums), expected)
    }
}
