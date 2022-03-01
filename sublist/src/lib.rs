#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Clone>(_first_list: &[T], _second_list: &[T]) -> Comparison {
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

    if first_length == second_length {
        if f_list == s_list {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    } else if first_length > second_length {
        for item in s_list {
            if !f_list.contains(&item) {
                return Comparison::Unequal;
            }
        }
        return Comparison::Superlist;
    } else if first_length < second_length {
        for item in f_list {
            if !s_list.contains(&item) {
                return Comparison::Unequal;
            }
        }
        return Comparison::Sublist;
    }
    return Comparison::Unequal;
}
