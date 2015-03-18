use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

static INPUT_FILE: &'static str = "input.txt";

pub fn read() -> Vec<String> {
    let mut values = Vec::new();

    let path = Path::new(INPUT_FILE);
    let mut file = BufReader::new(File::open(&path).unwrap());
    let mut line = String::new();

    let bytes_read = file.read_line(&mut line).unwrap();
    if bytes_read == 0 {
        panic!("No data in input.txt");
    }

    let mut current_word = String::new();
    for character in line.chars() {
        // Split character on white space
        if character.is_whitespace() {
            if current_word.len() > 0 {
                values.push(current_word);
                current_word = String::new();
                continue;
            }
        }

        current_word.push(character);
    }

    if current_word.len() > 0 {
        values.push(current_word);
    }

    return values;
}