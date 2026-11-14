use std::fmt;

use uuid::Uuid;

/// Identifier attached to one inbound HTTP request.
///
/// This is a process-local correlation handle. It is not a distributed trace.
/// It is also not the domain correlation id in `trading-domain`. That type is
/// separate, and neither crate converts one into the other.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorrelationId(Uuid);

impl CorrelationId {
    /// Generates a new random identifier.
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.hyphenated())
    }
}

#[cfg(test)]
mod tests {
    use super::CorrelationId;

    #[test]
    fn generated_ids_differ_and_use_uuid_text() {
        let first = CorrelationId::generate();
        let second = CorrelationId::generate();
        assert_ne!(first, second);

        let text = first.to_string();
        assert_eq!(text.len(), 36);
        assert!(
            text.chars()
                .all(|character| character.is_ascii_hexdigit() || character == '-')
        );
        assert!(uuid::Uuid::parse_str(&text).is_ok());
    }
}
