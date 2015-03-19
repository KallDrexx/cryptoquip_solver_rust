use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use types;
use fingerprinter;

pub fn get_candidates<T: Read>(mut reader: BufReader<T>, input_values: Vec<types::FingerprintedValue>) -> Vec<types::Candidate> {
	let mut candidates = Vec::new();

	// Pre-create the candidates
	for input in input_values.into_iter() {
		candidates.push(types::Candidate {
			input: input,
			possibilities: Vec::new()
		})
	}

	loop {
		let mut line = String::new();
		let bytes_read = reader.read_line(&mut line).unwrap();
		if bytes_read == 0 {
			break;
		}

		let mut trimmed_line = String::new();
        for character in line.chars() {
            if character.is_alphabetic() {
                trimmed_line.push(character);
            }
        }

        let dictionary_value = types::FingerprintedValue {
        	original: trimmed_line.clone(),
        	fingerprinted: fingerprinter::get_fingerprint(&trimmed_line[..])
        };

        for candidate in candidates.iter_mut() {
        	if candidate.input.fingerprinted == dictionary_value.fingerprinted {
        		candidate.possibilities.push(dictionary_value.clone());
        	}
        }
	}

	return candidates;
}

#[cfg(test)]
mod tests {
	use std::io::BufReader;
	use std::io::Cursor;
	use super::*;
	use types;

	fn create_buf_reader(dictionary: String) -> BufReader<Cursor<Vec<u8>>> {
		let bytes = dictionary.into_bytes();
		let cursor = Cursor::new(bytes);
		return BufReader::new(cursor);
	}

	#[test]
	fn gets_candidates_with_one_input() {
		let dictionary = "as\naa\nto\n".to_string();
		let buf_reader = create_buf_reader(dictionary);

		let mut input_values = Vec::new();
		input_values.push(types::FingerprintedValue {
			original: "me".to_string(),
			fingerprinted: "ab".to_string()
		});

		let output = get_candidates(buf_reader, input_values);

		assert_eq!(output.len(), 1);
		assert_eq!(output[0].input.original, "me");
		assert_eq!(output[0].input.fingerprinted, "ab");
		assert_eq!(output[0].possibilities.len(), 2);
		assert_eq!(output[0].possibilities[0].original, "as");
		assert_eq!(output[0].possibilities[0].fingerprinted, "ab");
		assert_eq!(output[0].possibilities[1].original, "to");
		assert_eq!(output[0].possibilities[1].fingerprinted, "ab");
	}
}