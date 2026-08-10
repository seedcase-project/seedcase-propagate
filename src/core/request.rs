//! Functions and types for making requests for subsets of the data package.

use chrono::{DateTime, Utc};
use semver::Version;

/// A type that maps the contents of `request.yaml`.
pub struct RequestMetadata {
    /// Details about the [`Request`] overall.
    pub request: Request,
    /// Details about the [`Requester`] who is making the request for the data.
    pub requester: Requester,
    /// Details about the research [`Project`] that will use the requested data.
    pub project: Project,
    /// Details about the data package ([`DataPackage`]) that the request is
    /// for. Mostly used to provide a reference to the original data
    /// package, as well as to provide a way for the Owner to track which
    /// version of the data package the request is for.
    pub data_package: DataPackage,
    /// The specific resources, columns, and rows to make as [`Subsets`] for the
    /// request.
    pub subsets: Subsets,
}

pub struct Request {
    /// The initial datetime when the request was created.
    pub datetime_created: DateTime<Utc>,
    /// The datetime when the request was last modified, mainly used when
    /// requesting an update to a previously submitted request.
    pub datetime_modified: DateTime<Utc>,
    /// The explanation for why the specific data is needed for the given
    /// [`Project`].
    pub motivation: String,
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

/// Details about what resources, columns, and rows are being requested as a
/// subset of the data package.
pub struct Subsets {
    /// TODO: Update after design PR has been merged.
    pub rows: String,
    pub columns: String,
}
