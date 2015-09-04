
extern crate num;
extern crate treedisplay;

pub mod metric;
pub mod common;
pub mod simple;



#[cfg(test)]
mod tests {
    use metric::MetricF64;
    use simple::CoverTree;
    use common::NearestNeighbor;

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

    #[test]
    fn empty_tree() {
        let ct: CoverTree<MetricF64> = CoverTree::new();
        assert_eq!(ct.count(), 0);
    }

    #[test]
    fn from_items() {
        let ct: CoverTree<MetricF64> = CoverTree::from_items(test_f64_data().into_iter());
        assert_eq!(ct.count(), test_f64_data().len());
    }

    #[test]
    fn manual_insert() {
        let mut ct: CoverTree<MetricF64> = CoverTree::new();

        let data = &test_f64_data();

        for point in data.iter() {
            ct.insert(*point);
        }
        assert_eq!(ct.count(), test_f64_data().len());
    }

    #[test]
    fn nearest_neighbor() {
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
}