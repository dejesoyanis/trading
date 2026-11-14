use std::fmt;

use uuid::Uuid;

/// Process-local identifier for one inbound HTTP request.
///
/// This is not a distributed trace. It is not
/// `trading_domain::CorrelationId`. Nothing in this crate converts one into
/// the other.
///
/// The HTTP response header remains `x-correlation-id`. That name is the
/// existing wire format. It does not make this value a domain correlation id.
/// `Display` is the header value: lowercase hyphenated UUID text. The previous
/// type name in this crate was removed so the two identifiers are not exported
/// under one unqualified name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpRequestId(Uuid);

impl HttpRequestId {
    /// Generates a new random identifier.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for HttpRequestId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.hyphenated())
    }
}

#[cfg(test)]
mod tests {
    use super::HttpRequestId;

    #[test]
    fn generated_ids_differ_and_use_uuid_text() {
        let first = HttpRequestId::generate();
        let second = HttpRequestId::generate();
        assert_ne!(first, second);

        let text = first.to_string();
        assert_eq!(text.len(), 36);
        assert_eq!(text.as_bytes()[8], b'-');
        assert_eq!(text.as_bytes()[13], b'-');
        assert_eq!(text.as_bytes()[18], b'-');
        assert_eq!(text.as_bytes()[23], b'-');
        assert!(
            text.chars()
                .all(|character| character.is_ascii_hexdigit() || character == '-')
        );
        assert!(uuid::Uuid::parse_str(&text).is_ok());
        assert!(
            std::any::type_name::<HttpRequestId>().ends_with("HttpRequestId"),
            "the HTTP type must not be named CorrelationId"
        );
    }
}
