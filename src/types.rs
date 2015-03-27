use std::collections::HashMap;

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