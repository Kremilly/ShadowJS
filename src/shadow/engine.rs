use regex::Regex;
use rand::{Rng, thread_rng, distributions::Alphanumeric};
use std::{fs, collections::HashMap};

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

    fn is_reserved_word(&self, word: &str) -> bool {
        const RESERVED_WORDS: &[&str] = &[
            "break", "case", "catch", "class", "const", "continue", "debugger", "default",
            "delete", "do", "else", "enum", "export", "extends", "false", "finally", "for",
            "function", "if", "import", "in", "instanceof", "new", "null", "return", "super",
            "switch", "this", "throw", "true", "try", "typeof", "var", "void", "while", "with",
            "yield", "let", "static", "implements", "interface", "package", "private", "protected",
            "public",
        ];

        RESERVED_WORDS.contains(&word)
    }

    fn generate_random_identifier(&self) -> String {
        let mut rng = thread_rng();
        let random_string: String = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        format!("_{}", random_string)
    }

    fn obsfucator(&self) -> String {
        let content = fs::read_to_string(&self.input).expect("Error reading input file.");

        let mut replacements: HashMap<String, String> = HashMap::new();
        let re_identifiers = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();

        let obfuscated = re_identifiers.replace_all(&content, |caps: &regex::Captures| {
            let identifier = &caps[0];

            if self.is_reserved_word(identifier) {
                return identifier.to_string();
            }

            if let Some(replacement) = replacements.get(identifier) {
                return replacement.clone();
            }

            let random_identifier = self.generate_random_identifier();
            replacements.insert(identifier.to_string(), random_identifier.clone());
            random_identifier
        });

        obfuscated.into_owned()
    }

    fn minify(&self) -> String {
        let input = &self.obsfucator();
        let re_space = Regex::new(r"\s+").unwrap();
        let minified = re_space.replace_all(input, " ");
    
        let re_comments = Regex::new(r"(?s)/\*.*?\*/|//.*?$").unwrap();
        let minified = re_comments.replace_all(&minified, "");
    
        minified.trim().to_string()
    }

    pub fn run(&self) {
        let minified = self.minify();

        fs::write(&self.output, minified).expect("Error writing output file.");
        println!("Obfuscated and minified successfully! File saved to {}", &self.output);
    }

}
