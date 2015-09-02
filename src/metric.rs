

pub trait Metric<RHS=Self> {
	fn distance(self, rhs: RHS) -> f64;
}


macro_rules! make_metric {
	($t:ty, $name:ident) => (
		pub type $name = $t;

		impl Metric<$t> for $name {
		    fn distance(self, rhs: $t) -> $t {
		        (rhs - self).abs() as $t
		    }
		}

		impl<'a> Metric<$t> for &'a $name {
		    fn distance(self, rhs: $t) -> $t {
		        (rhs - self).abs() as $t
		    }
		}

		impl<'b> Metric<&'b $t> for $name {
		    fn distance(self, rhs: &'b $t) -> $t {
		        (rhs - self).abs() as $t
		    }
		}

		impl<'a, 'b> Metric<&'b $t> for &'a $name {

		    fn distance(self, rhs:&'b $t) -> $t {
		        (rhs - self).abs() as $t
		    }
		}
	)
}


make_metric!(f64, MetricF64);