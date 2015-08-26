
pub mod common;
pub mod simple;

#[test]
fn it_works() {
	use common::NearestNeighbor;

	fn dist(a: &i32, b: &i32) -> f64 {
		(b - a).abs() as f64
	}

	let mut ct = simple::CoverTree::new(dist as common::Metric<i32>);

	ct.insert(34i32);
	ct.insert(31i32);
}
