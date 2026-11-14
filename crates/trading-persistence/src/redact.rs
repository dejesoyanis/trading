use url::Url;

/// Removes a connection string and its password from text that may be logged.
///
/// The full URL is replaced. The userinfo password is replaced both as stored
/// and percent-decoded. Query values other than that password are removed only
/// when they appear inside the full URL.
/// Removes `postgres://` and `postgresql://` URLs from text that may be logged.
///
/// The replacement runs until the next whitespace. Use
/// [`redact_connection_secrets`] as well when the original URL is known, so a
/// password that appears outside the URL is removed too.
pub fn redact_database_urls(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = find_database_url(rest) {
        output.push_str(&rest[..start]);
        let after_scheme = if rest[start..].starts_with("postgresql://") {
            start + "postgresql://".len()
        } else {
            start + "postgres://".len()
        };
        let tail = &rest[after_scheme..];
        let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
        output.push_str("[redacted]");
        rest = &tail[end..];
    }
    output.push_str(rest);
    output
}

/// Removes a connection string and its password from text that may be logged.
pub fn redact_connection_secrets(text: &str, database_url: &str) -> String {
    let mut redacted = replace_non_empty(text, database_url);
    let Ok(parsed) = Url::parse(database_url) else {
        return redacted;
    };
    if let Some(password) = parsed.password() {
        redacted = replace_non_empty(&redacted, password);
        let decoded = percent_decode(password);
        if decoded != password {
            redacted = replace_non_empty(&redacted, &decoded);
        }
    }
    redacted
}

/// Renders an error chain and strips database URLs from each segment.
pub fn redact_error(error: &(dyn std::error::Error + 'static)) -> String {
    let mut message = redact_database_urls(&error.to_string());
    let mut source = error.source();
    let mut depth = 0_u8;
    while let Some(inner) = source {
        depth += 1;
        if depth > 8 {
            message.push_str(": ...");
            break;
        }
        message.push_str(": ");
        message.push_str(&redact_database_urls(&inner.to_string()));
        source = inner.source();
    }
    message
}

fn find_database_url(text: &str) -> Option<usize> {
    let postgres = text.find("postgres://");
    let postgresql = text.find("postgresql://");
    match (postgres, postgresql) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(index), None) | (None, Some(index)) => Some(index),
        (None, None) => None,
    }
}

fn replace_non_empty(text: &str, needle: &str) -> String {
    if needle.is_empty() {
        text.to_owned()
    } else {
        text.replace(needle, "[redacted]")
    }
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = &input[index + 1..index + 3];
            if let Ok(byte) = u8::from_str_radix(hex, 16) {
                decoded.push(byte);
                index += 3;
                continue;
            }
        }
        decoded.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

#[cfg(test)]
mod tests {
    use super::{redact_connection_secrets, redact_database_urls, redact_error};

    #[test]
    fn redacts_the_url_and_leaves_the_username() {
        let url = "postgres://trading:super-secret-password@127.0.0.1:5432/trading";
        let text = format!("user trading failed with {url}");
        let redacted = redact_connection_secrets(&text, url);

        assert!(redacted.contains("user trading"));
        assert!(redacted.contains("[redacted]"));
        assert!(!redacted.contains("super-secret-password"));
        assert!(!redacted.contains(url));
    }

    #[test]
    fn redacts_percent_encoded_passwords() {
        let url = "postgres://trading:super%20secret@127.0.0.1:5432/trading";
        let text = "driver reported password super secret";
        let redacted = redact_connection_secrets(text, url);

        assert!(!redacted.contains("super secret"));
        assert!(!redacted.contains("super%20secret"));
        assert!(redacted.contains("[redacted]"));
    }

    #[test]
    fn strips_database_urls_without_a_known_secret() {
        let text = "connect postgres://trading:super-secret-password@127.0.0.1:5432/trading failed, then postgresql://other:secret@db/app";
        let redacted = redact_database_urls(text);

        assert_eq!(redacted, "connect [redacted] failed, then [redacted]");
        assert!(!redacted.contains("super-secret-password"));
        assert!(!redacted.contains("secret@"));
    }

    #[test]
    fn redacts_urls_inside_an_error_chain() {
        #[derive(Debug)]
        struct Leak(String);

        impl std::fmt::Display for Leak {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl std::error::Error for Leak {}

        let error = Leak(
            "driver said postgres://trading:super-secret-password@127.0.0.1/trading".to_owned(),
        );
        let redacted = redact_error(&error);
        assert!(redacted.contains("[redacted]"));
        assert!(!redacted.contains("super-secret-password"));
        assert!(!redacted.contains("postgres://"));
    }
}
