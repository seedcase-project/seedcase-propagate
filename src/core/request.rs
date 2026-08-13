//! Functions and types for making requests for subsets of the data package.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use semver::Version;

/// A type that maps the contents of `request.yaml`.
pub struct Request {
    /// The initial datetime when the request was created.
    pub datetime_created: DateTime<Utc>,
    /// The datetime when the request was last modified, mainly used when
    /// requesting an update to a previously submitted request.
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
    pub data_package: DataPackage,
    /// The specific rows from the resources requested, as [`Rows`].
    pub rows: Rows,
    /// The specific columns from the resources requested, as [`Columns`].
    pub columns: Columns,
}

/// The details about the research project that the request is for. This is used
/// to provide context to the requested data, so that the Owner can adequately
/// assess the request, the motivation, and whether the request is appropriate
/// for the given project.
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
pub struct Requester {
    /// The name of the Requester, for when the Owner needs to reach out to the
    /// Requester.
    pub name: String,
    /// The email of the Requester, for when contact is needed.
    pub email: String,
}

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

/// Details about what rows in the resources are being requested as a subset of
/// the data package.
pub struct Rows {
    /// Row-level subsets of resources (as a vector of hashmaps), with the
    /// resource name as the first item and the where conditions (as a
    /// vector) as the second.
    pub subsets: Vec<HashMap<String, Vec<Where>>>,
}

/// Details about how the rows will be kept from the resources.
pub enum Where {
    /// The logic conditions when **all** are true that determine which rows are
    /// kept, as a [`WhereCondition`] struct.
    All(Vec<WhereCondition>),
    /// The logic conditions when **any of them** true that determine which rows
    /// are kept, as a [`WhereCondition`] struct.
    Any(Vec<WhereCondition>),
}

/// The individual row conditions that will be applied to an individual
/// resource.
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

/// Details about what columns in resources are being requested as a subset of
/// the data package.
pub struct Columns {
    /// Column-level subsets of resources (as a vector of hashmaps) that
    /// contains the specific columns to include (as a vector of strings).
    pub subsets: Vec<HashMap<String, Vec<String>>>,
}
