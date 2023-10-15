#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let is_first_sublist_of_second = is_sublist_of(_first_list, _second_list);
    let is_second_sublist_of_first = is_sublist_of(_second_list, _first_list);

    if is_first_sublist_of_second && is_second_sublist_of_first {
        return Comparison::Equal
    }

    if !is_first_sublist_of_second && is_second_sublist_of_first {
        return Comparison::Superlist;
    }

    if is_first_sublist_of_second && !is_second_sublist_of_first {
        return Comparison::Sublist;
    }

    return Comparison::Unequal;
}

fn is_sublist_of<T: PartialEq>(left: &[T], right: &[T]) -> bool {
    if left.is_empty() {
        return true;
    }

    if right.is_empty() {
        return false;
    }

    if left.len() > right.len() {
        return false;
    }

    for i in 0..right.len() - left.len() + 1 {
        let mut is_equal = true;
        for j in 0..left.len() {
            if left[j] != right[i+j] {
                is_equal = false;
            }
        }
        if is_equal {
            return true;
        }
    }
    return false;
}
