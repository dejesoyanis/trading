use crate::error::AppError;

/// What the process should do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Serve liveness and read-only readiness.
    Serve,
    /// Apply infrastructure migrations and exit.
    Migrate,
}

/// Selects a command from process arguments.
///
/// The first value is the program name. No further arguments starts the
/// server. `migrate` with no further arguments applies migrations. Any other
/// argument is rejected.
///
/// # Errors
///
/// Returns [`AppError::Usage`] when the arguments are not recognized.
pub fn parse_args<I, S>(args: I) -> Result<Command, AppError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();
    let _program = args.next();
    match args.next() {
        None => Ok(Command::Serve),
        Some(command) if command.as_ref() == "migrate" && args.next().is_none() => {
            Ok(Command::Migrate)
        }
        Some(_) => Err(AppError::Usage),
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_args};
    use crate::error::AppError;

    #[test]
    fn no_arguments_serves_and_migrate_is_explicit() {
        assert_eq!(parse_args(["trading-api"]).expect("serve"), Command::Serve);
        assert_eq!(
            parse_args(["trading-api", "migrate"]).expect("migrate"),
            Command::Migrate
        );
    }

    #[test]
    fn unknown_arguments_are_rejected_without_secrets() {
        for args in [
            vec!["trading-api", "serve"],
            vec!["trading-api", "migrate", "--force"],
            vec![
                "trading-api",
                "postgres://trading:super-secret-password@127.0.0.1/trading",
            ],
        ] {
            let error = parse_args(args).expect_err("usage");
            assert!(matches!(error, AppError::Usage));
            let message = error.to_string();
            assert!(message.contains("usage: trading-api [migrate]"));
            assert!(!message.contains("super-secret-password"));
            assert!(!message.contains("postgres://"));
        }
    }
}
