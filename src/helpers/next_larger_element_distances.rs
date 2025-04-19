// src/helpers/next_larger_element_distances.rs
fn next_larger_element_distances(nums: &[i32]) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
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