pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        let mut proverb = (1..list.len())
            .map(|i| format!("For want of a {} the {} was lost.\n", list[i - 1], list[i]))
            .collect::<Vec<String>>()
            .join("");
        proverb.push_str(&format!("And all for the want of a {}.", list[0]));
        proverb
    }
}
