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

/// Represents the resource(s) in the data package. A resource is a single data file
/// or collection of related data files within a data package. Resources can be different
/// formats of data, such as Parquet, images, or audio files.
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

pub struct Contributor {
    pub name: String,
    pub email: Option<String>,
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
    /// The specific constraints the column has, e.g. minimum, maximum, or allowed values.
    pub constraints: Option<Constraints>,
    // TODO: Data Package has two fields for categories: `categories` and `constraints.enum`. Do we need/want both?
    // pub categories: Option<Vec<String>>,
}

pub struct Constraints {
    pub minimum: Option<Minimum>,
    pub maximum: Option<Maximum>,
    // TODO: This has `any` type in the spec, but should allow that here?
    /// The allowed values for a column (e.g. for categorical data). It's called `enum` in the Data Package spec.
    pub allowed_values: Option<Vec<String>>,
}

pub enum Minimum {
    Integer(i64),
    Float(f64),
    Character(String),
}

pub enum Maximum {
    Integer(i64),
    Float(f64),
    Character(String),
}
