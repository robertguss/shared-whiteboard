use regex::Regex;
use std::sync::LazyLock;

const PATTERNS: &[&str] = &[
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----(?s:.*?)-----END [A-Z ]*PRIVATE KEY-----",
    r"\b(?:AKIA|ASIA|ABIA|ACCA)[A-Z0-9]{16}\b",
    r"\bsk-[A-Za-z0-9_-]{20,}",
    r"\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36,}\b",
    r"\bgithub_pat_[A-Za-z0-9_]{22,}",
    r"\bxox[abprs]-[A-Za-z0-9-]{10,}",
    r"\bAIza[A-Za-z0-9_-]{35}",
    r"\b[sr]k_(?:live|test)_[A-Za-z0-9]{16,}",
    r"\beyJ[A-Za-z0-9_-]{10,}\.eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}",
    r"(?i)(\bbearer\s+)[A-Za-z0-9._~+/-]{16,}=*",
    r"(\b[a-z][a-z0-9+.-]*://[^\s:/@]+:)[^\s/@]+(@)",
    r#"(?i)(\b[a-z0-9_]*(?:key|token|secret|password)["']?\s*[:=]\s*["']?)[^\s"']{8,}"#,
];

static REGEXES: LazyLock<Vec<Regex>> =
    LazyLock::new(|| PATTERNS.iter().map(|p| Regex::new(p).unwrap()).collect());

pub fn mask(text: &str) -> String {
    REGEXES.iter().fold(text.to_string(), |text, regex| {
        regex.replace_all(&text, "${1}[masked]${2}").into_owned()
    })
}
