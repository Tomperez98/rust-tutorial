use std::collections::{HashMap, HashSet};

use crate::base::Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let set_jewels: HashSet<char> = jewels.chars().collect();
        let mut count_of_stones: HashMap<char, i32> = HashMap::new();
        for stone in stones.chars().into_iter() {
            match count_of_stones.get(&stone) {
                None => {
                    count_of_stones.insert(stone, 1);
                }
                Some(curr_count) => {
                    count_of_stones.insert(stone, curr_count + 1);
                }
            }
        }

        let mut number_of_jewels = 0;
        for jewel in set_jewels.into_iter() {
            match count_of_stones.get(&jewel) {
                None => continue,
                Some(num) => number_of_jewels += num,
            }
        }
        return number_of_jewels;
    }
}

mod test {
    use rstest::*;

    #[rstest]
    #[case("aA", "aAAbbbb", 3)]
    #[case("z", "ZZ", 0)]
    fn test_num_jewels_in_stones(
        #[case] jewels: String,
        #[case] stones: String,
        #[case] expected: i32,
    ) {
        assert_eq!(
            crate::base::Solution::num_jewels_in_stones(jewels, stones),
            expected
        )
    }
}
