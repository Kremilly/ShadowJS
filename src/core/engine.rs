use regex::Regex;

use std::{
    fs, 
    collections::HashMap
};

use crate::utils::{
    js::JS,
    random::Random
};

pub struct Engine {
    input: String,
    output: String,
}

impl Engine {

    pub fn new(input: &str, output: &str) -> Self {
        Self {
            input: input.to_string(),
            output: output.to_string(),
        }
    }

    fn obsfucator(&self) -> String {
        let content = fs::read_to_string(&self.input).expect("Error reading input file.");

        let mut replacements: HashMap<String, String> = HashMap::new();
        let re_identifiers = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();

        let obfuscated = re_identifiers.replace_all(&content, |caps: &regex::Captures| {
            let identifier = &caps[0];

            if JS.is_reserved_word(identifier) {
                return identifier.to_string();
            }

            if let Some(replacement) = replacements.get(identifier) {
                return replacement.clone();
            }

            let random_identifier = Random.generate_random_identifier();
            replacements.insert(identifier.to_string(), random_identifier.clone());
            random_identifier
        });

        obfuscated.into_owned()
    }

    pub fn run(&self) {
        let input = &self.obsfucator();
        let minified = JS.minify(input);

        fs::write(&self.output, minified).expect("Error writing output file.");
        println!("Obfuscated and minified successfully! File saved to {}", &self.output);
    }

}
