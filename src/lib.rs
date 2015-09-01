
extern crate num;
extern crate treedisplay;

pub mod common;
pub mod simple;

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

#[test]
fn test_simple_cover_tree() {
    use simple::CoverTree;
    use common::NearestNeighbor;
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

    assert_eq!(ct.find_nearest(10.0), Some(&10.0));
    assert_eq!(ct.find_nearest(94.0), Some(&90.0));
    assert_eq!(ct.find_nearest(80.0), Some(&78.0));
    assert_eq!(ct.find_nearest(3.0), Some(&1.0));
    assert_eq!(ct.find_nearest(121.0), Some(&122.0));
}

