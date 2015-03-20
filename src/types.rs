#[derive(Clone)]
pub struct FingerprintedValue {
    pub original: String,
    pub fingerprinted: String
}

pub struct InputValue {
	pub input: FingerprintedValue,
	pub possibilities: Vec<FingerprintedValue> 
}