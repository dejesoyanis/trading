use url::Url;

/// Removes a connection string and its password from text that may be logged.
///
/// The full URL is replaced. The userinfo password is replaced both as stored
/// and percent-decoded. Query values other than that password are removed only
/// when they appear inside the full URL.
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
    use super::redact_connection_secrets;

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
}
