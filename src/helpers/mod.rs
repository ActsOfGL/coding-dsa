// src/helpers/mod.rs [module]

pub mod two_sum;
pub mod anagram;
pub mod contains_duplicates;
pub mod has_nearby_duplicate;

// add other helper functions here
pub use two_sum::two_sum;
pub use anagram::group_anagrams;
pub use contains_duplicates::contains_duplicates;
pub use has_nearby_duplicate::has_nearby_duplicate;
