
use super::common::{NearestNeighbor, Metric};

#[derive(Debug, PartialEq, Clone)]
pub struct CoverTreeNode<D> where D: PartialEq {
	data: D,
	children: Vec<CoverTreeNode<D>>,
	level: usize,
	max_distance: f64
}

impl<D> CoverTreeNode<D> where D: PartialEq {
	pub fn new(data: D, level: usize) -> CoverTreeNode<D> where D: PartialEq {
		CoverTreeNode {data:data, 
					   children: Vec::new(), 
					   level: level,
					   max_distance: 0.0}
	}

	pub fn add_child(&mut self, child: CoverTreeNode<D>) {
		if !self.children.contains(&child) {
			self.children.push(child)
		}
	}

	pub fn cover_distance(&self, span_factor: f64) -> f64 {
		span_factor.powf(self.level as f64)
	}

	pub fn seperation_distance(&self, span_factor: f64) -> f64 {
		span_factor.powf((self.level - 1) as f64)
	}

	pub fn insert<'a>(node: &'a mut CoverTreeNode<D>, 
			      data: D, 
			      metric: Metric<D>,
				  span_factor: f64)
			      -> &'a mut CoverTreeNode<D> {
		
		fn insert_<'a, T>(node: &'a mut CoverTreeNode<T>, 
				          data: T,
				          metric: Metric<T>,
						  span_factor: f64) 
				          -> &'a mut CoverTreeNode<T>
				          where T: PartialEq {
			// Pseudocode from paper:
			// function insert_(cover tree p, data point x)
			//   prerequisites: d(p,x) ≤ covdist(p)
			//   for q ∈ children(p) do
			// 	    if d(q, x) ≤ covdist(q) then
			// 		  q′ ← insert_(q, x)
			// 		  p′ ← p with child q replaced with q′
			// 		  return p′
			//   return p with x added as a child 
			assert!((metric)(&node.data, &data) <= node.cover_distance(span_factor),
					"CoverTree invariant violated: d(p,x) ≤ covdist(p)");
			node
		}

		// Pseudocode from paper:
		// function insert(cover tree p, data point x) 
		//   if d(p, x) > covdist(p) then
		// 	   while d(p, x) > 2*covdist(p) do
		//       Remove any leaf q from p
		// 		 p′ ← tree with root q and p as only child
		// 		 p ← p′
		// 	   return tree with x as root and p as only child
		//   return insert_(p, x)

		node
	}
}



pub struct CoverTree<D> where D: PartialEq {
	root: Option<CoverTreeNode<D>>,
	metric: Metric<D>,
	span_factor: f64,
}

impl<D> CoverTree<D> where D: PartialEq {
	pub fn new(metric: Metric<D>) -> CoverTree<D> where D: PartialEq {
		CoverTree {root: None, 
				   metric: metric, 
				   span_factor: 1.3}
	}

	pub fn from_items<T>(metric: Metric<D>, 
						 items: T)  
						 -> CoverTree<D>
						 where T: Iterator<Item=D> {

		let mut tree = CoverTree {root: None, 
				   			  metric: metric, 
				   			  span_factor: 1.3};
		tree.insert_all(items);
		tree
	}

	pub fn insert_all<T>(&mut self, items: T) where T: Iterator<Item=D> {
		for item in items {
			self.insert(item);
		}
	}
}


impl<D> NearestNeighbor<D> for CoverTree<D> where D: PartialEq {
	type Node = CoverTreeNode<D>;
	
	// fn find_nearest(data: &D) -> &D {
	// }

	fn insert(&mut self, data: D) {
		if let Some(ref mut node) = self.root {
			CoverTreeNode::insert(node, data, self.metric, self.span_factor);
			return;
		}
		self.root = Some(CoverTreeNode::new(data, 0));
	}

	fn distance(&self, a: &D, b: &D) -> f64 {
		(self.metric)(a, b)
	}
}