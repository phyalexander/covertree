
pub trait NearestNeighbor<D> where D: Metric {
	type Node;
	
	fn find_nearest<'a>(&'a mut self, query: D) -> Option<&'a D>;
	fn insert(&mut self, data: D);
}


pub trait CoverTreeData: Metric + PartialEq + PartialOrd + Copy {}

pub trait Metric<RHS=Self> {
	fn distance(self, rhs: RHS) -> f64;
}