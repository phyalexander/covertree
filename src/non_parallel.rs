// Copyright 2018 Skylor R. Schermer.
// Copyright 2025 phyalex.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use Point;
use cover::Cover;
use DEFAULT_SPAN_FACTOR;

use std::default;
use std::mem;


/// A cover tree containing [`Point`]s of type P.
///
/// [`Point`]: trait.Point.html
#[derive(Debug, Clone, PartialEq)]
pub struct CoverTree<P> where P: Point {
    /// The root of the tree.
    root: Option<Cover<P>>,
    /// The span factor for each Cover.
    span_factor: f64,
    /// The number of items in the tree.
    len: usize
}


impl<P> CoverTree<P> where P: Point {
    /// Constructs an empty `CoverTree`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #    
    /// let cover_tree: CoverTree<f32> = CoverTree::new();
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn new() -> CoverTree<P> {
        Default::default()
    }

    /// Constructs an empty `CoverTree` with the specified span factor.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #    
    /// let cover_tree: CoverTree<f32> = CoverTree::with_span_factor(2.0);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn with_span_factor(span_factor: f64) -> Self {
        CoverTree {
            root: None,
            span_factor: span_factor,
            len: 0,
        }
    }

    /// Constructs a `CoverTree` containing all of the [`Point`]s in the given 
    /// [`Iterator`].
    ///
    /// [`Point`]: trait.Point.html
    /// [`Iterator`]: http://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn from_items<I>(points: I) -> Self where I: Iterator<Item=P> {
        let mut tree = CoverTree::new();
        tree.insert_all(points);
        tree
    }

    // TODO(Sky): Can we avoid mut here?
    /// Returns the point nearest to the given of [`Point`] in the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// let nearest = cover_tree.find_nearest(1.2).unwrap();
    /// assert_eq!(nearest, &1.3f32);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn find_nearest<'a>(&'a mut self, query_point: P) -> Option<&'a P> {
        if let Some(ref mut cover) = self.root {
            Some(cover.find_nearest(query_point, None))
        } else {
            None
        }
    }

    /// Returns the number of [`Point`]s in the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the span factor of the `CoverTree`.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::with_span_factor(2.1);
    /// 
    /// assert_eq!(cover_tree.span_factor(), 2.1f64);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn span_factor(&self) -> f64 {
        self.span_factor
    }

    /// Returns `true` if the `CoverTree` is empty.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert!(!cover_tree.is_empty());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts the given [`Point`] into the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::new();
    ///
    /// cover_tree.insert(1.3);
    /// cover_tree.insert(1.52);
    /// 
    /// assert_eq!(cover_tree.len(), 2);
    /// assert_eq!(cover_tree.find_nearest(1.4).unwrap(), &1.3f32);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn insert(&mut self, point: P) {
        let new_cover = Cover::new(point, 1);
        if let Some(ref mut cover) = self.root {
            let n = mem::replace(cover, new_cover);
            mem::replace(cover, n.insert(point, self.span_factor));
        } else {
            self.root = Some(new_cover);
        }

        self.len += 1;
    }

    /// Inserts each of the [`Point`]s in the given [`Iterator`] into the 
    /// `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// [`Iterator`]: http://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::new();
    ///
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    ///
    /// cover_tree.insert_all(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn insert_all<I>(&mut self, points: I) where I: Iterator<Item=P> {
        for point in points {
            self.insert(point);
        }
    }

    /// Removes the given [`Point`] from the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    ///
    /// cover_tree.remove(1.3);
    ///
    /// assert_eq!(cover_tree.len(), 3);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn remove(&mut self, point: P) -> Option<P> {
        if let Some(ref mut cover) = self.root {
            let removed = cover.remove(point);
            if removed.is_some() {self.len -= 1;}
            removed
        } else {
            None
        }
    }

    /// Removes each [`Point`] in the given [`Iterator`] from the `CoverTree`.
    ///
    /// [`Point`]: trait.Point.html
    /// [`Iterator`]: http://doc.rust-lang.org/std/iter/trait.Iterator.html
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// let mut cover_tree = CoverTree::from_items(nums.into_iter());
    /// 
    /// assert_eq!(cover_tree.len(), 4);
    ///
    /// let unwanted = vec![1.0, 2.3, 4.6];
    /// cover_tree.remove_all(unwanted.into_iter());
    ///
    /// assert_eq!(cover_tree.len(), 2);
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn remove_all<I>(&mut self, points: I) where I: Iterator<Item=P> {
        for point in points {
            self.remove(point);
        }
    }

    /// Removes all points from the `CoverTree`.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use covertree::CoverTree;
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// # 
    /// let nums: Vec<f32> = vec![1.0, 1.3, 3.5, 4.6];
    /// 
    /// let mut cover_tree: CoverTree<f32> = CoverTree::from_items(nums.into_iter());
    /// 
    /// cover_tree.clear();
    /// assert!(cover_tree.is_empty());
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn clear(&mut self) {
        self.root = None;
        self.len = 0;
    }
}


impl<P> default::Default for CoverTree<P> where P: Point {
    fn default() -> Self {
        CoverTree::with_span_factor(DEFAULT_SPAN_FACTOR)
    }
}

