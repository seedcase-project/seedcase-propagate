//! Library functionality for interacting with the metadata of a data package.

/// Top-level representation of the metadata of a data package. Contains only
/// the fields from the Data Package spec that are relevant to Propagate.
pub struct Package {
    /// The data package version used to determine which
    /// version of the data package is being displayed when creating the
    /// request.
    pub version: Option<String>,
    pub contributors: Option<Vec<Contributor>>,
}

/// Represents the resource(s) in the data package. A resource is a single data
/// file or collection of related data files within a data package. Resources
/// can be different formats of data, such as Parquet, images, or audio files.
pub struct Resource {
    /// The resource name (no spaces) used as an identifier.
    pub name: String,
    /// The resource title (human formatted) used for display purposes.
    pub title: Option<String>,
    /// The path to the resource file(s) used when running the
    /// `subset` command.
    pub path: String,
    /// The schema for the contents of the resource containing the column
    /// information. Only relevant for tabular data.
    pub schema: Schema,
}

/// Contributor or author information for the data package. This is used when
/// displaying who to send the request to.
pub struct Contributor {
    /// The name of the contributor.
    pub name: String,
    /// The email address of the contributor.
    pub email: String,
    /// The role of the contributor in the data package. This is only used to
    /// display contributors who are contact persons (corresponding authors)
    /// like the owner or manager.
    pub role: Option<String>,
}

pub struct Schema {
    /// The resource columns. Only relevant for resources in tabular format. Called
    /// `fields` in the Data Package spec.
    pub columns: Vec<Column>,
    pub primary_key: Option<Vec<String>>,
    pub foreign_keys: Option<Vec<String>>,
}

/// A column within a resource. Called `field` in the Data Package spec.
pub struct Column {
    /// The column name (no spaces) used as an identifier.
    pub name: String,
    /// The column title (human formatted) used for display purposes.
    pub title: String,
    /// The column's data type.
    pub column_type: String,
    /// The column's constraints, e.g. minimum, maximum, or allowed values.
    pub constraints: Option<Constraints>,
    // TODO: Data Package has two fields for categories: `categories` and `constraints.enum`. Do we
    // need/want both?
}

/// The column constraints, i.e. the minimum and maximum values,
/// as well as allowed values.
pub struct Constraints {
    /// The minimum allowed value for a column. The type of the minimum value
    /// depends on the type of the column.
    pub minimum: Option<Minimum>,
    /// The maximum allowed value for a column. The type of the maximum value
    /// depends on the type of the column.
    pub maximum: Option<Maximum>,
    // TODO: This has `any` type in the spec, but should we allow that here?
    /// The allowed values for a column (e.g. for categorical data). It's called
    /// `enum` in the Data Package spec.
    pub allowed_values: Option<Vec<String>>,
}

/// The allowed minimum value for a column.
pub enum Minimum {
    /// The allowed minimum value for a column with values as integers (numbers
    /// without a decimal point).
    Integer(i64),
    /// The allowed minimum value for a column with values as numbers (numbers
    /// with a decimal point).
    Number(f64),
    // TODO: Set as date type with chrono package?
    /// The minimum value for a column of type `date`. The string should be in
    /// the format `YYYY-MM-DD`.
    Date(String),
    /// The minimum value for a column of type `datetime`. The string should be
    /// in the format `YYYY-MM-DDTHH:MM:SS`.
    Datetime(String),
}

/// The allowed maximum value for a column.
pub enum Maximum {
    /// The allowed maximum value for a column with values as integers (numbers
    /// without a decimal point).
    Integer(i64),
    /// The allowed maximum value for a column with values as numbers (numbers
    /// with a decimal point).
    Number(f64),
    /// The maximum value for a column of type `date`. The string should be in
    /// the format `YYYY-MM-DD`.
    Date(String),
    /// The maximum value for a column of type `datetime`. The string should be
    /// in the format `YYYY-MM-DDTHH:MM:SS`.
    Datetime(String),
}
