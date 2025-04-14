// src/helpers/contains_duplicates.rs
pub fn contains_duplicates(nums: &[i32]) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicates_true() {
        let nums = vec![1, 2, 3, 4, 5, 1];
        assert_eq!(contains_duplicates(&nums), true);
    }
}