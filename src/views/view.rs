// src/view/view.rs [renderer]

use std::vec;

use crate::helpers::two_sum;
use crate::helpers::group_anagrams;
use crate::helpers::contains_duplicates;
use crate::helpers::has_nearby_duplicate;
use crate::helpers::has_nearby_divisible_duplicate;
use crate::helpers::has_subarray_with_sum_multiple_of_k;
use crate::helpers::next_larger_element_distances;
use crate::helpers::max_sum_sliding_window;
use crate::helpers::max_sum_sliding_window_of_k_adaptive;
use crate::helpers::max_sum_with_prefix_sum;
use crate::helpers::get_length_of_shortest_window;
use crate::helpers::get_max_stable_session_count;
use crate::helpers::count_subarrays_with_sum_k_positives_only;
use crate::helpers::count_subarrays_with_sum_k_any_values;
use crate::helpers::count_valid_subarray_sum_of_k;
use crate::helpers::count_valid_subarray_sum_of_k_optimized;
use crate::helpers::count_subarrays_with_bounded_diff;
use crate::helpers::get_subarray_length_bound_diff_at_most_k;
use crate::helpers::count_subarrays_with_bounded_sum;
use crate::helpers::count_subarrays_with_exact_sum_and_min_len;

// use std::io::{self};

pub struct View;

impl View {
    pub fn new() -> Self {
        View
    }

    // test renderer for the two_sum problem
    pub fn render_two_sum(&self) {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;

        match two_sum(&nums, target) {
            Some((a, b)) => {
                println!("The two numbers are at indices {} and {}.", a, b);
            }
            None => {
                println!("No two numbers add up to the target.");
            }
        }
    }

    pub fn render_group_anagrams(&self) {
        let words: Vec<&str> = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let grouped: Vec<Vec<String>> = group_anagrams(words);

        for group in grouped {
            println!("{:?}", group);
        }
    }

    pub fn render_contains_duplicates(&self) {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 1];
        println!(
            "Contains duplicates: {}",
            contains_duplicates(&nums),
        );
    }

    pub fn render_has_nearby_duplicate(&self) {
        let nums: Vec<i32> = vec![1, 0, 1, 1, 1, 1];
        let distance: i32 = 1;
        println!(
            "Has duplicates within distance: {}",
            has_nearby_duplicate(&nums, distance),
        );
    }

    pub fn render_has_nearby_divisible_duplicate(&self) {
        let nums: Vec<i32> = vec![3, 0, 3, 3, 2, 1, 5, 6, 4];
        let distance: i32 = 1;
        let divisor: i32 = 3;

        println!(
            "Has divisible duplicates within distance: {}",
            has_nearby_divisible_duplicate(&nums, distance, divisor),
        );
    }

    pub fn render_has_subarray_with_sum_multiple_of_k(&self) {
        let nums: Vec<i32> = vec![23, 2, 4, 6, 7];
        let target: i32 = 6;

        println!(
            "Has a subarray with sum multiple of k: {}",
            has_subarray_with_sum_multiple_of_k(&nums, target),
        );
    }

    pub fn render_next_larger_element_distances(&self) {
        let temps: Vec<i32> = vec![73, 74, 75, 71, 69, 72, 76, 73]; // temparature examples
        println!(
            "Next larger element distances: {:?}",
            next_larger_element_distances(&temps),
        );
    }

    pub fn render_max_sum_sliding_window(&self) {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 3;
        println!(
            "Max sum of subarray within an array using sliding window: {}",
            max_sum_sliding_window(&nums, k),
        );
    }

    pub fn render_max_sum_sliding_window_of_k_adaptive(&self) {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 3;
        println!(
            "Max sum of subarray within an array using sliding window of k (adaptive): {}",
            max_sum_sliding_window_of_k_adaptive(&nums, k),
        );
    }

    pub fn render_max_sum_with_prefix_sum(&self) {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 3;
        println!(
            "Max sum of subarray within an array using prefix sum: {}",
            max_sum_with_prefix_sum(&nums, k),
        );
    }

    pub fn render_get_length_of_shortest_window(&self) {
        let logs: Vec<i32> = vec![20, 30, 50, 90, 80, 20];
        let k: usize = 3;
        let threshold: i32 = 60;
        println!(
            "Shortest length of the sliding window: {}",
            get_length_of_shortest_window(&logs, k, threshold),
        );
    }

    pub fn render_get_max_stable_session_count(&self) {
        let sessions: Vec<i32> = vec![20, 10, 5, 25, 15, 5];
        let threshold:i32 = 40;
        println!(
            "Max stable session count: {}",
            get_max_stable_session_count(&sessions, threshold),
        );
    }

    pub fn render_count_subarrays_with_sum_k_positives_only(&self) {
        let logs: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let k: i32 = 15;
        println!(
            "Count of subarrays with sum k (positives only): {}",
            count_subarrays_with_sum_k_positives_only(&logs, k),
        );
    }

    pub fn render_count_subarrays_with_sum_k_any_values(&self) {
        let logs: Vec<i32> = vec![1, -2, -3, 4, 5, 6, -7, 8, -9, 10];
        let k: i32 = 15;
        println!(
            "Count of subarrays with sum k (any values): {}",
            count_subarrays_with_sum_k_any_values(&logs, k),
        );
    }

    pub fn render_count_valid_subarray_sum_of_k(&self) {
        let nums: Vec<i32> = vec![1, -1, 5, -2, 3];
        let k: i32 = 3;
        let min_len: isize = 2;
        println!(
            "Count of valid subarrays with sum k: {}",
            count_valid_subarray_sum_of_k(&nums, k, min_len),
        );
    }

    pub fn render_count_valid_subarray_sum_of_k_optimized(&self) {
        let nums: Vec<i32>= vec![1, -1, 5, -2, 3];
        let k: i32 = 3;
        let min_len: usize = 2;
        println!(
            "Count of valid subarrays with sum k optimized: {}",
            count_valid_subarray_sum_of_k_optimized(&nums, k, min_len),
        );
    }

    pub fn render_count_subarrays_with_bounded_diff(&self) {
        let nums: Vec<i32> = vec![1, -2, -3, 4, 5, 6, -7, 8, -9, 10];
        let k: i32 = 3;
        let min_len: usize = 2;
        println!(
            "Count subarrays with bounded diff: {}",
            count_subarrays_with_bounded_diff(&nums, k, min_len),
        );
    }

    pub fn render_get_subarray_length_bound_diff_at_most_k(&self) {
        let activities: Vec<i32> = vec![3, -1, 4, 2, 6, -2, 5, -3, 7];
        let k: i32 = 6;
        println!(
            "Longest subarray with max-min <= {}: {}",
            k,
            get_subarray_length_bound_diff_at_most_k(&activities, k),
        );
    }

    pub fn render_count_subarrays_with_bounded_sum(&self) {
        let nums: Vec<i32> = vec![1, -2, -3, 4, 5, 6, -7, 8, -9, 10];
        let k: i32 = 3;
        let min_len: usize = 2;
        println!(
            "Count subarrays with bounded sum: {}",
            count_subarrays_with_bounded_sum(&nums, k, min_len),
        );
    }

    pub fn render_count_subarrays_with_exact_sum_and_min_len(&self) {
        let nums: Vec<i32>= vec![1, -2, 4, 5, 6, -7, 8, 10];
        let k: i32= 3;
        let min_len: usize = 2;
        println!(
            "Count subarrays with exact sum and with minimun length: {}",
            count_subarrays_with_exact_sum_and_min_len(&nums, k, min_len),
        );
    }

}
