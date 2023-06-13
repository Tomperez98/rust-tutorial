use crate::base::Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut zeros = 0;
        for j in 0..(nums.len()) {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            } else {
                zeros += 1;
            }
        }
        for _ in 0..zeros {
            nums[i] = 0;
            i += 1;
        }
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![0,1,0,3,12], vec![1,3,12,0,0])]
    #[case(vec![1], vec![1])]
    fn test_move_zeroes(#[case] mut nums: Vec<i32>, #[case] expected: Vec<i32>) {
        crate::base::Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected)
    }
}
