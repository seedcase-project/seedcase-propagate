//! Functions and types for making requests for subsets of the data package.

use std::collections::HashMap;

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
    /// kept, as a [`WhereSubset`] struct.
    All(WhereSubset),
    /// The logic conditions when **any of them** true that determine which rows
    /// are kept, as a [`WhereSubset`] struct.
    Any(WhereSubset),
}

/// The individual row conditions that will be applied to an individual
/// resource.
pub struct WhereSubset {
    /// The column to apply the logic condition to.
    pub column: String,
    // TODO: This should probably be converted into an enum of allowed operators.
    /// The logic operator to apply between the `column` and `value`.
    pub operator: String,
    /// The value to compare against the row value.
    pub value: String,
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
