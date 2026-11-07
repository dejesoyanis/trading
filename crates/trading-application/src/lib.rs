//! Application-level orchestration boundary.
//!
//! The composition root can construct an [`Application`] to record the
//! deployment environment. Order submission workflows are not implemented.

/// Order submission workflows are not implemented in this milestone.
pub const ORDER_SUBMISSION_IMPLEMENTED: bool = false;

/// Deployment environment of a process.
///
/// This is not a trading mode. Selecting [`DeploymentEnvironment::Production`]
/// does not enable paper trading or live trading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    /// A developer machine.
    Local,
    /// Automated tests or continuous integration.
    Test,
    /// A deployed process. Live trading remains unsupported.
    Production,
}

impl DeploymentEnvironment {
    /// Returns the stable configuration spelling of this environment.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Test => "test",
            Self::Production => "production",
        }
    }
}

/// Application context constructed at process startup.
///
/// The context does not submit orders or authorize trading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Application {
    environment: DeploymentEnvironment,
}

impl Application {
    /// Creates an application context for a deployment environment.
    pub const fn new(environment: DeploymentEnvironment) -> Self {
        Self { environment }
    }

    /// Returns the deployment environment captured at startup.
    pub const fn environment(self) -> DeploymentEnvironment {
        self.environment
    }
}

#[cfg(test)]
mod tests {
    use super::{Application, DeploymentEnvironment, ORDER_SUBMISSION_IMPLEMENTED};

    #[test]
    fn environment_round_trip_does_not_enable_submission() {
        let cases = [
            (DeploymentEnvironment::Local, "local"),
            (DeploymentEnvironment::Test, "test"),
            (DeploymentEnvironment::Production, "production"),
        ];
        for (environment, spelling) in cases {
            let application = Application::new(environment);
            assert_eq!(application.environment(), environment);
            assert_eq!(environment.as_str(), spelling);
        }
        const { assert!(!ORDER_SUBMISSION_IMPLEMENTED) };
    }
}
