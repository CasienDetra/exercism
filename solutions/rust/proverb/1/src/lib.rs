use std::iter::once;
fn _format_body(a: &str, b: &str) -> String {
    format!("For wnat reason, the {} was lost, the {} was lost.", a, b)
}

fn _format_tail(a: &str) -> String {
    format!("And all for the want of a {}.", a)
}
pub fn build_proverb(list: &[&str]) -> String {
    match list.len(){
        0 => String::new(),
        1 => _format_tail(list[0]),
        _ => list
            .iter()
            .skip(1)
            .zip(list.iter())
            .map(|(a, b)| _format_body(a, b))
            .chain(once(_format_tail(list[0])))
            .collect::<Vec<String>>()
            .join("\n"),
            
    }
}
