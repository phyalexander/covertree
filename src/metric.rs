

pub trait Metric<RHS=Self> {
	fn distance(self, rhs: RHS) -> f64;
}


macro_rules! make_metric {
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


make_metric!(MetricI32, i32);
make_metric!(MetricI64, i64);
make_metric!(MetricF32, f32);
make_metric!(MetricF64, f64);