use crate::base::Solution;
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut triplets: i32 = 0;
        for i in 0..(nums.len() - 2) {
            for j in i..(nums.len() - 1) {
                for k in j..(nums.len()) {
                    if (nums[j] - nums[i] == diff) && (nums[k] - nums[j] == diff) {
                        triplets += 1
                    }
                }
            }
        }

        triplets
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![0, 1, 4, 6, 7, 10], 3, 2)]
    fn test_arithmetic_triplets(#[case] nums: Vec<i32>, #[case] diff: i32, #[case] expected: i32) {
        assert_eq!(
            crate::base::Solution::arithmetic_triplets(nums, diff),
            expected
        )
    }
}
