use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use crate::error::ReadinessProblem;

/// Metadata key written by migration `0001_infrastructure_metadata`.
pub const INFRASTRUCTURE_METADATA_KEY: &str = "schema_purpose";

/// Metadata value that identifies the infrastructure-only schema.
///
/// This value is necessary and not sufficient. Readiness also requires every
/// embedded SQLx migration to be recorded as successful with its checksum.
pub const INFRASTRUCTURE_METADATA_VALUE: &str = "infrastructure-metadata-only";

/// Client-side bound for one readiness check, including connection acquisition.
pub const READINESS_TIMEOUT: Duration = Duration::from_secs(2);

/// How long [`crate::Database::connect`] waits for a pooled connection.
pub const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(1);

/// PostgreSQL `statement_timeout` applied to API connections.
pub const READINESS_STATEMENT_TIMEOUT: &str = "1s";

/// Client-side bound for one migration command.
pub const MIGRATION_TIMEOUT: Duration = Duration::from_secs(60);

/// PostgreSQL `statement_timeout` applied to the connection that runs migrations.
///
/// That connection is closed afterward so the longer timeout is not reused by
/// a readiness check.
pub const MIGRATION_STATEMENT_TIMEOUT: &str = "30s";

/// One row of `_sqlx_migrations`, read for comparison only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecordedMigration {
    pub(crate) version: i64,
    pub(crate) checksum: Vec<u8>,
    pub(crate) success: bool,
}

/// One migration embedded in this build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectedMigration {
    pub(crate) version: i64,
    pub(crate) checksum: Vec<u8>,
}

/// Compares recorded SQLx history with the migrations embedded in this build.
///
/// An empty history is missing, not an invitation to migrate. A checksum
/// mismatch, a failed row, or a version this build does not contain is
/// incompatible. This function does not write.
pub(crate) fn assess_migrations(
    applied: &[RecordedMigration],
    expected: &[ExpectedMigration],
) -> Result<(), ReadinessProblem> {
    if expected.is_empty() {
        return Err(ReadinessProblem::SchemaIncompatible);
    }
    if applied.is_empty() {
        return Err(ReadinessProblem::SchemaMissing);
    }
    if applied.iter().any(|row| !row.success) {
        return Err(ReadinessProblem::SchemaIncompatible);
    }

    let mut expected_by_version = BTreeMap::<i64, &[u8]>::new();
    for migration in expected {
        if expected_by_version
            .insert(migration.version, migration.checksum.as_slice())
            .is_some()
        {
            return Err(ReadinessProblem::SchemaIncompatible);
        }
    }

    let mut seen = BTreeSet::new();
    for row in applied {
        let Some(checksum) = expected_by_version.get(&row.version) else {
            return Err(ReadinessProblem::SchemaIncompatible);
        };
        if *checksum != row.checksum.as_slice() {
            return Err(ReadinessProblem::SchemaIncompatible);
        }
        if !seen.insert(row.version) {
            return Err(ReadinessProblem::SchemaIncompatible);
        }
    }

    if seen.len() != expected_by_version.len() {
        Err(ReadinessProblem::SchemaMissing)
    } else {
        Ok(())
    }
}

/// Checks the infrastructure metadata row after migration history has matched.
///
/// A missing or different value is incompatible. Readiness does not insert it.
pub(crate) fn assess_metadata(value: Option<&str>) -> Result<(), ReadinessProblem> {
    match value {
        Some(INFRASTRUCTURE_METADATA_VALUE) => Ok(()),
        _ => Err(ReadinessProblem::SchemaIncompatible),
    }
}

/// Maps a PostgreSQL SQLSTATE to a readiness problem.
///
/// Unknown codes stay unreachable. They are not treated as a schema this
/// process may repair.
pub(crate) fn problem_for_sqlstate(code: Option<&str>) -> ReadinessProblem {
    match code {
        Some("57014") => ReadinessProblem::TimedOut,
        Some("42P01") => ReadinessProblem::SchemaMissing,
        Some("42703") => ReadinessProblem::SchemaIncompatible,
        Some("42501" | "28000" | "28P01") => ReadinessProblem::AccessDenied,
        _ => ReadinessProblem::Unreachable,
    }
}

#[cfg(test)]
mod tests {
    use super::INFRASTRUCTURE_METADATA_VALUE;
    use super::{
        ExpectedMigration, RecordedMigration, assess_metadata, assess_migrations,
        problem_for_sqlstate,
    };
    use crate::error::ReadinessProblem;

    fn recorded(version: i64, checksum: &[u8], success: bool) -> RecordedMigration {
        RecordedMigration {
            version,
            checksum: checksum.to_vec(),
            success,
        }
    }

    fn expected(version: i64, checksum: &[u8]) -> ExpectedMigration {
        ExpectedMigration {
            version,
            checksum: checksum.to_vec(),
        }
    }

    #[test]
    fn missing_history_is_not_ready() {
        let error = assess_migrations(&[], &[expected(1, b"abc")]).expect_err("missing");
        assert_eq!(error, ReadinessProblem::SchemaMissing);
    }

    #[test]
    fn matching_history_and_metadata_are_ready() {
        let applied = [recorded(1, b"abc", true)];
        let expected = [expected(1, b"abc")];
        assert_eq!(assess_migrations(&applied, &expected), Ok(()));
        assert_eq!(assess_metadata(Some(INFRASTRUCTURE_METADATA_VALUE)), Ok(()));
    }

    #[test]
    fn forged_metadata_without_a_matching_checksum_is_not_enough() {
        let applied = [recorded(1, b"forged", true)];
        let expected = [expected(1, b"abc")];
        let error = assess_migrations(&applied, &expected).expect_err("checksum");
        assert_eq!(error, ReadinessProblem::SchemaIncompatible);
        assert_eq!(assess_metadata(Some(INFRASTRUCTURE_METADATA_VALUE)), Ok(()));
    }

    #[test]
    fn matching_checksum_with_wrong_metadata_is_incompatible() {
        assert_eq!(
            assess_migrations(&[recorded(1, b"abc", true)], &[expected(1, b"abc")]),
            Ok(())
        );
        assert_eq!(
            assess_metadata(Some("forged-purpose")),
            Err(ReadinessProblem::SchemaIncompatible)
        );
        assert_eq!(
            assess_metadata(None),
            Err(ReadinessProblem::SchemaIncompatible)
        );
    }

    #[test]
    fn failed_unknown_and_partial_histories_are_not_ready() {
        let expected = [expected(1, b"one"), expected(2, b"two")];
        assert_eq!(
            assess_migrations(&[recorded(1, b"one", false)], &expected),
            Err(ReadinessProblem::SchemaIncompatible)
        );
        assert_eq!(
            assess_migrations(
                &[recorded(1, b"one", true), recorded(9, b"extra", true)],
                &expected
            ),
            Err(ReadinessProblem::SchemaIncompatible)
        );
        assert_eq!(
            assess_migrations(&[recorded(1, b"one", true)], &expected),
            Err(ReadinessProblem::SchemaMissing)
        );
    }

    #[test]
    fn sqlstates_are_classified_without_driver_text() {
        assert_eq!(
            problem_for_sqlstate(Some("42P01")),
            ReadinessProblem::SchemaMissing
        );
        assert_eq!(
            problem_for_sqlstate(Some("42703")),
            ReadinessProblem::SchemaIncompatible
        );
        assert_eq!(
            problem_for_sqlstate(Some("57014")),
            ReadinessProblem::TimedOut
        );
        assert_eq!(
            problem_for_sqlstate(Some("28P01")),
            ReadinessProblem::AccessDenied
        );
        assert_eq!(
            problem_for_sqlstate(Some("42501")),
            ReadinessProblem::AccessDenied
        );
        assert_eq!(
            problem_for_sqlstate(Some("08006")),
            ReadinessProblem::Unreachable
        );
        assert_eq!(problem_for_sqlstate(None), ReadinessProblem::Unreachable);
    }
}
