//! Library functionality for checking the request itself and the metadata
//! against the request.

use crate::core::Rap;
use std::error::Error;

/// Checks a request yaml against the metadata file.
///
/// The checks include:
///
/// - Required sections and keys are present in the request and metadata.
/// - Data package name and version in the request matches the metadata.
/// - The resources and columns in the request exist in the metadata.
/// - The row filters match columns in the metadata.
///
/// # Arguments:
///
/// - `request`: The [`Request`] struct with the request details.
/// - `metadata`: The [`Package`] struct with the data package's metadata.
///
/// # Errors:
///
/// Errors if any check fails.
// TODO: Remove allow once implemented.
#[allow(unused_variables)]
pub fn check_request(rap: &Rap) -> Result<(), Box<dyn Error>> {
    // TODO: We may need to revise this so we can group all errors together and output them at the end.

    // Package name and version matches metadata.
    // check_package_name(&rap)?;
    // check_package_version(&rap)?;

    // Resource names match metadata.
    // check_resource_names(&rap)?;

    // Column names match metadata.
    // check_column_names(&rap)?;

    // Row filters match metadata columns (including whether the operator
    // types and values are allowed with the column type).
    // check_row_filters(&rap)?;

    // Ok(())

    todo!("Planned")
}
