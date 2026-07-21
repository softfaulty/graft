//! stable error reports shared by the host, the recovery and the runtime binaries
//!
//! codes use `GRAFT-<AREA-<NUMBER>` and keep the same meaning once published
//! cause chains are ordered from the outer failure to the root cause

use std::{error::Error, fmt};

use serde::Serialize;

// TODO: please do not forget to update this whenever the schema changes >:(
pub const SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[must_use]
pub struct ErrorReport {
    schema_version: u16,
    code: String,
    summary: String,
    causes: Vec<String>,
    remediation: String,
}

impl ErrorReport {
    pub fn new(
        code: impl Into<String>,
        summary: impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            code: code.into(),
            summary: summary.into(),
            causes: Vec::new(),
            remediation: remediation.into(),
        }
    }

    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.causes.push(cause.into());
        self
    }

    pub fn with_error(mut self, error: &(dyn Error + 'static)) -> Self {
        let mut current = Some(error);

        while let Some(cause) = current {
            self.causes.push(cause.to_string());
            current = cause.source();
        }

        self
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn causes(&self) -> &[String] {
        &self.causes
    }

    pub fn remediation(&self) -> &str {
        &self.remediation
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl fmt::Display for ErrorReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code, self.summary)?;

        for (index, cause) in self.causes.iter().enumerate() {
            let label = if index == 0 { "cause" } else { "caused by" };
            write!(formatter, "\n{label}: {cause}")?;
        }

        write!(formatter, "\nremediation: {}", self.remediation)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn renders_human_report() {
        let report = ErrorReport::new(
            "GRAFT-TEST-0001",
            "image verification failed",
            "recreate the staged image",
        )
        .with_cause("filesystem check failed")
        .with_cause("invalid superblock");

        assert_eq!(
            report.to_string(),
            "[GRAFT-TEST-0001] image verification failed\n\
             cause: filesystem check failed\n\
             caused by: invalid superblock\n\
             remediation: recreate the staged image"
        );
    }

    #[test]
    fn serializes_versioned_json_report() {
        let report = ErrorReport::new(
            "GRAFT-TEST-0002",
            "state could not be loaded",
            "restore a verified state generation",
        )
        .with_cause("unexpected end of file");

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&report.to_json().unwrap()).unwrap(),
            json!({
                "schema_version": 1,
                "code": "GRAFT-TEST-0002",
                "summary": "state could not be loaded",
                "causes": ["unexpected end of file"],
                "remediation": "restore a verified state generation",
            })
        );
    }
}
