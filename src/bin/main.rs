// Copyright 2018 Skylor R. Schermer.
// Copyright 2025 phyalex.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



extern crate covertree;


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

    println!("Nearest to 66: {}", ct.find_nearest(66.0).unwrap());
    println!("Nearest to 93: {}", ct.find_nearest(93.0).unwrap());
    println!("Nearest to 94: {}", ct.find_nearest(94.0).unwrap());
}


