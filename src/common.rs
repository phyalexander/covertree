
use metric;

pub trait NearestNeighbor<D> where D: metric::Metric {
	type Node;
	
	fn find_nearest<'a>(&'a mut self, query: D) -> Option<&'a D>;
	fn insert(&mut self, data: D);
	fn remove(&mut self, data: D) -> Result<D, String>;
	fn count(&self) -> usize;
}


pub trait CoverTreeData: metric::Metric + PartialEq + PartialOrd + Copy {}


impl CoverTreeData for metric::MetricI32 {}
impl CoverTreeData for metric::MetricI64 {}
impl CoverTreeData for metric::MetricF32 {}
impl CoverTreeData for metric::MetricF64 {}
