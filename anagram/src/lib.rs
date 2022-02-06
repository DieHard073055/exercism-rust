use std::collections::HashMap;
use std::collections::HashSet;

fn create_map_for(word: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for letter in word.to_lowercase().chars() {
        let entry_letter = map.entry(letter.clone()).or_insert(0);
        *entry_letter += 1;
    }
    map
}

fn is_valid_anagram(word: &str, potential_anagram: &str) -> bool {
    // both words should be of same length
    if word.len() != potential_anagram.len() {
        return false;
    }
    // same word is not allowed to be an anagram
    if word.to_lowercase() == potential_anagram.to_lowercase() {
        return false;
    }
    let anagram_word_map = create_map_for(word);
    let potential_anagram_map = create_map_for(potential_anagram);

    let potential_anagram_keys: Vec<char> = potential_anagram_map.clone().into_keys().collect();
    // ensure we have the required letters to complete the anagram
    for letter in potential_anagram_keys {
        if !anagram_word_map.contains_key(&letter) {
            return false;
        }
        if potential_anagram_map.get(&letter).unwrap() > anagram_word_map.get(&letter).unwrap() {
            return false;
        }
    }
    true
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut confirmed_anagrams = HashSet::new();
    for potential_anagram in possible_anagrams {
        if is_valid_anagram(word, potential_anagram) {
            confirmed_anagrams.insert(potential_anagram.clone());
        }
    }

    confirmed_anagrams
}
