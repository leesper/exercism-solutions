use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().cloned().filter(|item| {
        if item.to_lowercase() == word.to_lowercase() {
            false
        } else {
            let mut item_chars = item.to_lowercase().chars().collect::<Vec<_>>();
            let mut word_chars = word.to_lowercase().chars().collect::<Vec<_>>();
            item_chars.sort_unstable();
            word_chars.sort_unstable();
            if item_chars == word_chars {
                return true;
            }
            false
        }
    }).collect()
}
