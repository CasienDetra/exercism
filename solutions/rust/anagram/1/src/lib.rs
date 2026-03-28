use std::collections::HashSet;

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &'a [&'a str],
) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for i in possible_anagrams {
        if is_anagram(word, i) {
            anagrams.insert(i);
        }
    }
    anagrams
}
fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    if word.to_lowercase() == possible_anagram.to_lowercase() {
        return false;
    }
    sort_lowercase(word) == sort_lowercase(possible_anagram)
}
fn sort_lowercase(word: &str) -> Vec<char> {
    let mut word_chars = word.to_lowercase().chars().collect::<Vec<char>>();
    word_chars.sort_unstable();
    word_chars
}
