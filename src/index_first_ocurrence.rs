use crate::base::Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case("sadbutsad".to_owned(), "sad".to_owned(), 0)]
    fn test_str_str(#[case] haystack: String, #[case] needle: String, #[case] expected: i32) {
        assert_eq!(crate::base::Solution::str_str(haystack, needle), expected)
    }
}
