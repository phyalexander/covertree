
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

    ct.insert(5f64);
    ct.insert(6f64);
    ct.insert(16f64);
    ct.insert(23f64);
    ct.insert(1f64);
    ct.insert(11f64);
    ct.insert(12f64);
    ct.insert(18f64);
    ct.insert(81f64);
    ct.insert(91f64);
    ct.insert(10f64);
    ct.insert(235f64);
    ct.insert(15f64);
    ct.insert(115f64);
    ct.insert(125f64);
    ct.insert(236f64);
    ct.insert(17f64);
    ct.insert(116f64);
    ct.insert(126f64);
    println!("{}\n", ct.tree_string());


    println!("Nearest to 66: {}", ct.find_nearest(66f64).unwrap());
    println!("Nearest to 14: {}", ct.find_nearest(14f64).unwrap());
}