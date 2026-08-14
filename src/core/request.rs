//! Functions and types for making requests for subsets of the data package.

use chrono::{DateTime, Utc};
use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;

/// A type that maps the contents of `request.yaml`.
#[derive(Debug, Deserialize)]
pub struct Request {
    /// The initial datetime when the request was created.
    #[serde(rename = "datetime-created")]
    pub datetime_created: DateTime<Utc>,

    /// The request's datetime when it was last modified, mainly used when
    /// requesting an update to a previously submitted request.
    #[serde(rename = "datetime-modified")]
    pub datetime_modified: DateTime<Utc>,

    /// The explanation for why the specific data is needed for the given
    /// [`Project`].
    pub motivation: String,

    /// Details about the [`Requester`] who is making the request for the data.
    pub requester: Requester,

    /// Details about the research [`Project`] that will use the requested data.
    pub project: Project,

    /// Details about the data package ([`DataPackage`]) that the request is
    /// for. Mostly used to provide a reference to the original data
    /// package, as well as to provide a way for the Owner to track which
    /// version of the data package the request is for.
    #[serde(rename = "data-package")]
    pub data_package: DataPackage,

    /// Row-level subsets of resources (as a hashmaps), with the
    /// resource name as the first item and the where conditions (as a
    /// vector) as the second.
    pub rows: HashMap<String, Vec<Where>>,

    /// Column-level subsets of resources (as a hashmaps) that
    /// contains the specific columns to include (as a vector of strings).
    pub columns: HashMap<String, Vec<String>>,
}

/// The details about the research project that the request is for. This is used
/// to provide context to the requested data, so that the Owner can adequately
/// assess the request, the motivation, and whether the request is appropriate
/// for the given project.
#[derive(Debug, Deserialize)]
pub struct Project {
    /// The name of the project in identifier format (lowercase, `-` for
    /// spaces).
    pub name: String,

    /// The title of the project in human-readable format.
    pub title: String,

    /// A description of the project. Should contain enough information for the
    /// Owner to assess the request. Ideally should have the research
    /// question(s) and general analysis methodology included.
    pub description: String,
}

/// The details about the person making the request for the data. This is used
/// to both to verify where/who the request is coming from and to provide
/// contact information for the Owner to reach out to the Requester if there are
/// any questions or concerns about the request.
#[derive(Debug, Deserialize)]
pub struct Requester {
    /// The name of the Requester, for when the Owner needs to reach out to the
    /// Requester.
    pub name: String,

    /// The email of the Requester, for when contact is needed.
    pub email: String,
}

/// Details about the data package that the request is asking for a data subset
/// from.
#[derive(Debug, Deserialize)]
pub struct DataPackage {
    /// The name of the data package, which is mainly used as a (human-readable)
    /// identifier.
    pub name: String,

    /// The version of the data package, which is used to determine which
    /// version of the data package is being displayed when creating the
    /// request. Can be used to determine which version the request was
    /// created for.
    pub version: Version,
}

/// Details about how the rows will be kept from the resources.
#[derive(Debug, Deserialize)]
pub enum Where {
    /// The logic conditions when **all** are true that determine which rows are
    /// kept, as a [`WhereCondition`] struct.
    #[serde(rename = "all")]
    All(Vec<WhereCondition>),

    /// The logic conditions when **any of them** true that determine which rows
    /// are kept, as a [`WhereCondition`] struct.
    #[serde(rename = "any")]
    Any(Vec<WhereCondition>),
}

/// The individual row conditions that will be applied to an individual
/// resource.
#[derive(Debug, Deserialize)]
pub struct WhereCondition {
    /// The column to apply the logic condition to.
    pub column: String,

    // TODO: This should probably be converted into an enum of allowed operators.
    /// The logic operator to apply between the `column` and `value`.
    pub operator: String,

    /// The value to compare against the row value.
    pub value: Option<String>,

    /// Whether to do the inverse of the logical condition.
    pub not: Option<bool>,
}

#[cfg(test)]
mod tests {
    // To import all code from above in this file.
    use super::*;
    use serde_saphyr;

    #[test]
    fn test_deserialising_correctly() {
        // `r#` means "raw string literal", to allow using `"` without escaping.
        let test_request_yaml = r#"
datetime-modified: "2026-07-06T01:45:34Z"
datetime-created: "2026-07-04T01:44:34Z"
motivation: |
  We would like access to metabolic and block variables to evaluate
  our hypothesis regarding ...

requester:
  name: "First Last"
  email: "TEXT"

project:
  name: "metabolic-cost"
  title: "Metabolic cost estimation"
  description: |
    Our project investigates the gas exchange during metabolism
    with the aim to determine ...

data-package:
  name: "example-seed-beetle"
  version: "0.5.1"

columns:
  metabolic-rate:
    - "strain"
    - "activity"
  biometrics:
  ids:

rows:
  metabolic-rate:
    - all:
      - column: "block"
        operator: "="
        value: "block1"
    - any:
      - column: "block1"
        operator: "="
        value: "10"
        not: true
    - any:
      - column: "block2"
        operator: "="
        value: "10"
        not: true
"#;

        let config: Result<Request, _> = serde_saphyr::from_str(test_request_yaml);
        assert!(config.is_ok())
    }

    #[test]
    fn test_serde_fail_not_all_present() {
        let test_request_yaml = r#"
datetime-modified: "2026-07-06T01:45:34Z"
datetime-created: "2026-07-04T01:44:34Z"
motivation: |
  We would like access to metabolic and block variables to evaluate
  our hypothesis regarding ...
"#;

        let config: Result<Request, _> = serde_saphyr::from_str(test_request_yaml);
        assert!(config.is_err())
    }
}
