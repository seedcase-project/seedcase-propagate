//! Module for the library functionality of Propagate.

// `build` and `create-request` CLI commands contain some functionality from
// these modules, but (will likely) be specific to the CLI commands themselves.
// Therefore, they aren't included here.
pub mod check;
pub mod metadata;
pub mod request;
pub mod subset;

// TODO: Remove allow once implemented.
#[allow(unused)]
pub struct Rap {
    package: metadata::Package,
    request: request::Request,
}
