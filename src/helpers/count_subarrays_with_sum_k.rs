// src/helpers/count_subarrays_with_sum.rs
use std::collections::HashMap;

pub fn count_subarrays_with_sum_k_positives_only(
    logs: &[i32],
    k: i32,
) -> i32 {
    let mut left: usize = 0;
    let mut sum: i32 = 0;
    let mut count: i32 = 0;

    for right in 0..logs.len() {
        sum += logs[right];

        while left <= right && sum > k {
            sum -= logs[left];
            left += 1;
        }

        if sum == k {
            count += 1;
        }
    }

    count
}

pub fn count_subarrays_with_sum_k_any_values(
    logs: &[i32],
    k: i32,
) -> i32 {
    let mut prefix_sum: i32 = 0;
    let mut count: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    map.insert(0, 1); // 👈 Needed to handle subarrays starting from index 0

    for &num in logs {
        prefix_sum += num;

        if let Some(&freq) = map.get(&(prefix_sum - k)) {
            count += freq;
        }

        // 👇 Non-shortcut version (for understanding)
        // if map.contains_key(&prefix_sum) {
        //     let current = map.get(&prefix_sum).unwrap();
        //     map.insert(prefix_sum, current + 1);
        // } else {
        //     map.insert(prefix_sum, 1);
        // }

        *map.entry(prefix_sum).or_insert(0) += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays_with_sum_k_positives_only() {
        let logs: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 10;
        assert_eq!(
            count_subarrays_with_sum_k_positives_only(
                &logs,
                k
            ),
            1
        );
    }

    #[test]
    fn test_count_subarrays_with_sum_k_any_values() {
        let logs = vec![6, -2, -3, 4, 5];
        let k = 10;
        assert_eq!(
            count_subarrays_with_sum_k_any_values(
                &logs,
                k
            ),
            1
        );
    }
}
