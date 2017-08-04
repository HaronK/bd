//! Tool for investigating binary data format.

// #![deny(missing_docs,
//         missing_debug_implementations,
//         missing_copy_implementations,
//         trivial_casts,
//         trivial_numeric_casts,
// //        unsafe_code,
//         unstable_features,
//         unused_import_braces,
//         unused_qualifications)]

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

/// Adapter macros
//#[macro_use]
//extern crate adapter;

pub mod core;
pub mod types;
pub mod blocks;
