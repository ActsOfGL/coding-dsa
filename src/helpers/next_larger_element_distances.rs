// src/helpers/next_larger_element_distances.rs
pub fn next_larger_element_distances(nums: &[i32]) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0; nums.len()];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..nums.len() {
        while let Some(&last_index) = stack.last() {
            if nums[i] > nums[last_index] {
                stack.pop();
                res[last_index] = (i - last_index) as i32;
            } else {
                break;
            }
        }
        stack.push(i);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_larger_element_distances_example_case() {
        let temps: Vec<i32> = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected: Vec<i32> = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn test_next_larger_element_distances_strictly_decreasing() {
        let temps: Vec<i32> = vec![5, 4, 3, 2, 1];
        // No future day is ever warmer
        let expected: Vec<i32> = vec![0, 0, 0, 0, 0];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn test_next_larger_element_distances_all_equal() {
        let temps: Vec<i32> = vec![7, 7, 7, 7];
        // All days have the same temperature, so no warmer day
        let expected: Vec<i32> = vec![0, 0, 0, 0];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn test_next_larger_element_distances_two_elements_warm_then_cool() {
        let temps: Vec<i32> = vec![5, 6];
        let expected: Vec<i32> = vec![1, 0]; // day 0 waits 1 day, day 1 has no warmer day
        assert_eq!(next_larger_element_distances(&temps), expected);

        let temps: Vec<i32> = vec![6, 5];
        let expected: Vec<i32> = vec![0, 0]; // neither day finds a warmer future day
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn test_next_larger_element_distances_single_element() {
        let temps: Vec<i32> = vec![42];
        let expected: Vec<i32> = vec![0]; // no future days at all
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn test_next_larger_element_distances_empty_input() {
        let temps: Vec<i32> = Vec::new();
        let expected: Vec<i32> = Vec::new();
        assert_eq!(next_larger_element_distances(&temps), expected);
    }
}
