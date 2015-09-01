
use super::common::{NearestNeighbor, Metric, CoverTreeData};
use treedisplay::TreeDisplay;
use std::fmt;
use std::mem;

pub struct CoverTreeNode<D> where D: CoverTreeData {
    /// The data stored in the node.
    data: D,
    /// The children of the node. Each child must be withing cover_distance of
    /// the node.
    children: Option<Vec<CoverTreeNode<D>>>,
    /// The level of the node.
    level: usize,
    /// The maximum distance from this node to any of its descendents.
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn seperation_distance(&self, span_factor: f64) -> f64 {
        span_factor.powf((self.level - 1) as f64)
    }

    fn descendents(&self) -> Vec<&CoverTreeNode<D>> {
        let mut descendents: Vec<&CoverTreeNode<D>> = Vec::new();
        if let Some(ref children) = self.children {
            for child in children {
                descendents.push(&child);
                for desc in child.descendents() {
                    descendents.push(desc);
                }
            }
        }
        descendents
    }

    fn max_distance(&self) -> f64 {
        let mut dist = 0f64;
        for descendent in self.descendents() {
            dist = self.data
                       .distance(descendent.data)
                       .max(dist);
        }
        dist
    }

    fn add_child(&mut self, node: CoverTreeNode<D>) {
        match self.children {
            None => self.children = Some(vec![node]),
            Some(ref mut children) => children.push(node)
        }
    }

    // Remove any leaf q from p
    // p′ ← tree with root q and p as only child
    // p ← p′
    fn promote_leaf(&mut self) {
        if self.children.is_none() {return;}

        if let Some(leaf) = self.remove_leaf() {
            let old_root = mem::replace(self, leaf);
            self.add_child(old_root);
        }
    }

    fn remove_leaf(&mut self) -> Option<CoverTreeNode<D>> {
        let mut leaf = None;
        let mut was_last = false;
        if let Some(ref mut children) = self.children {
            // Find index of leaf.
            if let Some(index) = children
                                   .iter()
                                   .position(|x| x.children.is_none()) {
                // Remove leaf and set was_last flag if needed.
                leaf = Some(children.swap_remove(index));
                if children.len() == 0 {was_last = true;}
            } else {
                // There are no leaves at this level, so recurse.
                leaf = children
                         .first_mut()
                         .expect("get first child")
                         .remove_leaf();
            }
        }
        if was_last {self.children = None;} // Erase empty vec.
        leaf
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
        let mut level = self.level;
        if self.data.distance(data) > self.cover_distance(span_factor) {
            while self.data.distance(data) > self.cover_distance(span_factor) * 2.0 {
                self.promote_leaf();
                level += 1;
                self.level = level;
            }
            let mut root = CoverTreeNode::new(data, level + 1);
            root.children = Some(vec![self]);
            return root;
        }

        self.insert_(data, span_factor)
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
    fn insert_(mut self,
               data: D,
               span_factor: f64) -> CoverTreeNode<D> {
        
        // Verify that the node can be inserted here.
        let dist = self.data.distance(data);
        let covdist = self.cover_distance(span_factor);
        assert!(dist <= covdist,
                "CoverTree invariant violated: d(p,x) ≤ covdist(p)"); 

        // Cache the maximum distance for this node.
        // self.max_distance = self.max_distance.max(dist);
        let mut done = false;
        if let Some(ref mut children) = self.children {
            for child in children {
                let dummy = CoverTreeNode::new(data, 0); // Placeholder data.
                if child.data.distance(data) <= child.cover_distance(span_factor) {

                    // Gain ownership over child and insert data.
                    let child_new = mem::replace(child, dummy)
                                      .insert_(data, span_factor);

                    // Restore child to where it was.
                    mem::replace(child, child_new);

                    // We want to return self, but we've borrowed children, 
                    // so we just set a flag and break instead.
                    done = true;
                    break;
                }
            }
        }
        if !done {
            // No children: just add the one we've got.
            if self.level == 1 {self.level += 1;}
            let new_node = CoverTreeNode::new(data, self.level-1);
            self.add_child(new_node);
        }
        self
    }

    // Pseudocode from paper:
    // function findNearestNeighbor(cover tree p, 
    //                              query point x, 
    //                              nearest neighbor so far y)
    //     if d(p, x) < d(y, x) then
    //         y←p
    //     for each child q of p sorted by distance to x do
    //         if d(y, x) > d(x, q) − maxdist(q) then
    //             y ← findNearestNeighbor(q, x, y)
    //     return y 
    fn find_nearest<'a>(&'a mut self, 
                        query: D,
                        nearest_yet: Option<&'a D>) 
                        -> &'a D {
        
        // Save closes value yet seen.
        let mut nearest = if nearest_yet.is_none() ||
                             self.data.distance(query) < nearest_yet.expect("data is nearest yet")
                                                                    .distance(query) { 
            &self.data 
        } else {
            nearest_yet.expect("provided is nearest yet")
        };

        if let Some(ref mut children) = self.children {
            // Sort children by distance to query point.
            children.sort_by(|a: &CoverTreeNode<D>, 
                              b: &CoverTreeNode<D>| a.data.distance(query)
                                                          .partial_cmp(&b.data.distance(query))
                                                          .expect("sort by distance to target"));
            
            for child in children {
                // If closer points could be below this one, recurse.
                if nearest.distance(query) > query.distance(child.data) - child.max_distance() {
                    nearest = child.find_nearest(query, Some(&nearest));
                }
            }
        }
        nearest
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
               self.max_distance(),
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