use std::collections::hash_map::{HashMap, Entry};
use types;

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

pub fn get_compatibility(left: &HashMap<char, char>, right: &HashMap<char, char>) -> types::Compatibility {
	let mut total_matches = 0;
	for (left_key, left_value) in left.iter() {
		match right.get(left_key) {
			Some(right_value) => {
				if left_value != right_value { 
					return types::Compatibility::Incompatible 
				} else {
					total_matches = total_matches + 1;
				}
			},
			None => ()
		}
	}

	return types::Compatibility::TotalMatches(total_matches);
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::HashMap;
	use types;

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

	#[test]
	fn hash_maps_with_same_key_value_pairs_shows_total_matches() {
		let mut left = HashMap::new();
		let mut right = HashMap::new();

		left.insert('a', 'b');
		left.insert('b', 'c');
		left.insert('c', 'd');

		right.insert('d', 'e');
		right.insert('a', 'b');
		right.insert('c', 'd');

		let result = get_compatibility(&left, &right);
		assert_eq!(result, types::Compatibility::TotalMatches(2));
	}

	#[test]
	fn hash_maps_with_conflicting_values_are_incompatible() {
		let mut left = HashMap::new();
		let mut right = HashMap::new();

		left.insert('a', 'b');
		left.insert('b', 'c');

		right.insert('b', 'd');
		right.insert('a', 'b');

		let result = get_compatibility(&left, &right);
		assert_eq!(result, types::Compatibility::Incompatible);
	}
}