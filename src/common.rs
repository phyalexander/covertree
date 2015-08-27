
pub trait NearestNeighbor<D> where D: Metric {
	type Node;
	
	fn find_nearest<'a>(&'a self, query: D) -> Option<&'a D>;
	fn insert(&mut self, data: D);
}


pub trait CoverTreeData: Metric + PartialEq + PartialOrd + Copy {}

pub trait Metric<RHS=Self> {
	fn distance(self, rhs: RHS) -> f64;
}

// macro_rules! implement_primitive_metric {
// 	($imp:ident, $t:ty, $u:ty) => {
// 		impl common::Metric<$t> for $imp {
// 		    type Output = $u;
// 		    fn distance(self, rhs: $t) -> Self::Output {
// 		        (rhs - self).abs() as $u
// 		    }
// 		}

// 		impl<'a> common::Metric<$t> for &'a $imp {
// 		    type Output = $u;
// 		    fn distance(self, rhs: $t) -> Self::Output {
// 		        (rhs - self).abs() as $u
// 		    }
// 		}

// 		impl<'b> common::Metric<&'b $t> for $imp {
// 		    type Output = $u;
// 		    fn distance(self, rhs: &'b $t) -> Self::Output {
// 		        (rhs - self).abs() as $u
// 		    }
// 		}

// 		impl<'a, 'b> common::Metric<&'b $t> for &'a $imp {
// 		    type Output = $u;

// 		    fn distance(self, rhs:&'b $t) -> Self::Output {
// 		        (rhs - self).abs() as $u
// 		    }
// 		}
// 	}
// }