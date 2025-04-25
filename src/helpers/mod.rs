// src/helpers/mod.rs [module]

pub mod two_sum;
pub mod anagram;
pub mod contains_duplicates;
pub mod has_nearby_duplicate;
pub mod has_nearby_divisible_duplicate;
pub mod has_subarray_with_sum_multiple_of_k;
pub mod next_larger_element_distances;
pub mod max_sum_sliding_window;
pub mod max_sum_with_prefix_sum;
pub mod get_length_of_shortest_window;

// add other helper functions here
pub use two_sum::two_sum;
pub use anagram::group_anagrams;
pub use contains_duplicates::contains_duplicates;
pub use has_nearby_duplicate::has_nearby_duplicate;
pub use has_nearby_divisible_duplicate::has_nearby_divisible_duplicate;
pub use has_subarray_with_sum_multiple_of_k::has_subarray_with_sum_multiple_of_k;
pub use next_larger_element_distances::next_larger_element_distances;
pub use max_sum_sliding_window::max_sum_sliding_window;
pub use max_sum_with_prefix_sum::max_sum_with_prefix_sum;
pub use get_length_of_shortest_window::get_length_of_shortest_window;
