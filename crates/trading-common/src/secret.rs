use std::fmt;

/// A string whose debug and display forms hide the contained value.
///
/// Call [`SecretString::expose`] only at a use site that genuinely needs the
/// raw value, such as opening a database connection. Do not log the result.
#[derive(Clone)]
pub struct SecretString(String);

impl SecretString {
    /// Stores a sensitive value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the raw value.
    ///
    /// The caller is responsible for keeping this out of logs.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretString([redacted])")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[redacted]")
    }
}

#[cfg(test)]
mod tests {
    use super::SecretString;

    #[test]
    fn debug_and_display_hide_the_value() {
        let secret = SecretString::new("super-secret-password");
        let debug = format!("{secret:?}");
        let display = secret.to_string();

        assert_eq!(secret.expose(), "super-secret-password");
        assert!(!debug.contains("super-secret-password"));
        assert!(!display.contains("super-secret-password"));
        assert!(debug.contains("[redacted]"));
        assert_eq!(display, "[redacted]");
    }
}
