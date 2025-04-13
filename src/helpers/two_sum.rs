// src/helpers/two_sum.rs
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&item) = map.get(&complement) {
            return Some((item, i));
        }
        map.insert(num, i);
    }

    None
}
