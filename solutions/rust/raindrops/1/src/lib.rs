const MAX_SOUND_LEN: usize = 15;
const SOUND_MAP: [(u32, &str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong"),
];
pub fn raindrops(n: u32) -> String {
    let mut sounds = String::with_capacity(MAX_SOUND_LEN);
    for (divisor, sound) in SOUND_MAP {
        if n.is_multiple_of(divisor) {
            sounds.push_str(sound);
        }
    }
    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}
