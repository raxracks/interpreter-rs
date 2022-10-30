#[macro_export]
macro_rules! s {
    ($a:expr) => {
        $a.to_string()
    };
}

pub fn unescape(string: String) -> String {
    return string
        .replace("\\b", r"\b")
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\f", r"\f")
        .replace("\\r", "\r")
        .replace("\\'", "'")
        .replace("\\\"", "\"");
}
