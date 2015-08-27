
extern crate num;

pub mod common;
pub mod simple;

type TreeItem = f64;

impl common::Metric<f64> for TreeItem {
    type Output = f64;
    fn distance(self, rhs: f64) -> Self::Output {
        (rhs - self).abs() as f64
    }
}

impl<'a> common::Metric<f64> for &'a TreeItem {
    type Output = f64;
    fn distance(self, rhs: f64) -> Self::Output {
        (rhs - self).abs() as f64
    }
}

impl<'b> common::Metric<&'b f64> for TreeItem {
    type Output = f64;
    fn distance(self, rhs: &'b f64) -> Self::Output {
        (rhs - self).abs() as f64
    }
}

impl<'a, 'b> common::Metric<&'b f64> for &'a TreeItem {
    type Output = f64;

    fn distance(self, rhs:&'b f64) -> Self::Output {
        (rhs - self).abs() as f64
    }
}

#[test]
fn test_node() {
    use simple::CoverTreeNode;

}


