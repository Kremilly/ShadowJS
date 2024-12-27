pub struct RegExp;

impl RegExp {
    
    pub const SPACES: &'static str = r"\s+";
    pub const COMMENTS: &'static str = r"(?s)/\*.*?\*/|//.*?$";
    pub const IDENTIFIERS: &'static str = r"\b[a-zA-Z_][a-zA-Z0-9_]*\b";
    
}
