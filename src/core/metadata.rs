//! Library functionality for interacting with the metadata of a data package.

use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;

/// Top-level representation of the metadata of a data package. Contains only
/// the fields from the Data Package spec that are relevant to Propagate.
#[derive(Debug, Deserialize)]
pub struct Package {
    /// The data package version used to determine which
    /// version of the data package is being displayed when creating the
    /// request.
    pub version: Option<String>,
    pub contributors: Option<Vec<Contributor>>,
    pub resources: Vec<Resource>,
}

/// Represents the resource(s) in the data package. A resource is a single data
/// file or collection of related data files within a data package. Resources
/// can be different formats of data, such as Parquet, images, or audio files.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct Contributor {
    /// The name of the contributor.
    pub title: String,
    /// The email address of the contributor.
    pub email: String,
    /// The role of the contributor in the data package. This is only used to
    /// display contributors who are contact persons (corresponding authors)
    /// like the owner or manager.
    pub roles: Option<Vec<String>>,
}

/// The schema for the resource containing the column information. Only relevant
/// for tabular data.
#[derive(Debug, Deserialize)]
pub struct Schema {
    /// The resource columns. Only relevant for resources in tabular format.
    /// Called `fields` in the Data Package spec.
    #[serde(rename = "fields")]
    pub columns: Vec<Column>,
    /// The primary key for the resource.
    pub primary_key: Option<Vec<String>>,
    /// The foreign key relationships between this resource and other resources
    /// in the data package. This is used to determine how to effectively filter
    /// by rows. A subset of a data package should only contain rows in all
    /// resources of the relevant "observational units" (keys that show up
    /// in all resources). For example, if one resource keeps rows for only
    /// women and another resource keeps rows for only those with diabetes
    /// status, all requested resources should only contain rows with the
    /// intersection of these two conditions.
    pub foreign_keys: Option<Vec<ForeignKey>>,
}

/// A column within a resource. Called `field` in the Data Package spec.
#[derive(Debug, Deserialize)]
pub struct Column {
    /// The column name (no spaces) used as an identifier.
    pub name: String,
    /// The column title (human formatted) used for display purposes.
    pub title: Option<String>,
    /// The column data type.
    #[serde(rename = "type")]
    pub column_type: ColumnType,
    /// The column value constraints, e.g. minimum, maximum, or allowed values.
    pub constraints: Option<Constraints>,
    // TODO: Data Package has two fields for categories: `categories` and `constraints.enum`. Do we
    // need/want both?
}

// TODO: I'm not sure if these need explicit Rust types... We can see in
// practice.
/// The supported column data types from the metadata file. Also matches
/// what's allowed in Parquet files (our default format).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnType {
    String,
    Integer,
    Number,
    Boolean,
    Date,
    Datetime,
    Time,
    Array,
}

/// Represents a foreign key relationship between two resources in the data
/// package.
#[derive(Debug, Deserialize)]
pub struct ForeignKey {
    /// The column(s) in the current resource that form the foreign key.
    pub columns: Vec<String>,
    /// The resource that the foreign key references.
    pub reference_resource: String,
    /// The column(s) in the referenced resource that the foreign key
    /// references.
    pub reference_columns: Vec<String>,
}

/// The column constraints, i.e. the minimum and maximum values, as well as
/// allowed values.
#[derive(Debug, Deserialize)]
pub struct Constraints {
    /// The minimum allowed value for a column. The type of the minimum value
    /// depends on the type of the column.
    pub minimum: Option<Extreme>,
    /// The maximum allowed value for a column. The type of the maximum value
    /// depends on the type of the column.
    pub maximum: Option<Extreme>,
    // TODO: This has `any` type in the spec, but should we allow that here?
    /// The allowed values for a column (e.g. for categorical data). It's called
    /// `enum` in the Data Package spec.
    pub allowed_values: Option<Vec<String>>,
}

/// The allowed extreme value for a column (e.g. max or min).
#[derive(Debug, Deserialize)]
pub enum Extreme {
    /// The allowed value for a column with values as integers (numbers
    /// without a decimal point).
    Integer(i64),
    /// The allowed extreme value for a column with values as numbers (numbers
    /// with a decimal point).
    Number(f64),
    /// The allowed extreme value for a column of type `date`. The string should
    /// be in the format `YYYY-MM-DD`.
    Date(String),
    /// The allowed extreme value for a column of type `datetime`. The string
    /// should be in the format `YYYY-MM-DDTHH:MM:SS`.
    Datetime(String),
}

#[allow(clippy::needless_pass_by_value)]
pub enum PackageSource {
    // TODO: May need to use e.g. `Path` or `PathBuf`, depends on what the `open` functions need.
    Path(PathBuf),
    Https(String),
    GitHub(String),
}

//#[derive(Debug, thiserror::Error)]
//pub enum MetadataError {
//    #[error("failed to read metadata file: {0}")]
//    Io(#[from] std::io::Error),

//    #[error("invalid datapackage.json: {0}")]
//    Json(#[from] serde_json::Error),
//}

/// Reads and parses a data package's metadata file into a `Package` struct.
///
/// # Argument:
///
/// - `source`: This is the source location to the metadata file for the data
///   package, either a path, URL or `gh:` (GitHub) repo locations are
///   supported.
///
/// # Errors
///
/// Returns `Error` if the metadata file cannot be opened (e.g., it doesn't
/// exist) or if the metadata file is malformed (e.g., a `datapackage.json` file
/// that doesn't contain parsable JSON).
#[allow(unused_variables)]
pub fn read_package_metadata(source: &PackageSource) -> Result<Package, Box<dyn Error>> {
    // Box holds an unknown number of errors known only at runtime.
    // Open the metadata from the source locations
    // We'll have to either make custom errors or make use of error packages
    // like `thiserror`, `anyhow`, and/or `eyre`. Or just bundle the original
    // errors from e.g. the default reader and make reporting of them nicer with
    // `eyre`. Right now, we'll use `?` and see how that goes.
    // let package = match source {
    //   PackageSource::Path(file) => open_file(file)?,
    //   PackageSource::Https(url) => open_url(url)?,
    //   PackageSource::GitHub(gh) => open_url(gh_to_url(gh))?
    // };

    // Potentially include a match for if the file is JSON or other file format.
    // For now, only load in `datapackage.json` structured JSON.
    // Read the JSON contents of the file as an instance of `Package`.
    // let package: Package = read_from_json(package)?;
    // Ok(package)
    // todo!("Planned")
    match source {
        PackageSource::Path(path) => {
            let contents = std::fs::read_to_string(path)?;
            let package: Package = serde_json::from_str(&contents)?;
            Ok(package)
        }

        PackageSource::Https(_) => {
            todo!("HTTPS not yet supported")
        }

        PackageSource::GitHub(_) => {
            todo!("GitHub not yet supported")
        }
    }
}

/// An example of a datapackage.json following the Data Package standard.
/// Used for testing read and write functions.
// We add another # delimiter to raw because of the use of # in the JSON.
pub const EXAMPLE_DATAPACKAGE_JSON: &str = r##"
{
  "name": "diabetes-study",
  "id": "0c178bd2-5f27-4c9c-af73-5b06d82ef8ac",
  "title": "A Study on Diabetes",
  "description": "# Data from a 2021 study on diabetes prevalence\n\nThis data package contains data from a study conducted in 2021 on the\n*prevalence* of diabetes in various populations. The data includes:\n\n- demographic information\n- health metrics\n- survey responses about lifestyle\n",
  "version": "0.1.0",
  "created": "2026-08-24T17:57:46+00:00",
  "contributors": [
    {
      "title": "Jamie Jones",
      "path": "example.com/jamie_jones",
      "email": "jamie_jones@example.com",
      "roles": [
        "creator"
      ]
    }
  ],
  "licenses": [
    {
      "name": "ODC-BY-1.0",
      "path": "https://opendatacommons.org/licenses/by",
      "title": "Open Data Commons Attribution License 1.0"
    }
  ],
  "resources": [
    {
      "name": "patients",
      "path": "resources/patients/data.parquet",
      "type": "table",
      "title": "Patients Data",
      "description": "This data resource contains data about patients in a diabetes study.",
      "format": "parquet",
      "mediatype": "application/parquet",
      "schema": {
        "fields": [
          {
            "name": "id",
            "type": "integer"
          },
          {
            "name": "age",
            "type": "integer"
          },
          {
            "name": "sex",
            "type": "string"
          },
          {
            "name": "height",
            "type": "number"
          },
          {
            "name": "weight",
            "type": "number"
          },
          {
            "name": "diabetes_type",
            "type": "string"
          }
        ]
      }
    }
  ]
}
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_struct_deserialization() {
        let package: Package = serde_json::from_str(EXAMPLE_DATAPACKAGE_JSON).unwrap();

        assert_eq!(package.version.as_deref(), Some("0.1.0"));
        assert_eq!(package.resources.len(), 1);
        assert_eq!(package.resources[0].name, "patients");
    }

    #[test]
    fn test_read_package_metadata_using_path_input() {
        use std::io::Write;
        // see https://rust-exercises.com/advanced-testing/05_filesystem_isolation/02_tempfile.html
        // for my design choices around tempfile::NamedTempFile

        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(EXAMPLE_DATAPACKAGE_JSON.as_bytes()).unwrap();

        let source = PackageSource::Path(file.path().to_path_buf());

        let package = read_package_metadata(&source).unwrap();

        assert_eq!(package.version.as_deref(), Some("0.1.0"));
        assert_eq!(package.resources.len(), 1);
        assert_eq!(package.resources[0].name, "patients");
        assert_eq!(package.resources[0].title.as_deref(), Some("Patients Data"));
    }
}
