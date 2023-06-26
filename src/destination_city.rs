use std::collections::HashSet;

use crate::base::Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut depts = HashSet::with_capacity(paths.len());
        let mut dests = HashSet::with_capacity(paths.len());

        for path in paths {
            depts.insert(path.first().unwrap().to_owned());
            dests.insert(path.last().unwrap().to_owned());
        }
        return dests.difference(&depts).last().unwrap().to_owned();
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case(vec![
        vec!["B".to_owned(), "C".to_owned()],
        vec!["D".to_owned(), "B".to_owned()],
        vec!["C".to_owned(), "A".to_owned()],
    ], "A")]
    #[case(vec![
        vec!["A".to_owned(), "Z".to_owned()],
    ], "Z")]
    fn test_dest_city(#[case] cities: Vec<Vec<String>>, #[case] expected: String) {
        assert_eq!(crate::base::Solution::dest_city(cities), expected)
    }
}
