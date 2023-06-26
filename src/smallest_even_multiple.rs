use crate::base::Solution;
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            n * 2
        }
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(6, 6)]
    fn test_smallest_even_multiple(#[case] n: i32, #[case] expected: i32) {
        assert_eq!(crate::base::Solution::smallest_even_multiple(n), expected)
    }
}
