use crate::base::Solution;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut unequal = 0;
        for i in 0..nums.len() - 2 {
            for j in i..nums.len() - 1 {
                for k in j..nums.len() {
                    if (nums[i] != nums[j]) && (nums[k] != nums[i]) && (nums[k] != nums[j]) {
                        unequal += 1;
                    }
                }
            }
        }
        unequal
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![4, 4, 2, 4, 3], 3)]
    fn test_unequal_triplets(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(crate::base::Solution::unequal_triplets(nums), expected)
    }
}
