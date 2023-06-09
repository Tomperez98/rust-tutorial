use std::collections::HashMap;

use crate::base::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut x: HashMap<i32, i32> = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let rem = target - num;
            match x.get(num) {
                None => {
                    x.insert(rem, idx as i32);
                }
                Some(val) => return vec![val.to_owned(), idx as i32],
            }
        }

        unreachable!()
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![2, 7, 11, 16], 9, vec![0, 1])]
    #[case(vec![3,2,4], 6, vec![1, 2])]
    #[case(vec![3,3], 6, vec![0, 1])]
    fn test_two_sum(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        assert_eq!(crate::base::Solution::two_sum(nums, target), expected)
    }
}
