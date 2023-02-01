#[cfg(feature = "capi")]
pub(crate) mod capi;

pub(crate) mod alloc;
pub(crate) mod util;

pub mod core {
    #![allow(dead_code)]
    #![allow(mutable_transmutes)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(unused_assignments)]
    #![allow(unused_mut)]
    #![allow(ambiguous_glob_reexports)]

    pub use c2rust_bitfields::*;

    #[allow(unused, clippy::all)]
    pub(crate) mod api_raw;
    #[allow(unused, clippy::all)]
    pub(crate) mod get_changed_ranges;
    #[allow(unused, clippy::all)]
    pub(crate) mod language;
    #[allow(unused, clippy::all)]
    pub(crate) mod lexer;
    #[allow(unused, clippy::all)]
    pub(crate) mod node;
    #[allow(unused, clippy::all)]
    pub(crate) mod parser;
    #[allow(unused, clippy::all)]
    pub(crate) mod query;
    #[allow(unused, clippy::all)]
    pub(crate) mod stack;
    #[allow(unused, clippy::all)]
    pub(crate) mod subtree;
    #[allow(unused, clippy::all)]
    pub(crate) mod tree;
    #[allow(unused, clippy::all)]
    pub(crate) mod tree_cursor;

    pub(crate) use crate::core_wrapper::alloc;
    pub(crate) use crate::core_wrapper::util;

    pub use alloc::*;
    pub use api_raw::*;
    pub use get_changed_ranges::*;
    pub use language::*;
    pub use lexer::*;
    pub use node::*;
    pub use parser::*;
    pub use query::*;
    pub use stack::*;
    pub use subtree::*;
    pub use tree::*;
    pub use tree_cursor::*;

    #[cfg(feature = "capi")]
    pub use crate::core_wrapper::capi::*;

    pub(crate) mod defines;
    pub use defines::*;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct TSLookaheadIterator {
        _unused: [u8; 0],
    }
}
