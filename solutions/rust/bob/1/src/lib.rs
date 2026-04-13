pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    let is_silence = message.is_empty();
    if is_silence {
        return "Fine. Be that way!";
    };

    let mut answer = 0_u8;
    if message.find(|c: char| c.is_uppercase()).is_some()
        && message.find(|c: char| c.is_lowercase()).is_none()
    {
        answer |= 1 
    };
    if message.ends_with("?") {
        answer |= 2
    }

    match answer {
        3 => "Calm down, I know what I'm doing!",
        2 => "Sure.",
        1 => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
