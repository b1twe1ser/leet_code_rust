use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
    // Create hashmap
    let mut complements: HashMap<i32, i32> = HashMap::new();
    // iterate over hwowashmap
    for (i, v) in nums.iter().enumerate() {
        // check if the element already exists in the map
        match complements.get(v) {
            // if it does, return the match
            Some(&index) => return Some(vec![index, i as i32]),
            // else put the difference in the map
            None => complements.insert(target - v, i as i32),
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        assert_eq!(two_sum(vec![1, 2, 3, 4], 3), Some(vec![0, 1]));
        assert_eq!(two_sum(vec![1, 2, 3, 4], 6), Some(vec![1, 3]));
        assert_eq!(two_sum(vec![3, 3], 6), Some(vec![0, 1]));
        assert_eq!(two_sum(vec![1, 10, 4], 5), Some(vec![0, 2]));
        assert_eq!(two_sum(vec![1, 5, 6, 8, 9], 11), Some(vec![1, 2]));
    }
}
