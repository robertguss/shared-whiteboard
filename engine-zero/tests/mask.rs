use engine_zero::mask::mask;

const MASKED: &[(&str, &str)] = &[
    ("aws key AKIAAAAAAAAAAAAAAAAA here", "aws key [masked] here"),
    (
        "model key sk-ant-api03-AAAAAAAAAAAAAAAAAAAAAAAA end",
        "model key [masked] end",
    ),
    (
        "model key sk-proj-AAAAAAAAAAAAAAAAAAAAAAAA end",
        "model key [masked] end",
    ),
    (
        "token ghp_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA end",
        "token [masked] end",
    ),
    (
        "token github_pat_AAAAAAAAAAAAAAAAAAAAAAAAAA end",
        "token [masked] end",
    ),
    ("slack xoxb-1111111111-AAAAAAAAAA end", "slack [masked] end"),
    (
        "google AIzaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA end",
        "google [masked] end",
    ),
    (
        "stripe sk_live_AAAAAAAAAAAAAAAAAAAA end",
        "stripe [masked] end",
    ),
    (
        "jwt eyJAAAAAAAAAAAA.eyJAAAAAAAAAAAA.AAAAAAAAAAAA end",
        "jwt [masked] end",
    ),
    (
        "-----BEGIN RSA PRIVATE KEY-----\nAAAA\nAAAA\n-----END RSA PRIVATE KEY-----",
        "[masked]",
    ),
    (
        "Authorization: Bearer AAAAAAAAAAAAAAAAAAAA",
        "Authorization: Bearer [masked]",
    ),
    (
        "clone https://someone:notarealpassword@example.com/repo",
        "clone https://someone:[masked]@example.com/repo",
    ),
    (
        "export DATABASE_PASSWORD=\"aaaaaaaaaaaa\"",
        "export DATABASE_PASSWORD=\"[masked]\"",
    ),
    ("\"api_key\": \"aaaaaaaaaaaa\"", "\"api_key\": \"[masked]\""),
];

const LEFT_ALONE: &[&str] = &[
    "commit 3f2a9c1e5b7d4f6a8c0e2b4d6f8a1c3e5b7d9f0a",
    "aws AKIASHORT",
    "a risk-assessment-for-the-quarterly-plan",
    "token ghp_short",
    "token github_pat_short",
    "slack xoxb-123",
    "google AIzaShort",
    "stripe sk_test_short",
    "jwt eyJ alone",
    "-----BEGIN PUBLIC KEY-----\nAAAA\n-----END PUBLIC KEY-----",
    "the bearer of bad news",
    "see https://example.com/path@v2 and git@example.com:org/repo",
    "MAX_TOKENS=40960000",
];

#[test]
fn each_secret_shape_is_masked() {
    for (secret, masked) in MASKED {
        assert_eq!(mask(secret), *masked, "{secret}");
    }
}

#[test]
fn near_misses_are_left_alone() {
    for text in LEFT_ALONE {
        assert_eq!(mask(text), *text);
    }
}
