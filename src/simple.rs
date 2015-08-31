
use super::common::{NearestNeighbor, Metric, CoverTreeData};
use treedisplay::TreeDisplay;
use std::fmt;
use std::mem;
use std::cmp;

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

    fn new(data: D, level: usize) -> CoverTreeNode<D> where D: PartialEq {
        CoverTreeNode {data:data, 
                       children: None, 
                       level: level,
                       max_distance: 0.0}
    }

    fn cover_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf(self.level as f64)
    }

    // fn seperation_distance(&self, span_factor: f64) -> f64 {
    //     span_factor.powf((self.level - 1) as f64)
    // }

    // Pseudocode from paper:
    // function findNearestNeighbor(cover tree p, 
    //                              query point x, 
    //                              nearest neighbor so far y)
    //     if d(p, x) < d(y, x) then
    //         y←p
    //     for each child q of p sorted by distance to x do
    //         if d(y, x) > d(y, q) − maxdist(q) then
    //             y ← findNearestNeighbor(q, x, y)
    //     return y 
    fn find_nearest<'a>(&'a mut self, 
                        query: D,
                        nearest_yet: Option<&'a D>) 
                        -> &'a D {

        let mut nearest = if nearest_yet.is_none() ||
                             &self.data.distance(query) < &nearest_yet.expect("data is nearest yet")
                                                                      .distance(query) { 
            &self.data 
        } else {
            nearest_yet.expect("provided is nearest yet")
        };



        if let Some(ref mut children) = self.children {
            children.sort_by(|a: &CoverTreeNode<D>, 
                              b: &CoverTreeNode<D>| a.data.distance(query)
                                                          .partial_cmp(&b.data.distance(query))
                                                          .expect("sort by distance to target"));
            for child in children {
                if nearest.distance(query) > nearest.distance(child.data) - child.max_distance {
                    nearest = child.find_nearest(query, Some(&nearest));
                }
            }
        }
        &nearest
    }

    fn validate(&mut self) {
        self.max_distance = 0f64;
        self.level = 0;
        if let Some(ref mut children) = self.children {
            for child in children {
                child.validate();
                self.level = cmp::max(self.level, child.level+1);
                self.max_distance = self.max_distance.max(self.data.distance(child.data));
            }
        }
    }

    fn remove_leaf(&mut self) -> Option<CoverTreeNode<D>> {
        // let mut leaf: Option<CoverTreeNode<D>> = None;

        // if self.children.is_some() && self.max_distance == 0f64 {
        //     self.max_distance = 0f64;
        //     leaf = Some(mem::replace(&mut self.children, None).expect("get children")
        //                                                       .pop()
        //                                                       .expect("pull last child"));
        //     self.children = None;
        //     return leaf
        // }

        if let Some(ref mut children) = self.children {
            if let Some(index) = children.iter().position(|x| x.children.is_none()) {
                return Some(children.remove(index));
            }
            for child in children {
                let leaf = child.remove_leaf();
                if leaf.is_some() {return leaf;}
            }
        }
        None
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
    fn insert(mut self,
              data: D,
              span_factor: f64) ->CoverTreeNode<D> {

        if (&self.data).distance(data) > self.cover_distance(span_factor) {
            while (&self.data).distance(data) > self.cover_distance(span_factor) * 2f64 {
        
                if let Some(mut leaf) = self.remove_leaf() {
                    leaf.children = Some(vec![self]);
                    self = leaf;
                } else {
                    break;
                }
            }
            let mut root = CoverTreeNode::new(data, 0);
            root.children = Some(vec![self]);
            return root;
        }
        self.insert_(data, span_factor);
        return self;
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
               span_factor: f64) -> bool {
        let dist = self.data.distance(data);

        assert!(&dist <= &self.cover_distance(span_factor),
                "CoverTree invariant violated: d(p,x) ≤ covdist(p)");  
        
        if dist > self.max_distance { 
            self.max_distance = dist; 
        }
        let raise = if self.level == 0 { 
            self.level = 1; 
            true 
        } else { 
            false 
        };

        if let Some(ref mut children) = self.children {
            for child in &mut children.iter_mut() {
                if child.data.distance(data) <= child.cover_distance(span_factor) {
                    return if child.insert_(data, span_factor) {
                        self.level += 1;
                        true
                    } else {
                        false
                    };
                }
            }
            children.push(CoverTreeNode::new(data, self.level - 1));
        } else {
            self.children = Some(vec![CoverTreeNode::new(data, self.level - 1)]);
        }
        raise
    }
}


pub struct CoverTree<D> where D: CoverTreeData {
    root: Option<CoverTreeNode<D>>,
    span_factor: f64,
}

impl<D> CoverTree<D> where D: CoverTreeData {
    pub fn new() -> CoverTree<D> where D: PartialEq {
        CoverTree {root: None,
                   span_factor: 1.3}
    }

    pub fn from_items<T>(items: T)  
                         -> CoverTree<D>
                         where T: Iterator<Item=D> {

        let mut tree = CoverTree::new();
        tree.insert_all(items);
        tree
    }

    pub fn insert_all<T>(&mut self, items: T) where T: Iterator<Item=D> {
        for item in items {
            self.insert(item);
        }
    }
}

impl<D> NearestNeighbor<D> for CoverTree<D> where D: CoverTreeData {
    type Node = CoverTreeNode<D>;
    
    fn find_nearest<'a>(&'a mut self, query: D) -> Option<&'a D> {
        if let Some(ref mut node) = self.root {
            Some(node.find_nearest(query, None))
        } else {
            None
        }
    }

    fn insert(&mut self, data: D) {
        let temp = CoverTreeNode::new(data, 1);
        if let Some(ref mut node) = self.root {
            let n = mem::replace(node, temp);
            mem::replace(node, n.insert(data, self.span_factor));
            node.validate();
        } else {
            self.root = Some(temp);
        }
    }
}




impl<D> TreeDisplay for CoverTreeNode<D> where D: fmt::Display + CoverTreeData {
    type Node = CoverTreeNode<D>;
    fn node_string(&self) -> String {format!("{}", self)}
    fn children(&self) -> Option<&Vec<Self>> {self.children.as_ref()}
}

impl<D> fmt::Display for CoverTreeNode<D> where D: fmt::Display + CoverTreeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
               "L{}[{}]: {}", 
               self.level,
               self.max_distance,
               self.data)
    }
}

impl<D> CoverTree<D> where D: CoverTreeData + fmt::Display {
    pub fn tree_string(&self) -> String { 
        match self.root {
            Some(ref root) => root.tree_string(),
            None => "EMPTY".to_string()
        }
    }
}