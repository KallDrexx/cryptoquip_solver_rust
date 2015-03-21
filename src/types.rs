use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct FingerprintedValue {
    pub original: String,
    pub fingerprinted: String
}

pub struct InputValue {
	pub input: FingerprintedValue,
	pub possibilities: Vec<Possibility> 
}

#[derive(Clone)]
pub struct Possibility {
	pub fingerprint: FingerprintedValue,
	pub char_mapping: HashMap<char, char>
}

#[derive(PartialEq)]
pub enum Compatibility {
	TotalMatches(i32),
	Incompatible
}

impl fmt::Debug for Compatibility {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let result = match *self {
			Compatibility::Incompatible => write!(f, "Incompatible"),
			Compatibility::TotalMatches(x) => write!(f, "Total Matches: {}", x)
		};

		return result;
	}
}