#[macro_export]
macro_rules! s {
    ($a:expr) => {
        $a.to_string()
    };
}

#[macro_export]
macro_rules! unescape {
    ($a:expr) => {
        $a.replace("\\b", r"\b")
            .replace("\\t", "\t")
            .replace("\\n", "\n")
            .replace("\\f", r"\f")
            .replace("\\r", "\r")
            .replace("\\'", "'")
            .replace("\\\"", "\"")
    };
}
