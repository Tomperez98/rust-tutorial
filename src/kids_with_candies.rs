use crate::base::Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = candies.iter().max().expect("No max value.").to_owned();
        let mut can_have_the_most: Vec<bool> = Vec::with_capacity(candies.len());
        for child in candies.into_iter() {
            if child + extra_candies >= max_candies {
                can_have_the_most.push(true)
            } else {
                can_have_the_most.push(false)
            }
        }
        can_have_the_most
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![2,3,5,1,3], 3, vec![true,true,true,false,true] )]
    #[case(vec![12, 1, 12], 10, vec![true,false,true] )]
    #[case(vec![4,2,1,1,2], 1, vec![true,false,false,false,false]  )]
    fn test_kids_with_candies(
        #[case] candies: Vec<i32>,
        #[case] extra_candies: i32,
        #[case] expected: Vec<bool>,
    ) {
        assert_eq!(
            crate::base::Solution::kids_with_candies(candies, extra_candies),
            expected
        )
    }
}
