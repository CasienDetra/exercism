const VERSE_KEY_WORDS: [(&str, &str, &str, &str); 10] = [
    ("One", "bottle", "no", "bottles"),
    ("Two", "bottles", "one", "bottle"),
    ("Three", "bottles", "two", "bottles"),
    ("Four", "bottles", "three", "bottles"),
    ("Five", "bottles", "four", "bottles"),
    ("Six", "bottles", "five", "bottles"),
    ("Seven", "bottles", "six", "bottles"),
    ("Eight", "bottles", "seven", "bottles"),
    ("Nine", "bottles", "eight", "bottles"),
    ("Ten", "bottles", "nine", "bottles"),
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (start_bottles - take_down + 1..=start_bottles)
        .rev()
        .map(|n| get_verse((n - 1) as usize))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn get_verse(idx: usize) -> String {
    let (current_count, bottles_start, next_count, bottles_end) =
        &VERSE_KEY_WORDS[idx];

    format!(
"{current_count} green {bottles_start} hanging on the wall,
{current_count} green {bottles_start} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {next_count} green {bottles_end} hanging on the wall."
    )
}
