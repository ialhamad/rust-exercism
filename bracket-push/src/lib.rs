pub fn brackets_are_balanced(string: &str) -> bool {
    let mut close_brackets: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '{' => close_brackets.push('}'),
            '[' => close_brackets.push(']'),
            '(' => close_brackets.push(')'),
            '}' | ']' | ')' => {
                if close_brackets.last() == Some(&c) {
                    close_brackets.pop();
                } else {
                    close_brackets.push(c)
                }
            }
            _ => (),
        }
    }
    close_brackets.is_empty()
}
