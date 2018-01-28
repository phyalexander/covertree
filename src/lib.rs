// Copyright 2018 Skylor R. Schermer.
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
mod nearest;
mod simple;
#[cfg(test)]
mod tests;


// Reexports.
pub use metric::{
    Metric,
    MetricI8,
    MetricI16,
    MetricI32,
    MetricI64,
    MetricF32,
    MetricF64,
};
pub use simple::CoverTree;


////////////////////////////////////////////////////////////////////////////////
// Default covertree span factor
////////////////////////////////////////////////////////////////////////////////
pub const DEFAULT_SPAN_FACTOR: f64 = 1.3;

////////////////////////////////////////////////////////////////////////////////
// Point
////////////////////////////////////////////////////////////////////////////////
/// Trait representing requirements for insertion into a `CoverTree`.
pub trait Point: Metric + PartialEq + PartialOrd + Copy {}

// Blanket impl.
impl<T> Point for T where T: Metric+ PartialEq + PartialOrd + Copy {}