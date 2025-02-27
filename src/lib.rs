// Copyright 2018 Skylor R. Schermer.
// Copyright 2025 phyalex.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

////////////////////////////////////////////////////////////////////////////////
//!
//! Cover tree design based on this [paper]
//!
//! [paper]: https://izbicki.me/public/papers/icml2015-faster-cover-trees.pdf
////////////////////////////////////////////////////////////////////////////////

#![doc(html_root_url = "https://docs.rs/covertree/0.2.0")]


// Module declarations.
mod metric;
mod non_parallel;
#[cfg(test)]
mod tests;
mod cover;

// Reexports.
pub use metric::{
    MetricSpace,
    MetricI8,
    MetricI16,
    MetricI32,
    MetricI64,
    MetricF32,
    MetricF64,
};
pub use non_parallel::CoverTree;


////////////////////////////////////////////////////////////////////////////////
// Default covertree span factor
////////////////////////////////////////////////////////////////////////////////
pub const DEFAULT_SPAN_FACTOR: f64 = 1.3;

////////////////////////////////////////////////////////////////////////////////
// Point
////////////////////////////////////////////////////////////////////////////////
/// Trait representing requirements for insertion into a `CoverTree`.
pub trait Point: MetricSpace + PartialEq + PartialOrd + Copy {}

// Blanket impl.
impl<T> Point for T where T: MetricSpace + PartialEq + PartialOrd + Copy {}