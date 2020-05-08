// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO: Document API entities.
// #![warn(missing_docs)]

//! A library for communicating with Gitlab instances.

#[macro_use]
mod macros;
#[cfg(feature = "client_api")]
mod gitlab;

pub mod hooks;
pub mod systemhooks;
pub mod types;
pub mod webhooks;

#[cfg(feature = "client_api")]
pub use crate::gitlab::{
    CommitStatusInfo, Gitlab, GitlabBuilder, GitlabError, MergeRequestStateFilter, TokenError,
};
pub use crate::types::*;

#[cfg(test)]
mod test;
