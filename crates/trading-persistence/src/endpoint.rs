use std::fmt;
use std::str::FromStr;

use sqlx::postgres::PgConnectOptions;
use url::Url;

use crate::error::{DatabaseUrlProblem, PersistenceError};

/// PostgreSQL target that is safe to write to logs.
///
/// The password and the query string are not retained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseEndpoint {
    username: String,
    host: String,
    port: u16,
    database: Option<String>,
}

impl DatabaseEndpoint {
    /// Database role named by the connection URL.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// TCP host named by the connection URL.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// TCP port named by the connection URL.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Database name, when the URL included one.
    pub fn database(&self) -> Option<&str> {
        self.database.as_deref()
    }
}

impl fmt::Display for DatabaseEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let host = if self.host.contains(':') {
            format!("[{}]", self.host)
        } else {
            self.host.clone()
        };
        write!(
            formatter,
            "postgres://{}@{}:{}",
            self.username, host, self.port
        )?;
        if let Some(database) = &self.database {
            write!(formatter, "/{database}")?;
        }
        Ok(())
    }
}

/// Parses a PostgreSQL URL into a log-safe endpoint description.
///
/// # Errors
///
/// Returns [`PersistenceError::InvalidUrl`] when the value cannot be used as a
/// TCP PostgreSQL connection URL. The error does not echo the input.
pub fn database_endpoint(database_url: &str) -> Result<DatabaseEndpoint, PersistenceError> {
    let parsed = Url::parse(database_url)
        .map_err(|_| PersistenceError::InvalidUrl(DatabaseUrlProblem::Malformed))?;
    if !matches!(parsed.scheme(), "postgres" | "postgresql") {
        return Err(PersistenceError::InvalidUrl(
            DatabaseUrlProblem::UnsupportedScheme,
        ));
    }
    match parsed.host_str() {
        Some(host) if !host.is_empty() => host,
        _ => {
            return Err(PersistenceError::InvalidUrl(
                DatabaseUrlProblem::MissingHost,
            ));
        }
    };
    let options = PgConnectOptions::from_str(database_url)
        .map_err(|_| PersistenceError::InvalidUrl(DatabaseUrlProblem::Rejected))?;
    let database = options
        .get_database()
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned);
    Ok(DatabaseEndpoint {
        username: options.get_username().to_owned(),
        host: options.get_host().to_owned(),
        port: options.get_port(),
        database,
    })
}

#[cfg(test)]
mod tests {
    use super::database_endpoint;

    const SECRET_URL: &str =
        "postgres://trading:super-secret-password@127.0.0.1:5432/trading?sslmode=disable";

    #[test]
    fn endpoint_omits_password_and_query() {
        let endpoint = database_endpoint(SECRET_URL).expect("url should parse");
        let rendered = format!("{endpoint} {endpoint:?}");

        assert_eq!(endpoint.username(), "trading");
        assert_eq!(endpoint.host(), "127.0.0.1");
        assert_eq!(endpoint.port(), 5432);
        assert_eq!(endpoint.database(), Some("trading"));
        assert_eq!(
            endpoint.to_string(),
            "postgres://trading@127.0.0.1:5432/trading"
        );
        assert!(!rendered.contains("super-secret-password"));
        assert!(!rendered.contains("sslmode"));
    }

    #[test]
    fn endpoint_reads_explicit_parts() {
        let endpoint =
            database_endpoint("postgresql://app@127.0.0.1:5433/market").expect("url should parse");
        assert_eq!(endpoint.username(), "app");
        assert_eq!(endpoint.port(), 5433);
        assert_eq!(endpoint.database(), Some("market"));
        assert_eq!(endpoint.to_string(), "postgres://app@127.0.0.1:5433/market");
    }

    #[test]
    fn invalid_urls_do_not_echo_the_password() {
        let cases = [
            "not-a-url-super-secret-password",
            "http://trading:super-secret-password@127.0.0.1/trading",
            "postgres://trading:super-secret-password@/trading",
        ];
        for input in cases {
            let error = database_endpoint(input).expect_err("url should be rejected");
            let message = error.to_string();
            assert!(
                !message.contains("super-secret-password"),
                "error echoed a password"
            );
        }
    }
}
