
use super::common::{NearestNeighbor, Metric, CoverTreeData};
use std::cell::RefCell;
use num::traits::Zero;

pub struct CoverTreeNode<D> where D: CoverTreeData {
    data: D,
    children: RefCell<Vec<CoverTreeNode<D>>>,
    level: usize,
    max_distance: f64
}

impl<D> CoverTreeNode<D> where D: CoverTreeData {
    pub fn new(data: D, level: usize) -> CoverTreeNode<D> where D: PartialEq {
        CoverTreeNode {data:data, 
                       children: RefCell::new(Vec::new()), 
                       level: level,
                       max_distance: 0.0}
    }

    fn cover_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf(self.level as f64) as f64
    }

    fn seperation_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf((self.level - 1) as f64) as f64
    }

    fn find_nearest<'a>(&'a self, 
                        query: D,
                        nearest_yet: &'a D) 
                        -> &'a D {
        // Pseudocode from paper:
        // function findNearestNeighbor(cover tree p, 
        //                              query point x, 
        //                              nearest neighbor so far y)
        // 
        //  if d(p, x) < d(y, x) then
        //      y←p
        //  for each child q of p sorted by distance to x do
        //      if d(y, x) > d(y, q) − maxdist(q) then
        //          y ← findNearestNeighbor(q, x, y)
        //      return y 

        let mut nearest = if &self.data.distance(query) < &nearest_yet.distance(query) { 
            &self.data 
        } else {
            nearest_yet
        };

        for child in self.children.borrow().iter() {
            if nearest.distance(query) > nearest.distance(self.data) - self.max_distance {
            }
        }


        &nearest
    }
}
//     fn insert<'a>(&'a mut self, 
//                   data: D, 
//                   metric: Metric<D>,
//                   span_factor: f64)
//                   -> &'a mut CoverTreeNode<D> {
        
//         fn insert_<'a, T>(node: &'a mut CoverTreeNode<T>, 
//                           data: T,
//                           metric: Metric<T>,
//                           span_factor: f64) 
//                           -> &'a mut CoverTreeNode<T>
//                           where T: PartialEq {
//             // Pseudocode from paper:
//             // function insert_(cover tree p, data point x)
//             //   prerequisites: d(p,x) ≤ covdist(p)
//             //   for q ∈ children(p) do
//             //      if d(q, x) ≤ covdist(q) then
//             //        q′ ← insert_(q, x)
//             //        p′ ← p with child q replaced with q′
//             //        return p′
//             //   return p with x added as a child 
//             assert!((metric)(&node.data, &data) <= node.cover_distance(span_factor),
//                     "CoverTree invariant violated: d(p,x) ≤ covdist(p)");
            
//             // for child in node.children.borrow_mut() {
//             //     if (metric)(&child.data, &data) <= child.cover_distance(span_factor) {
//             //         let child_prime = insert_(child, data, metric, span_factor);
//             //     }
//             // }
//             node
//         }

//         // Pseudocode from paper:
//         // function insert(cover tree p, data point x) 
//         //   if d(p, x) > covdist(p) then
//         //     while d(p, x) > 2*covdist(p) do
//         //       Remove any leaf q from p
//         //       p′ ← tree with root q and p as only child
//         //       p ← p′
//         //     return tree with x as root and p as only child
//         //   return insert_(p, x)

//         self
//     }
// }



// pub struct CoverTree<D> where D: PartialEq {
//     root: Option<CoverTreeNode<D>>,
//     metric: Metric<D>,
//     span_factor: f64,
// }

// impl<D> CoverTree<D> where D: PartialEq {
//     pub fn new(metric: Metric<D>) -> CoverTree<D> where D: PartialEq {
//         CoverTree {root: None, 
//                    metric: metric, 
//                    span_factor: 1.3}
//     }

//     pub fn from_items<T>(metric: Metric<D>, 
//                          items: T)  
//                          -> CoverTree<D>
//                          where T: Iterator<Item=D> {

//         let mut tree = CoverTree {root: None, 
//                                   metric: metric, 
//                                   span_factor: 1.3};
//         tree.insert_all(items);
//         tree
//     }

//     pub fn insert_all<T>(&mut self, items: T) where T: Iterator<Item=D> {
//         for item in items {
//             self.insert(item);
//         }
//     }
// }


// impl<D> NearestNeighbor<D> for CoverTree<D> where D: PartialEq {
//     type Node = CoverTreeNode<D>;
    
//     fn find_nearest<'a>(&'a self, query: &'a D) -> Option<&'a D> {
//         if let Some(ref node) = self.root {
//             Some(node.find_nearest(query, &node.data, self.metric))
//         } else {
//             None
//         }
//     }

//     fn insert(&mut self, data: D) {
//         if let Some(ref mut node) = self.root {
//             node.insert(data, self.metric, self.span_factor);
//             return;
//         }
//         self.root = Some(CoverTreeNode::new(data, 0));
//     }

//     fn distance(&self, a: &D, b: &D) -> f64 {
//         (self.metric)(a, b)
//     }
// }