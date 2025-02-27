# Cover Tree
## Cover tree data structure implemented in Rust

![Static Badge](https://img.shields.io/badge/phyalexander-covertree-covertree)
![GitHub top language](https://img.shields.io/github/languages/top/phyalexander/covertree)
![GitHub](https://img.shields.io/github/license/phyalexander/covertree)
![GitHub Repo stars](https://img.shields.io/github/stars/phyalexander/covertree)
![GitHub issues](https://img.shields.io/github/issues/phyalexander/covertree)
[![Crate](https://img.shields.io/crates/v/covertree.svg)](https://crates.io/crates/covertree)
[![API](https://docs.rs/covertree/badge.svg)](https://docs.rs/covertree)

### Overview

The Cover Tree is a data structure designed for efficient nearest neighbor search in metric spaces.
It is particularly useful for applications in machine learning, data mining, and computational
geometry where fast querying of high-dimensional data is required. The Cover Tree achieves this by
maintaining a hierarchical structure that allows for logarithmic time complexity in both insertion
and query operations.


---
### Acknowledgments

- The Cover Tree algorithm was [originally introduced](https://norbertwiener.umd.edu/Education/IMA2015/t6/CoverTrees_Beygelzimer_Kakade_Langford.pdf)
    by Alina Beygelzimer, Sham Kakade, and John Langford.
- The [improvement](https://izbicki.me/public/papers/icml2015-faster-cover-trees.pdf)
    of the approach was suggested by Mike Izbicki and Christian R. Shelton
- Special thanks to [solarretrace](https://github.com/solarretrace)
  for their original code, which served as the foundation for this implementation.


---
### Features

- **Efficient Nearest Neighbor Search**: The Cover Tree allows for fast querying of the nearest 
    neighbors in logarithmic time.
- **Updating structure**: Simple API for insertion, deletion, and querying operations.
- **Scalability**: Designed to handle large datasets efficiently, 
    making it suitable for high-dimensional data.


---
### Installation

To use the Cover Tree in your project, you can:

* Run the following Cargo command in your project directory:
    ```bash
    cargo add covertree
    ```
* Or add the following line to your `Cargo.toml` (specify required version):
    ```toml
    covertree = "1.0.0"
    ```


---
### Usage

Here is a quick example of how to use the Cover Tree:

```rust
println!("hello world");
```


---
### Documentation

For detailed documentation on the API and advanced usage, please visit the [documentation
site](https://docs.rs/covertree)

---
### License
This project is licensed under [MIT](http://opensource.org/licenses/MIT) or
[Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0) licenses.
**Feel free to choose the one you want.**