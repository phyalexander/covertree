
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
    fn test_empty_tree() {
        let ct: CoverTree<MetricF64> = CoverTree::new();
    }

    #[test]
    fn test_from_items() {
        let ct: CoverTree<MetricF64> = CoverTree::from_items(test_f64_data().into_iter());
    }

    #[test]
    fn test_manual_insert() {
        let mut ct: CoverTree<MetricF64> = CoverTree::new();

        let data = &test_f64_data();

        for point in data.iter() {
            ct.insert(*point);
        }
    }
}