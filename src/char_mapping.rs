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

pub fn get_largest_map(maps: Vec<&HashMap<char, char>>) -> Option<&HashMap<char, char>> {
	let mut largest_map = None;
	for comparison_map in maps.into_iter() {
		if largest_map.is_none() {
			largest_map = Some(comparison_map);
		} else {
			if largest_map.unwrap().len() < comparison_map.len() {
				largest_map = Some(comparison_map);
			}
		}
	}

	return largest_map;
}

pub fn apply_mapping(input: &str, mapping: &HashMap<char, char>) -> String {
	let mut output = String::new();
	for character in input.chars() {
		match mapping.get(&character) {
			None => output.push(character),
			Some(x) => output.push(x.clone())
		};
	}

	return output;
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

	#[test]
	fn largest_map_is_returned() {
		let mut map1 = HashMap::new();
		let mut map2 = HashMap::new();
		let mut map3 = HashMap::new();
		let mut maps = Vec::new();

		map1.insert('a', 'b');
		map2.insert('b', 'c');
		map2.insert('c', 'd');
		map3.insert('d', 'e');

		maps.push(&map1);
		maps.push(&map2);
		maps.push(&map3);

		let result = get_largest_map(maps);
		assert!(result.is_some());
		assert_eq!(result.unwrap(), &map2);
	}

	#[test]
	fn correctly_returns_string_by_applying_mapping() {
		let input = "aba";
		let mut map = HashMap::new();
		map.insert('a', 'b');

		let result = apply_mapping(input, &map);
		assert_eq!(result, "bbb");
	}
}