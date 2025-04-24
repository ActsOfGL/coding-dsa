// src/view/view.rs [renderer]

use crate::helpers::two_sum;
use crate::helpers::group_anagrams;
use crate::helpers::contains_duplicates;
use crate::helpers::has_nearby_duplicate;
use crate::helpers::has_nearby_divisible_duplicate;
use crate::helpers::has_subarray_with_sum_multiple_of_k;
use crate::helpers::next_larger_element_distances;
use crate::helpers::max_sum_sliding_window;
use crate::helpers::max_sum_with_prefix_sum;

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
        let nums = vec![1, 2, 3, 4, 5, 1];
        println!("Contains duplicates: {}", contains_duplicates(&nums));
    }

    pub fn render_has_nearby_duplicate(&self) {
        let nums = vec![1, 0, 1, 1, 1, 1];
        let distance = 1;
        println!(
            "Has duplicates within distance: {}",
            has_nearby_duplicate(&nums, distance)
        );
    }

    pub fn render_has_nearby_divisible_duplicate(&self) {
        let nums = vec![3, 0, 3, 3, 2, 1, 5, 6, 4];
        let distance = 1;
        let divisor = 3;

        println!(
            "Has divisible duplicates within distance: {}",
            has_nearby_divisible_duplicate(&nums, distance, divisor)
        );
    }

    pub fn render_has_subarray_with_sum_multiple_of_k(&self) {
        let nums = vec![23, 2, 4, 6, 7];
        let target = 6;

        println!(
            "Has a subarray with sum multiple of k: {}",
            has_subarray_with_sum_multiple_of_k(&nums, target)
        );
    }

    pub fn render_next_larger_element_distances(&self) {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73]; // temparature examples
        println!(
            "Next larger element distances: {:?}",
            next_larger_element_distances(&temps)
        );
    }

    pub fn render_max_sum_sliding_window(&self) {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        println!(
            "Max sum of subarray within an array using sliding window: {}",
            max_sum_sliding_window(&nums, 3)
        );
    }

    pub fn render_max_sum_with_prefix_sum(&self) {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        println!(
            "Max sum of subarray within an array using prefix sum: {}",
            max_sum_with_prefix_sum(&nums, 3)
        );
    }
}
