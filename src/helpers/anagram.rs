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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let words: Vec<&str> = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut expected: Vec<Vec<String>> = vec![
            vec!["eat", "tea", "ate"],
            vec!["tan", "nat"],
            vec!["bat"]
        ].into_iter()
           .map(|group| group.into_iter().map(String::from).collect())
           .collect();
        let mut result: Vec<Vec<String>> = group_anagrams(words);
        for group in &mut expected {
            group.sort();
        }
        for group in &mut result {
            group.sort();
        }
        expected.sort();
        result.sort();
        assert_eq!(expected, result);
    }
}