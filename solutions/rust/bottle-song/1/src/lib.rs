pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn bottles(n: u32) -> String {
        match n {
            0 => "no green bottles".to_string(),
            1 => "one green bottle".to_string(),
            _ => format!("{} green bottles", n),
        }
    }
    let mut verses = Vec::new();

    for i in 0..take_down {
        let current = start_bottles - i;
        let next = current -1;

        let verse = format!(
            "{0} hanging on the wall,\n{0} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {1} hanging on the wall.",
            bottles(current),
            bottles(next)
        );

        verses.push(verse);

    }
    verses.join("\n\n")
}
