use regex::Regex;

pub struct JS;

impl JS {

    pub fn is_reserved_word(&self, word: &str) -> bool {
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

    pub fn minify(&self, input: &str) -> String {
        let re_space = Regex::new(r"\s+").unwrap();
        let minified = re_space.replace_all(input, " ");
    
        let re_comments = Regex::new(r"(?s)/\*.*?\*/|//.*?$").unwrap();
        let minified = re_comments.replace_all(&minified, "");
    
        minified.trim().to_string()
    }

}
