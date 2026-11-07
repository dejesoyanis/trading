/// Renders an error and its sources as a single line.
///
/// The chain is capped so a cyclic [`std::error::Error::source`] cannot loop.
pub fn error_chain(error: &(dyn std::error::Error + 'static)) -> String {
    let mut message = error.to_string();
    let mut source = error.source();
    let mut depth = 0_u8;
    while let Some(inner) = source {
        depth += 1;
        if depth > 16 {
            message.push_str(": ...");
            break;
        }
        message.push_str(": ");
        message.push_str(&inner.to_string());
        source = inner.source();
    }
    message
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::error_chain;

    #[derive(Debug)]
    struct Inner;

    impl fmt::Display for Inner {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("inner")
        }
    }

    impl std::error::Error for Inner {}

    #[derive(Debug)]
    struct Outer;

    impl fmt::Display for Outer {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("outer")
        }
    }

    impl std::error::Error for Outer {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&INNER)
        }
    }

    static INNER: Inner = Inner;

    #[test]
    fn includes_the_source_error() {
        assert_eq!(error_chain(&Outer), "outer: inner");
    }
}
