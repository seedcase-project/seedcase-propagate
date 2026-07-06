//! Library functionality for interacting with the metadata of a data package.

/// Top-level representation of the metadata of a data package. Contains only
/// the fields from the Data Package spec that are relevant to Propagate.
pub struct Metadata {
    /// The version of the data package, which can be used to determine which
    /// version of the data package is being displayed when creating the
    /// request.
    pub version: Option<String>,
    pub contributors: Option<Vec<Contributor>>,
}

/// Represents the resource(s) in the data package. A resource is a single data
/// file or collection of related data files within a data package. Resources
/// can be different formats of data, such as Parquet, images, or audio files.
pub struct Resource {
    /// The name of the resource (no spaces), which is used as an identifier.
    pub name: String,
    /// The title (human formatted) of the resource, which is used for display
    /// purposes.
    pub title: Option<String>,
    /// The path to the resource file(s), which is used when running the
    /// `subset` command.
    pub path: String,
    /// The schema for the contents of the resource, which contains the column
    /// information.
    pub schema: Option<Schema>,
}

/// Contributor or author information for the data package. This is used when
/// displaying who to send the request to.
pub struct Contributor {
    /// The name of the contributor.
    pub name: String,
    /// The email address of the contributor.
    pub email: Option<String>,
    /// The role of the contributor in the data package. This is only used to
    /// display contributors who are contact persons (corresponding authors)
    /// like the owner or manager.
    pub role: Option<String>,
}

pub struct Schema {
    /// The list of columns in the resource, when in tabular format. Called
    /// `fields` in the Data Package spec.
    pub columns: Vec<Column>,
    pub primary_key: Option<Vec<String>>,
    pub foreign_keys: Option<Vec<String>>,
}

/// A column within a resource. Called `field` in the Data Package spec.
pub struct Column {
    pub name: String,
    pub title: String,
    /// The type of data for the column.
    pub column_type: String,
    /// The specific constraints the column has, e.g. minimum, maximum, or
    /// allowed values.
    pub constraints: Option<Constraints>,
    // TODO: Data Package has two fields for categories: `categories` and `constraints.enum`. Do we
    // need/want both? pub categories: Option<Vec<String>>,
}

/// The constraints for a column, which can include minimum and maximum values,
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
    /// The allowed minimum value for a column with values as integers (those
    /// without a decimal point).
    Integer(i64),
    /// The allowed minimum value for a column with values as numbers (those
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
    /// The allowed maximum value for a column with values as integers (those
    /// without a decimal point).
    Integer(i64),
    /// The allowed maximum value for a column with values as numbers (those
    /// with a decimal point).
    Number(f64),
    /// The maximum value for a column of type `date`. The string should be in
    /// the format `YYYY-MM-DD`.
    Date(String),
    /// The maximum value for a column of type `datetime`. The string should be
    /// in the format `YYYY-MM-DDTHH:MM:SS`.
    Datetime(String),
}
