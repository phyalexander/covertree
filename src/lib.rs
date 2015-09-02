
extern crate num;
extern crate treedisplay;

pub mod metric;
pub mod common;
pub mod simple;


#[test]
fn test_simple_cover_tree() {
    use metric::MetricF64;
    use simple::CoverTree;
    use common::NearestNeighbor;
    let mut ct: CoverTree<MetricF64> = CoverTree::new();

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

