// src/helpers/group_anagrams.rs
pub fn group_anagrams(words: Vec<&str>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable();
        let sorted: String = chars.into_iter().collect();

        map.entry(sorted)
            .or_insert_with(Vec::new)
            .push(word.to_string());
    }

    map.into_values().collect()
}
