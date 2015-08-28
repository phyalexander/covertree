
use super::common::{NearestNeighbor, Metric, CoverTreeData};
use std::cmp::Ordering;
pub struct CoverTreeNode<D> where D: CoverTreeData {
    /// The data stored in the node.
    data: D,
    /// The children of the node. Each child must be withing cover_distance of
    /// the node.
    children: Option<Vec<CoverTreeNode<D>>>,
    /// The level of the node.
    level: usize,
    /// The maximum distance from this node to any of its descendents.
    max_distance: f64
}

impl<D> CoverTreeNode<D> where D: CoverTreeData {

    pub fn new(data: D, level: usize) -> CoverTreeNode<D> where D: PartialEq {
        CoverTreeNode {data:data, 
                       children: None, 
                       level: level,
                       max_distance: 0.0}
    }

    fn cover_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf(self.level as f64)
    }

    fn seperation_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf((self.level - 1) as f64)
    }

    // Pseudocode from paper:
    // function findNearestNeighbor(cover tree p, 
    //                              query point x, 
    //                              nearest neighbor so far y)
    //     if d(p, x) < d(y, x) then
    //         y←p
    //     for each child q of p sorted by distance to x do
    //         if d(y, x) > d(y, q) − maxdist(q) then
    //             y ← findNearestNeighbor(q, x, y)
    //         return y 
    pub fn find_nearest<'a>(&'a mut self, 
                            query: D,
                            nearest_yet: &'a D) 
                            -> &'a D {

        let mut nearest = if &self.data.distance(query) < &nearest_yet.distance(query) { 
            &self.data 
        } else {
            nearest_yet
        };

        if let Some(ref mut children) = self.children {
            children.sort_by(|a: &CoverTreeNode<D>, 
                              b: &CoverTreeNode<D>| a.data.distance(query)
                                                          .partial_cmp(&b.data.distance(query))
                                                          .unwrap_or(Ordering::Equal));
            for child in children {
                if nearest.distance(query) > nearest.distance(child.data) - child.max_distance {
                    nearest = child.find_nearest(query, &nearest);
                }
            }
        }
        &nearest
    }

    // Pseudocode from paper:
    // function insert(cover tree p, data point x) 
    //     if d(p, x) > covdist(p) then
    //         while d(p, x) > 2*covdist(p) do
    //             Remove any leaf q from p
    //             p′ ← tree with root q and p as only child
    //             p ← p′
    //         return tree with x as root and p as only child
    //     return insert_(p, x)
    pub fn insert(&mut self,
                  data: D,
                  span_factor: f64) {


        if self.data.distance(data) > self.cover_distance(span_factor) {
            while self.data.distance(data) > self.cover_distance(span_factor) * 2f64 {

                // todo
            }
        }

    }

    // Pseudocode from paper:
    // function insert_(cover tree p, data point x)
    //     prerequisites: d(p,x) ≤ covdist(p)
    //     for q ∈ children(p) do
    //          if d(q, x) ≤ covdist(q) then
    //              q′ ← insert_(q, x)
    //              p′ ← p with child q replaced with q′
    //              return p′
    //     return p with x added as a child 
    fn insert_(&mut self,
               data: D,
               span_factor: f64) {

        assert!(&self.data.distance(data) <= &self.cover_distance(span_factor),
                "CoverTree invariant violated: d(p,x) ≤ covdist(p)");  
        
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                if child.data.distance(data) <= child.cover_distance(span_factor) {
                    child.insert_(data, span_factor);
                    return;
                }
            }
            children.push(CoverTreeNode::new(data, self.level - 1));
        } else {
            self.children = Some(vec![CoverTreeNode::new(data, self.level - 1)]);
        }
    }
}



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