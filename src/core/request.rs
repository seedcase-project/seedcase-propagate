//! Functions and types for making requests for subsets of the data package.

/// Details about what resources, columns, and rows are being requested as a
/// subset of the data package.
pub struct Subsets {
    /// Information on what rows to keep from the resources, as a
    /// [`RowSubset`].
    pub rows: Vec<RowsSubset>,
    /// Information on what columns to keep from the resources, as a
    /// [`ColumnsSubset`].
    pub columns: Vec<ColumnsSubset>,
}

/// Details about which columns will be selected from the resources.
pub struct ColumnsSubset {
    pub resource: String,
    /// The columns of the resource that are being requested as a subset.
    pub columns: Vec<String>,
}

/// Details about how the rows will be kept from the resources.
pub struct RowsSubset {
    /// The name of the resource that is being requested as a subset.
    pub resource: String,
    /// The logic conditions when **all** are true that determine which rows are
    /// kept, as a [`RowWhere`] struct.
    pub where_all: Option<Vec<RowWhere>>,
    /// The logic conditions when **any of them** true that determine which rows
    /// are kept, as a [`RowWhere`] struct.
    pub where_any: Option<Vec<RowWhere>>,
}

/// The individual row conditions that will be applied to an individual
/// resource.
pub struct RowWhere {
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
