
pub trait NearestNeighbor<D> where D: PartialEq {
	type Node;
	
	// fn find_nearest(data: &D) -> &D;
	fn insert(&mut self, data: D);
	fn distance(&self, a: &D, b: &D) -> f64;
}


pub type Metric<D> = fn(a: &D, b: &D) -> f64;


