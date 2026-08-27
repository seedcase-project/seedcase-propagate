//! Library functionality for making the requested subset from the data package.

use std::sync::Arc;
use polars::prelude::LazyFrame;
use crate::core::Rap;
use std::error::Error;
// use itertools::Itertools;
// use crate::core::metadata::Resource;
// use crate::core::request::Subset;

/// Subsets the resources based on the details in the request.
///
/// This function does not do any checks on the input [`Request`] and
/// [`Package`] (within [`Rap`]), that should be done outside of this function.
/// Based on the request, this function will read the path to the data package's
/// resource (given in [`Resource`]), so some I/O errors may occur at this point.
///
/// # Arguments:
///
/// - `rap`: The [`Rap`] struct that contains the [`Request`] and [`Package`]
///    structs.
///
/// # Errors:
///
/// Outputs any input errors as well as column and row selection errors.
// TODO: Not sure if `Arc` or `Vec` is better here.
// TODO: Remove allow after implementing.
#[allow(unused_variables)]
pub fn subset_resources(rap: Rap) -> Result<Arc<LazyFrame>, Box<dyn Error>> {
    // TODO: Not sure if `Box .. Error` is the right approach here.

    // Using `path` in `Package`, read in all requested resources and add as `data` to `RequestedResources`.
    // let requested_resources: Arc<RequestedResources> = get_resource_data(&Rap)?;

    // Using the requested row-filtering, map on all data to keep rows.
    // let requested_resources_rows: Arc<RequestedResources> = requested_resources.iter().map(keep_requested_rows).collect()?;

    // Join each requested resource subset with the package resource metadata.
    // let kept_resource_ids: Arc<Arc<String>> = requested_resources_rows
    //     .iter()
    //     .map(get_ids)
    //     .flatten() // Not sure this works.
    //     .unique(); // from itertools.
    //     .collect()?

    // Add the Ids to the existing `requested_resources` and then subset each
    // resource by row and column.
    // let subsetted_data = requested_resources
    //     .iter()
    //     .map(|r| RequestedResource{ids: kept_resource_ids, ..r})
    //     .map(subset_resource)
    //     .collect()?;

    // Ok(subsetted_data)
    todo!("Planned")
}

// fn keep_requested_rows(resource: RequestedResource) -> Result<RequestedResource, Box<dyn Error>> {
//   // Need to convert the where conditions to SQL (or direct Polars) to apply on the data.
//   let filtered_data: LazyFrame = keep_rows(resource.data, convert_to_sql(resource.request.rows));
//   // Create a new `RequestedResource` with the filtered data.
//   RequestedResource {
//     request: resource.request,
//     data: filtered_data,
//     ids: None
//   }
// }

// Contains the subset item for one of the requested resources. `ids` is added later in the processing
// so it `Option`. Putting `ids` in this struct makes it easier to filter on the kept Ids in all resources.
// struct RequestedResource {
//   request: Subset,
//   data: LazyFrame,
//   ids: Option<Arc<String>> // TODO: This might not be a string, but maybe convert to one?
// }

// fn subset_resource(resource: RequestedResource) -> Result<SubsettedResource, Box<dyn Error>> {
//   let kept_rows = keep_rows(resource.data, resource.ids)?;
//   let subsetted_data = select_columns(kept_rows, resource.request.columns)?;
//   SubsettedResource {
//     data: subsetted_data,
//     resource_name: resource.request.name
// }

// This might be fine to not have a function but instead use Polars directly in the functions above.
// fn keep_rows(...) {
//   ...
// }

// This might be fine to not have a function but instead use Polars directly in the functions above.
// fn select_columns(...) {
//   ...
// }

// Need to output this struct in order to keep the resource name for later processing.
// struct SubsettedResource {
//   data: LazyFrame,
//   resource_name: String
// }
