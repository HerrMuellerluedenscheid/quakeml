# QuakeML deserialization in rust

![crates.io](https://img.shields.io/crates/v/quakeml.svg)

[This python implementation](https://git.gfz-potsdam.de/nooshiri/pyquakeml/-/blob/master/src/pyquakeml.py)
by Nima Nooshiri has been used as reference.

## Usage

```rust
    use std::path::PathBuf;
    use crate::quakeml::read_quakeml;

    fn main() {
        let path = PathBuf::from(r"sample/data.quakeml");
        let catalog = read_quakeml(&path);
        println!("catalog data: {}", catalog);
    }
```
