
extern crate treedisplay;

pub mod simple;
pub mod common;

use simple::CoverTree;
use common::NearestNeighbor;
use treedisplay::TreeDisplay;

type TreeItem = f64;

impl common::Metric<f64> for TreeItem {
    fn distance(self, rhs: f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl<'a> common::Metric<f64> for &'a TreeItem {
    fn distance(self, rhs: f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl<'b> common::Metric<&'b f64> for TreeItem {
    fn distance(self, rhs: &'b f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl<'a, 'b> common::Metric<&'b f64> for &'a TreeItem {

    fn distance(self, rhs:&'b f64) -> f64 {
        (rhs - self).abs() as f64
    }
}

impl common::CoverTreeData for TreeItem {}

#[allow(dead_code)]
fn main() {

    let mut ct: CoverTree<TreeItem> = CoverTree::new();

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
    println!("{}\n", ct.tree_string());


    println!("Nearest to 66: {}", ct.find_nearest(66.0).unwrap());
    println!("Nearest to 93: {}", ct.find_nearest(93.0).unwrap());
    println!("Nearest to 94: {}", ct.find_nearest(94.0).unwrap());
}