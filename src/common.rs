
use metric;

pub trait NearestNeighbor<D> where D: metric::Metric {
	type Node;
	
	fn find_nearest<'a>(&'a mut self, query: D) -> Option<&'a D>;
	fn insert(&mut self, data: D);
}


pub trait CoverTreeData: metric::Metric + PartialEq + PartialOrd + Copy {}

impl CoverTreeData for metric::MetricF64 {}