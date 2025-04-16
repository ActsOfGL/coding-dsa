// src/view/view.rs [renderer]

use crate::helpers::two_sum;
use crate::helpers::group_anagrams;
use crate::helpers::contains_duplicates;
use crate::helpers::has_nearby_duplicate;
use crate::helpers::has_nearby_divisible_duplicate;

// use std::io::{self};

pub struct View;

impl View {
    pub fn new() -> Self {
        View
    }

    // test renderer for the two_sum problem
    pub fn render_two_sum(&self) {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

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
        let words = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let grouped = group_anagrams(words);

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
}
