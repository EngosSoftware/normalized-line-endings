### normalized-line-endings

[![crates.io][crates-badge]][crates-url]
![build][build-badge]
![coverage][cov-badge]
![mbh][mbh-badge]

[crates-badge]: https://img.shields.io/crates/v/normalized-line-endings.svg
[crates-url]: https://crates.io/crates/normalized-line-endings
[cov-badge]: https://img.shields.io/badge/cov-100%25-21b577.svg
[build-badge]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build.yml/badge.svg
[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-F5DEB3.svg

# Line endings normalizer

This crate provides an iterator over characters with normalized line endings,
meaning all valid line endings in the input are converted to a single newline
character: `\n` (U+000A), like this:

- `\n` → `\n`
- `\r` → `\n`
- `\r\n` → `\n`
 
The normalized iterator can be created by calling method `normalized`
on any iterator over characters, like in this example:

```rust
use normalized_line_endings::Normalized;

fn main() {
  let input = "Lorem \n ipsum \r dolor \n\r\n sit \r\r\n\n amet";
  assert_eq!(
    "Lorem \n ipsum \n dolor \n\n sit \n\n\n amet",
    input.chars().normalized().collect::<String>()
  );
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
