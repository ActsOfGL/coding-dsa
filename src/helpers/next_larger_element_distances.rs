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
    fn example_case() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn strictly_decreasing() {
        let temps = vec![5, 4, 3, 2, 1];
        // No future day is ever warmer
        let expected = vec![0, 0, 0, 0, 0];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn all_equal() {
        let temps = vec![7, 7, 7, 7];
        // All days have the same temperature, so no warmer day
        let expected = vec![0, 0, 0, 0];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn two_elements_warm_then_cool() {
        let temps = vec![5, 6];
        let expected = vec![1, 0]; // day 0 waits 1 day, day 1 has no warmer day
        assert_eq!(next_larger_element_distances(&temps), expected);

        let temps = vec![6, 5];
        let expected = vec![0, 0]; // neither day finds a warmer future day
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn single_element() {
        let temps = vec![42];
        let expected = vec![0]; // no future days at all
        assert_eq!(next_larger_element_distances(&temps), expected);
    }

    #[test]
    fn empty_input() {
        let temps: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(next_larger_element_distances(&temps), expected);
    }
}
