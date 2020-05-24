#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut result = Comparison::Unequal;

    if _first_list.len() <= _second_list.len() && do_contain_list(_first_list, _second_list) {
        if _first_list.len() < _second_list.len() {
            result = Comparison::Sublist
        } else {
            result = Comparison::Equal;
        }
    } else if _first_list.len() > _second_list.len() && do_contain_list(_second_list, _first_list) {
        result = Comparison::Superlist;
    }

    result
}

fn do_contain_list<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() || b.is_empty() {
        return true;
    }
    
    let mut result = false;
    for (index_b, item_b) in b.iter().enumerate() {
        if item_b == a.first().unwrap() {
            if index_b + a.len() > b.len() {
                break; // No matches for sure
            }
            // Extract slice
            let slice_b = &b[index_b..index_b + a.len()];
            if slice_b == a {
                result = true;
                break;
            }
        }
    }
    result
}
