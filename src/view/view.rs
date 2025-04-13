// src/view/view.rs [renderer]

use crate::helpers::*;
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

        match two_sum(nums, target) {
            Some((a, b)) => {
                println!("The two numbers are at indices {} and {}.", a, b);
            }
            None => {
                println!("No two numbers add up to the target.");
            }
        }
    }
}
