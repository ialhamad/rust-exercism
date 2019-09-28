pub fn reply(message: &str) -> &str {
  let trimmed_message = message.trim();
  let has_alphabetic = trimmed_message.chars().any(char::is_alphabetic);
  let has_lowercase = trimmed_message.chars().any(char::is_lowercase);

  if trimmed_message.is_empty() {
    "Fine. Be that way!"
  } else if has_alphabetic && !has_lowercase && trimmed_message.ends_with('?') {
    "Calm down, I know what I'm doing!"
  } else if has_alphabetic && !has_lowercase {
    "Whoa, chill out!"
  } else if trimmed_message.ends_with('?') {
    "Sure."
  } else {
    "Whatever."
  }
}
