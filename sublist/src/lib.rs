#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn inside<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    first.is_empty() || second.windows(first.len()).any(|list| list == first)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list { return Comparison::Equal;}

    if inside(_first_list, _second_list) { return Comparison::Sublist; }

    if inside(_second_list, _first_list) { return Comparison::Superlist;}

    return Comparison::Unequal;
    
}
