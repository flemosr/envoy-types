#![doc = include_str!("../README.md")]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![allow(missing_docs, rustdoc::invalid_html_tags, rustdoc::bare_urls)]

#[rustfmt::skip]
#[allow(clippy::all)]
mod generated;

/// Compiled protobuf types
pub mod pb {
    pub use crate::generated::*;
}

/// Convenience mod for `ext_authz` server implementation
pub mod ext_authz;
pub mod util;

mod sealed {
    pub trait Sealed {}
}
