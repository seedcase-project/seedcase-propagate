//! Functions and types for making requests for subsets of the data package.

/// A type that maps the contents of `request.yaml`.
pub struct Request {
    /// The initial date when the request was created.
    pub data_created: String,
    /// The date when the request was last modified, mainly used when requesting
    /// an update to a previously submitted request.
    pub data_modified: String,
    /// The number of times that a request has been updated, mainly to use for
    /// versioning of the request. It is optional because it requires manual
    /// incrementing that must be done by the Requester and checked by the
    /// Owner.
    pub iteration: Option<u8>,
    /// The explanation for why the specific data is needed for the given
    /// [`Project`].
    pub motivation: String,
    /// Details about the research [`Project`] that will use the requested data.
    pub project: Project,
    /// Details about the [`Requester`] who is making the request for the data.
    pub requester: Requester,
    /// Details about the data package ([`Package`]) that the request is for.
    /// Mostly used to provide a reference to the original data package, as
    /// well as to provide a way for the Owner to track which version of the
    /// data package the request is for.
    pub data_package: Package,
    /// The specific resources, columns, and rows to make as [`Subsets`] for the request.
    pub subsets: Subsets,
}

/// The details about the research project that the request is for. This is used
/// to provide context to the requested data, so that the Owner can adequately
/// assess the request, the motivation, and whether the request is appropriate
/// for the given project.
pub struct Project {
    /// The name of the project in identifier format (lowercase, `-` for spaces).
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

pub struct Package {
    /// The name of the data package, which is mainly used as a (human-readable)
    /// identifier.
    pub name: String,
    /// The version of the data package, which is used to determine which
    /// version of the data package is being displayed when creating the
    /// request. Can be used to determine which version the request was
    /// created for.
    pub version: String,
}

/// Details about what resources, columns, and rows are being requested as a
/// subset of the data package.
pub struct Subsets {
    /// Subset information for the specific resources being requested as a
    /// [`ResourceSubset`].
    pub resources: Vec<ResourceSubset>,
}

/// Information on which columns and rows that are being requested for a
/// specific resource in the data package.
pub struct ResourceSubset {
    /// The name of the resource that is being requested as a subset.
    pub name: String,
    /// The columns of the resource that are being requested as a subset.
    pub columns: Vec<String>,
    /// The rows of the resource that are being requested as a subset.
    pub rows: Vec<String>,
}
