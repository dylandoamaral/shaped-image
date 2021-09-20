pub fn minimum_string_length(s: &str, length: usize, placeholder: &str) -> String {
    if s.len() >= length {
        return String::from(s);
    }
    let number_placeholder = length - s.len();
    return placeholder.repeat(number_placeholder) + s;
}
