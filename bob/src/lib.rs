pub fn reply(message: &str) -> &str {
    match matchMessage(message) {
        "question" => "Sure",
        "exclamation" => "Whoa, chill out!",
        "silence" => "Fine. Be that way!",
        "anything_else" => "Whatever."
    }
}


pub fn matchMessage(message: &str) -> *const str {
    if message.ends_with('?') {
        "question"
    }
    if message.ends_with('!') {
        "exclamation"
    }
    if message.len() == 0 {
        "silence"
    }
    "anything_else"
}
