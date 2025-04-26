// src/helpers/get_max_stable_session_count.rs
pub fn get_max_stable_session_count(
    logs: &[i32],
    threshold: i32,
) -> usize {
    let mut left = 0;
    let mut sum = 0;
    let mut highest = 0;
    for right in 0..logs.len() {
        sum += logs[right];
        while left <= right && sum > threshold {
            sum -= logs[left];
            left += 1;
        }
        highest = highest.max(right - left + 1);
    }
    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_stable_session_count() {
        let logs = vec![20, 10, 5, 25, 15, 5];
        let threshold = 40;
        assert_eq!(get_max_stable_session_count(&logs, threshold), 3);
    }

    #[test]
    fn test_get_max_stable_session_count_empty() {
        let logs = Vec::new();
        let threshold = 40;
        assert_eq!(
          get_max_stable_session_count(
            &logs,
            threshold,
          ),
          0
        );
    }
}
