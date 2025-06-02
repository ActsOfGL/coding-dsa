// src/helpers/get_length_of_shortest_window.rs
pub fn get_length_of_shortest_window(
    logs: &[i32],
    k: usize,
    threshold: i32,
) -> i32 {
    let mut left: usize = 0;
    let mut sum: i32 = 0;
    let mut min_len: i32 = i32::MAX;

    for right in 0..logs.len() {
        sum += logs[right];

        while right - left + 1 >= k {
            let window_len: usize = right - left + 1;
            if sum > (threshold * window_len as i32) {
                min_len = min_len.min(window_len as i32);
            }

            // Try shrinking the window from the left
            sum -= logs[left];
            left += 1;
        }
    }

    if min_len == i32::MAX {
        -1 // No valid subarray found
    } else {
        min_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_window() {
        let logs: Vec<i32> = vec![20, 30, 50, 90, 80, 20];
        let k: usize = 3;
        let threshold: i32 = 60;
        assert_eq!(
            get_length_of_shortest_window(
                &logs,
                k,
                threshold,
            ),
            3,
        );
    }

    #[test]
    fn test_shortest_window_empty() {
        let logs: Vec<i32> = Vec::new();
        let k: usize = 3;
        let threshold: i32 = 60;
        assert_eq!(
          get_length_of_shortest_window(&logs, k, threshold),
          -1,
        );
    }
}
