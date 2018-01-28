// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Temporary file for testing & developing covertree.
//!
////////////////////////////////////////////////////////////////////////////////

extern crate covertree;

// use covertree::simple::{CoverTree, CoverTreeNode};
// use covertree::common::{NearestNeighbor, CoverTreeData};
// use covertree::metric;

// use std::fmt;

use covertree::CoverTree;

#[cfg_attr(test, allow(dead_code))]
fn main() {

    let mut ct: CoverTree<f64> = CoverTree::new();

    let data: &[f64; 12] = &[
        1.0, 
        10.0, 
        100.0, 
        122.0, 
        123.0, 
        144.0, 
        20.0, 
        25.0, 
        35.0, 
        78.0, 
        89.0,
        90.0 
    ];
    for point in data.iter() {
        ct.insert(*point);
    }
    println!("{:#?}", ct);

    // // println!("{}\n", ct.tree_string());
    // ct.remove(91.0).ok();

    // println!("Nearest to 66: {}", ct.find_nearest(66.0).unwrap());
    // println!("Nearest to 93: {}", ct.find_nearest(93.0).unwrap());
    // println!("Nearest to 94: {}", ct.find_nearest(94.0).unwrap());
}





////////////////////////////////////////////////////////////////////////////////
// Debug display trait
////////////////////////////////////////////////////////////////////////////////
// trait TreeDisplay {
//     type Node;
//     fn node_string(&self) -> String;
//     fn children(&self) -> Option<&Vec<Self>>;
// }



// impl<D> TreeDisplay for CoverTreeNode<D> where D: fmt::Display + CoverTreeData {
//     type Node = CoverTreeNode<D>;
//     fn node_string(&self) -> String {format!("{}", self)}
//     fn children(&self) -> Option<&Vec<Self>> {self.children.as_ref()}
// }

// impl<D> fmt::Display for CoverTreeNode<D> where D: fmt::Display + CoverTreeData {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, 
//                "L{}[{}]: {}", 
//                self.level,
//                self.max_distance(),
//                self.data)
//     }
// }

// impl<D> CoverTree<D> where D: CoverTreeData + fmt::Display {
//     pub fn tree_string(&self) -> String { 
//         match self.root {
//             Some(ref root) => root.tree_string(),
//             None => "EMPTY".to_string()
//         }
//     }
// }