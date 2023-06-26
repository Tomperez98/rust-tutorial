use crate::base::Solution;

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..(nums.len() - 1) {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let mut zeros: i32 = 0;
        let mut output: Vec<i32> = Vec::with_capacity(nums.len());
        for num in nums {
            if num == 0 {
                zeros += 1;
            } else {
                output.push(num);
            }
        }
        for _ in 0..zeros {
            output.push(0)
        }
        output
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![0,1], vec![1,0])]
    #[case(vec![1,2,2,1,1,0], vec![1,4,2,0,0,0])]
    fn test_apply_operations(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(crate::base::Solution::apply_operations(nums), expected)
    }
}
