use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

static DICTIONARY_FILE: &'static str = "en-US.dic";

pub fn read() -> Vec<String> {
    let mut values = Vec::new();

    let path = Path::new(DICTIONARY_FILE);
    let mut file = BufReader::new(File::open(&path).unwrap());

    loop {
        let mut line = String::new();
        let bytes_read = file.read_line(&mut line).unwrap();
        if bytes_read > 0 {
            let mut trimmed_line = String::new();
            for character in line.chars() {
                if character.is_alphabetic() {
                    trimmed_line.push(character);
                }
            }

            values.push(trimmed_line);
        }
        else {
            break;
        }
    }
   
    return values;
}