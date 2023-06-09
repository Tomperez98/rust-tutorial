use crate::base::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut num_good_pais: i32 = 0;
        for (idx, i) in nums.iter().enumerate() {
            for j in nums[idx + 1..].iter() {
                if i == j {
                    num_good_pais += 1
                }
            }
        }
        num_good_pais
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![1,2,3,1,1,3], 4)]
    #[case(vec![1,1, 1, 1], 6)]
    #[case(vec![1,2,3], 0)]
    fn test_num_identical_pairs(#[case] nums: Vec<i32>, #[case] expected: i32) {
        assert_eq!(crate::base::Solution::num_identical_pairs(nums), expected)
    }
}
