pub fn build_proverb(list: &[&str]) -> String {
    fn build_proverb_line(list: &[&str], i: usize) -> String {
        if i == list.len() - 1 {
            format!("And all for the want of a {}.", list[0])
        } else {
            format!(
                "For want of a {} the {} was lost.",
                list[i],
                list[i + 1]
            )
        }
    }
    (0..list.len())
        .map(|i| build_proverb_line(list, i))
        .collect::<Vec<String>>()
        .join("\n")
}
