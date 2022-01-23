// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();
    let mut notes_map = HashMap::new();

    // Create the magazine words hashmap
    for word in magazine {
        let current_word = word.to_string();
        let mag_word = magazine_map.entry(current_word).or_insert(0);
        *mag_word += 1;
    }

    // Create the note words hashmap
    for word in note {
        let current_word = word.to_string();
        let note_word = notes_map.entry(current_word).or_insert(0);
        *note_word += 1;
    }

    // Get a vector of the words in one map
    let notes_map_keys: Vec<String> = notes_map.clone().into_keys().collect();

    
    for word in notes_map_keys {
        if !magazine_map.contains_key(&word){
            return false;
        }
        if notes_map.get(&word).unwrap() > magazine_map.get(&word).unwrap() {
            return false;
        }
    }

    true
}