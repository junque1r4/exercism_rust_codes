#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn inside<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    let second_size = first.len();

    second_size == 0 || first.windows(second_size).any(|list| list == second)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (inside(_first_list, _second_list), inside(_second_list, _first_list)) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
