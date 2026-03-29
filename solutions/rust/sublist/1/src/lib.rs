use Comparison::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T : PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() && first_list.iter().zip(second_list.iter()).all(|(a, b)| a == b) {
        Equal
    } else if first_list.len() < second_list.len() && second_list.windows(first_list.len()).any(|window| window.iter().zip(first_list.iter()).all(|(a, b)| a == b)) {
        Sublist
    } else if first_list.len() > second_list.len() && first_list.windows(second_list.len()).any(|window| window.iter().zip(second_list.iter()).all(|(a, b)| a == b)) {
        Superlist
    } else {
        Unequal
    }
}
pub fn is_sublist<T: PartialEq>(smaller: &[T], bigger: &[T]) -> bool {
    if smaller.is_empty() {
        return true;
    }

    let indices = bigger
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if item == &smaller[0] {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    indices.iter().any(|&i| {
        let sub = &bigger[i..];
        sub.len() >= smaller.len()
            && smaller
                .iter()
                .zip(sub.iter())
                .all(|(a, b)| a == b)
    })
}
