// #![deny(unused_extern_crates)]
// #![warn(
//     box_pointers,
//     clippy::all,
//     clippy::nursery,
//     clippy::pedantic,
//     future_incompatible,
//     missing_copy_implementations,
//     // missing_docs,
//     nonstandard_style,
//     rust_2018_idioms,
//     trivial_casts,
//     trivial_numeric_casts,
//     unreachable_pub,
//     unused_qualifications
// )]

pub mod diagnostic;
pub mod parse_result;
pub mod parser;
pub mod queries;

pub use crate::parse_result::ParseResult;
