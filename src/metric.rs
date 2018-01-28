// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! 
//! Provides the `Metric` trait.
//! 
////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////
// Metric
////////////////////////////////////////////////////////////////////////////////
// TODO(Sky): Consider generic return value.
pub trait Metric<RHS=Self> {
	/// Calculates the distance between two points.
	fn distance(self, rhs: RHS) -> f64;
}


////////////////////////////////////////////////////////////////////////////////
// auto_impl_metric
////////////////////////////////////////////////////////////////////////////////
macro_rules! auto_impl_metric {
	($name:ident, $t:ty) => (
		pub type $name = $t;

		impl Metric<$t> for $name {
		    fn distance(self, rhs: $t) -> f64 {
		        (rhs - self).abs() as f64
		    }
		}

		impl<'a> Metric<$t> for &'a $name {
		    fn distance(self, rhs: $t) -> f64 {
		        (rhs - self).abs() as f64
		    }
		}

		impl<'b> Metric<&'b $t> for $name {
		    fn distance(self, rhs: &'b $t) -> f64 {
		        (rhs - self).abs() as f64
		    }
		}

		impl<'a, 'b> Metric<&'b $t> for &'a $name {
		    fn distance(self, rhs:&'b $t) -> f64 {
		        (rhs - self).abs() as f64
		    }
		}
	)
}


////////////////////////////////////////////////////////////////////////////////
// Default trait implementations.
////////////////////////////////////////////////////////////////////////////////
// Integer types.
auto_impl_metric!(MetricI8, i8);
auto_impl_metric!(MetricI16, i16);
auto_impl_metric!(MetricI32, i32);
auto_impl_metric!(MetricI64, i64);

// Float types.
auto_impl_metric!(MetricF32, f32);
auto_impl_metric!(MetricF64, f64);
