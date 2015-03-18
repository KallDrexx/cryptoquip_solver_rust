mod types;
mod fingerprinter;
mod input_reader;

mod utils {
    pub mod dictionary_reader;
    pub mod cryptoquip_input_reader;
}

static INPUT_FILE: &'static str = "input.txt";

use std::fs::File;
use std::path::Path;

fn main() {
    //let path = Path::new(INPUT_FILE);
    //let mut file = BufReader::new(File::open(&path).unwrap());
    //input_reader
}
