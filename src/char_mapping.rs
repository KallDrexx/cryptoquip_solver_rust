use std::collections::hash_map::{HashMap, Entry};

pub fn update_mappings(input: &str, candidate: &str, mapping: &mut HashMap<char, char>) {
	if input.len() != candidate.len() {
		panic!("input and candidate are not the same length");
	}

	for x in (0..input.len()) {
		let input_char = input.char_at(x);
		let candidate_char = candidate.char_at(x);
		match mapping.entry(input_char) {
			Entry::Vacant(entry) => {entry.insert(candidate_char);},
			Entry::Occupied(_) => () /* ignore */
		}; 
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashMap;

	#[test]
	fn can_correctly_set_character_mapping() {
		let input = "abcda";
		let candidate = "wxyzw";
		let mut mapping = HashMap::new();

		update_mappings(input, candidate, &mut mapping);

		assert_eq!(mapping.len(), 4);
		assert_eq!(mapping.get(&'a'), Some(&'w'));
		assert_eq!(mapping.get(&'b'), Some(&'x'));
		assert_eq!(mapping.get(&'c'), Some(&'y'));
		assert_eq!(mapping.get(&'d'), Some(&'z'));
	}
}