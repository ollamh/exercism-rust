extern crate regex;

use regex::Regex;

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match match_message(message) {
        "question" => { return "Sure.";}
        "exclamation" => { return "Whoa, chill out!"; }
        "silence" => "Fine. Be that way!",
        "anything_else" => "Whatever.",
        _ => ""
    }
}


pub fn match_message(message: &str) -> &str {
    let re = Regex::new(r"[a-zA-Z]").unwrap();
    let upper_message = message.to_uppercase();
    if message == upper_message  && re.is_match(message) {
        return "exclamation";
    }
    if message.ends_with('?') {
        return "question";
    }
    if message.len() == 0 {
        return "silence";
    }
    return "anything_else";
}
