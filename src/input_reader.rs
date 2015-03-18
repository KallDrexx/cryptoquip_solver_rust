use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use types;
use fingerprinter;

pub fn read_input<T: Read>(mut reader: BufReader<T>) -> Vec<types::FingerprintedValue> {
	let mut fingerprints = Vec::new();
	let mut words = Vec::new();
	let mut line = String::new();

	let bytes_read = reader.read_line(&mut line).unwrap();
    if bytes_read == 0 {
        panic!("No data in input.txt");
    }

    let mut current_word = String::new();
    for character in line.chars() {
        // Split character on white space
        if character.is_whitespace() {
            if current_word.len() > 0 {
            	words.push(current_word);
                current_word = String::new();
                continue;
            }
        }

        current_word.push(character);
    }

    if words.len() > 0 {
    	fingerprints.push(types::FingerprintedValue {
    		fingerprinted: fingerprinter::get_fingerprint(&words[0][..]),
    		original: words[0].clone()
    	});
    }

    return fingerprints;
}

#[cfg(test)]
mod tests {
	use std::io::BufReader;
	use std::io::Read;
	use std::io::Cursor;
	use super::*;

	fn create_buf_reader(input: String) -> BufReader<Cursor<Vec<u8>>> {
		let bytes = input.into_bytes();
		let cursor = Cursor::new(bytes);
		return BufReader::new(cursor);
	}

	#[test]
	fn reads_and_fingerprints_single_value() {
		let input = "testing\n".to_string();
		let buf_reader = create_buf_reader(input);

		let output = read_input(buf_reader);
		assert_eq!(output.len(), 1);
		assert_eq!(output[0].original, "testing");
		assert_eq!(output[0].fingerprinted, "abcadef");
	}
}