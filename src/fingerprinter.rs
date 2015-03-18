use std::collections::hash_map::{HashMap, Entry};

pub fn get_fingerprint(value: &str) -> String {
    let mut current_char = 'a';
    let mut new_value = String::new();
    let mut used_chars: HashMap<char, char> = HashMap::new();

    for character in value.chars() {
        let lowercase_character = character.to_lowercase().next().unwrap();
        match used_chars.entry(lowercase_character) {
            Entry::Occupied(entry) => new_value.push(*entry.get()),
            Entry::Vacant(entry) => {
                new_value.push(current_char);
                entry.insert(current_char);
                current_char = ((current_char as u8) + 1) as char;
            }
        }
    }

    return new_value;
}

#[test]
fn creates_correct_fingerprint() {
    let input = "testing";
    let expected_output = "abcadef".to_string();
    let actual_output = get_fingerprint(input);

    assert_eq!(expected_output, actual_output);
}