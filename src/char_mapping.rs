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

pub fn get_map_concatenation(maps: Vec<&HashMap<char, char>>) -> Option<HashMap<char, char>> {
	let mut concatenated_map = HashMap::new();
	for comparison_map in maps.into_iter() {
		for (comparison_key, comparison_value) in comparison_map.iter() {
			match concatenated_map.insert(comparison_key.clone(), comparison_value.clone()) {
				None => (), // New value, it's all good
				Some(x) => {
					if &x != comparison_value {
						return None;
					}
				}
			}
		}
	}

	return Some(concatenated_map);
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

	#[test]
	fn can_concat_compatible_maps() {
		let mut map1 = HashMap::new();
		let mut map2 = HashMap::new();
		let mut map3 = HashMap::new();
		let mut maps = Vec::new();

		map1.insert('a', 'b');
		map1.insert('b', 'c');
		map2.insert('b', 'c');
		map2.insert('c', 'd');
		map3.insert('d', 'e');

		maps.push(&map1);
		maps.push(&map2);
		maps.push(&map3);

		let result = get_map_concatenation(maps).unwrap();
		assert_eq!(result.len(), 4);
		assert_eq!(result.get(&'a'), Some(&'b'));
		assert_eq!(result.get(&'b'), Some(&'c'));
		assert_eq!(result.get(&'c'), Some(&'d'));
		assert_eq!(result.get(&'d'), Some(&'e'));
	}

	#[test]
	fn incompatible_maps_return_none() {
		let mut map1 = HashMap::new();
		let mut map2 = HashMap::new();
		let mut maps = Vec::new();

		map1.insert('a', 'b');
		map1.insert('b', 'c');
		map2.insert('b', 'd');
		map2.insert('c', 'd');

		maps.push(&map1);
		maps.push(&map2);

		let result = get_map_concatenation(maps);
		assert!(result.is_none())
	}
}