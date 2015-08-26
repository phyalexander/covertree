
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

	pub fn cover_distance(&self, tree: &CoverTree<D>) -> f64 {
		tree.span_factor.powf(self.level as f64)
	}

	pub fn seperation_distance(&self, tree: &CoverTree<D>) -> f64 {
		tree.span_factor.powf((self.level - 1) as f64)
	}

	pub fn insert(node: CoverTreeNode<D>, 
			      data: D, 
			      tree: &CoverTree<D>)
			      -> CoverTreeNode<D> {
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

	pub fn insert_(node: CoverTreeNode<D>, 
			       data: D,
			       tree: &CoverTree<D>) 
			       -> CoverTreeNode<D> {
		// Pseudocode from paper:
		// function insert_(cover tree p, data point x)
		//   prerequisites: d(p,x) ≤ covdist(p)
		//   for q ∈ children(p) do
		// 	    if d(q, x) ≤ covdist(q) then
		// 		  q′ ← insert_(q, x)
		// 		  p′ ← p with child q replaced with q′
		// 		  return p′
		//   return p with x added as a child 
		assert!((tree.metric)(&node.data, &data) <= node.cover_distance(tree),
				"CoverTree invariant violated: d(p,x) ≤ covdist(p)");

		// for child in node.children.iter().by_ref() {
		// 	if (self.metric)(&child.data, &data) <= child.cover_distance(self) {
				
		// 	}
		// }
		// let mut new_node = node.clone();
		// new_node.add_child(CoverTreeNode::new(data, node.level-1));
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

		let tree = CoverTree {root: None, 
				   			  metric: metric, 
				   			  span_factor: 1.3};
		tree.insert_all(items);
		tree
	}

	pub fn insert_all<T>(&self, items: T) where T: Iterator<Item=D> {
		for item in items {
			self.insert(item);
		}
	}
}


impl<D> NearestNeighbor<D> for CoverTree<D> where D: PartialEq {
	type Node = CoverTreeNode<D>;
	
	// fn find_nearest(data: &D) -> &D {
	// }

	fn insert(&self, data: D) {

	}

	fn distance(&self, a: &D, b: &D) -> f64 {
		(self.metric)(a, b)
	}
}