// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
////////////////////////////////////////////////////////////////////////////////

use CoverTree;
use MetricI64;
use MetricF64;


fn test_f64_data() -> Vec<f64> {
    vec![
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
    ]
}

fn test_i64_data() -> Vec<i64> {
    vec![
        18, 28, 5, 7303, 7430, 910, 1200, 190, 12, 
        512, 1002, 188, 134, 70, 8309, 23, 1003, 39, 
        89, 257, 72, 7, 15, 103, 2, 3134, 315, 95, 
        112, 26, 212, 369, 731, 100, 90, 60, 1, 3462, 
        9, 456, 91, 13, 699, 165, 301, 856, 17, 709, 
        634, 800, 619, 2778, -12, 4555, -89, 0, -60
    ]
}

#[test]
fn empty_tree() {
    let ct: CoverTree<MetricF64> = CoverTree::new();
    assert_eq!(ct.len(), 0);
}

#[test]
fn from_items() {
    let ct: CoverTree<MetricF64> = CoverTree::from_items(test_f64_data().into_iter());
    assert_eq!(ct.len(), test_f64_data().len());
}

#[test]
fn manual_insert() {
    let mut ct: CoverTree<MetricF64> = CoverTree::new();

    let data = &test_f64_data();

    for point in data.iter() {
        ct.insert(*point);
    }
    assert_eq!(ct.len(), test_f64_data().len());
}

#[test]
fn nearest_neighbor_f64() {
    let mut ct: CoverTree<MetricF64> = CoverTree::from_items(test_f64_data().into_iter());

    assert_eq!(ct.find_nearest(0.0).unwrap(), &1.0);
    assert_eq!(ct.find_nearest(2.0).unwrap(), &1.0);
    assert_eq!(ct.find_nearest(5.0).unwrap(), &1.0);
    assert_eq!(ct.find_nearest(5.49).unwrap(), &1.0);
    assert_eq!(ct.find_nearest(5.5).unwrap(), &10.0);
    assert_eq!(ct.find_nearest(6.0).unwrap(), &10.0);
    assert_eq!(ct.find_nearest(91.0).unwrap(), &90.0);
    assert_eq!(ct.find_nearest(-91.0).unwrap(), &1.0);
    assert_eq!(ct.find_nearest(1000.0).unwrap(), &144.0);
}

#[test]
fn nearest_neighbor_i64() {
    let mut ct: CoverTree<MetricI64> = CoverTree::from_items(test_i64_data().into_iter());

    assert_eq!(ct.find_nearest(0).unwrap(), &0);
}