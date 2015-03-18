use utils;
mod types;
mod fingerprinter;

pub fn execute() {
    let dictionary_values = utils::dictionary_reader::read();
    let input_values = utils::cryptoquip_input_reader::read();

    let fingerprinted_input = get_fingerprinted_values(input_values);
    //println!("{}", fingerprinted_input[0].fingerprinted);
    let test = fingerprinter::get_fingerprint("testingg");
    println!("{}", test);
}

fn get_fingerprinted_values(mut originals: Vec<String>) -> Vec<types::FingerprintedValue> {
    let mut result = Vec::new();

    loop {
        match originals.pop() {
            Some(x) => {
                let new_value = types::FingerprintedValue { 
                    original: x.clone(),
                    fingerprinted: fingerprinter::get_fingerprint(&x[..])
                };
                result.push(new_value);
            },
            None => break
        };
    }

    return result;
}