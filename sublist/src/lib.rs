use std::{collections::HashMap, hash::Hash};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Clone + Eq + Hash>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // empty == empty
    if _first_list.is_empty() && _second_list.is_empty() {
        return Comparison::Equal;
    }

    // empty is a sublist of anything
    if _first_list.is_empty() && !_second_list.is_empty() {
        return Comparison::Sublist;
    }

    // anything is a superlist of empty
    if !_first_list.is_empty() && _second_list.is_empty() {
        return Comparison::Superlist;
    }

    let first_length = _first_list.len();
    let second_length = _second_list.len();
    let f_list = _first_list.to_vec();
    let s_list = _second_list.to_vec();
    
    let mut f_map = HashMap::new();
    let mut s_map = HashMap::new();

    for item in s_list.clone() {
        let inserted_item = s_map.entry(item).or_insert(0);
        *inserted_item += 1;
    }
    for item in f_list.clone() {
        let inserted_item = f_map.entry(item).or_insert(0);
        *inserted_item += 1;
    }

    let s_map_keys: Vec<T> = s_map.clone().into_keys().collect();
    let f_map_keys: Vec<T> = f_map.clone().into_keys().collect();
    let s_map_keys_len = s_map_keys.len();
    let f_map_keys_len = f_map_keys.len();

    if first_length == second_length && s_map_keys_len == f_map_keys_len{
        for key in f_map_keys {
            if !s_map.contains_key(&key){
                return Comparison::Unequal;
            }
            if s_map.get(&key).unwrap() != f_map.get(&key).unwrap() {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    } else if first_length > second_length {
        for key in s_map_keys {
            if !f_map.contains_key(&key){
                return Comparison::Unequal;
            }

        }
        return Comparison::Superlist;
    } else if first_length < second_length {
        for key in f_map_keys {
            if !s_map.contains_key(&key){
                return Comparison::Unequal;
            }
        }
        return Comparison::Sublist;
    }
    return Comparison::Unequal;
}
