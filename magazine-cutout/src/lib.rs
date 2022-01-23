// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

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

    for word in note {
        let current_word = word.to_string();
        let note_word = notes_map.entry(current_word).or_insert(0);
        *note_word += 1;
    }


    let mut notes_map_keys: Vec<String> = notes_map.clone().into_keys().collect();
    let mut magazine_map_keys: Vec<String> = magazine_map.clone().into_keys().collect();

    notes_map_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    magazine_map_keys.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    
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