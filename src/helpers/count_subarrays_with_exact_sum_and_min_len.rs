// src/helpers/count_subarrays_with_exact_sum_and_min_len.rs
use std::collections::HashMap;

pub fn count_subarrays_with_exact_sum_and_min_len(
    nums: &[i32],
    k: i32,
    min_len: usize,
) -> i32 {
    let mut prefix_sum: i32 = 0;
    let mut count: i32 = 0;
    let mut prefix_count: HashMap<i32, Vec<isize>> = HashMap::new();
    prefix_count.entry(0).or_insert(vec![-1]);
    
    for (end, &num) in nums.iter().enumerate() {
        prefix_sum += num;
        let target = prefix_sum - k;
        if let Some(indices) = prefix_count.get(&target) {
            for &start in indices.iter() {
                if end as isize - start as isize >= min_len as isize {
                    count += 1;
                }
            }
        }
        prefix_count.entry(prefix_sum).or_insert(Vec::new()).push(end as isize);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays_with_exact_sum_and_min_len() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let k: i32 = 7;
        let min_len: usize = 2;
        assert_eq!(
            count_subarrays_with_exact_sum_and_min_len(
                &nums,
                k,
                min_len,
            ),
            1,
        );
    }

    #[test]
    fn test_count_subarrays_with_exact_sum_and_min_len_multiple_cases() {
        let nums: Vec<i32> = vec![1, 2, 3, 3, 4, 5, 4, 3, 3, 2, 2, 1, 1, 1];
        let k: i32 = 2;
        let min_len: usize = 2;
        assert_eq!(
            count_subarrays_with_exact_sum_and_min_len(
                &nums,
                k,
                min_len,
            ),
            2,
        );
    }

    #[test]
    fn test_count_subarrays_with_exact_sum_and_min_len_empty_array() {
        let nums: Vec<i32> = vec![];
        let k: i32 = 2;
        let min_len: usize = 2;
        assert_eq!(count_subarrays_with_exact_sum_and_min_len(&nums,k,min_len), 0);
    }
}
