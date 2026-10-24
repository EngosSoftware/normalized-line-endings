### normalized-line-endings

[![crates.io][crates-badge]][crates-url]
[![coverage][cov-badge]][cov-url]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build macOs][build-badge-macos]
![build macOs arm64][build-badge-macos-arm64]  
[![mit-license][mit-badge]][mit-license-url]
[![apache-license][apache-badge]][apache-license-url]
[![cc][cc-badge]][cc-url]
[![mbh][mbh-badge]][mbh-url]
[![es][es-badge]][es-url]

[crates-badge]: https://img.shields.io/crates/v/normalized-line-endings.svg
[crates-url]: https://crates.io/crates/normalized-line-endings
[cov-badge]: https://img.shields.io/badge/coverage-100%25-21b577.svg
[cov-url]: https://crates.io/crates/coverio
[build-badge-linux]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-macos-arm64.yml/badge.svg
[mit-badge]: https://img.shields.io/badge/License-MIT-4169E1.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-4169E1.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/LICENSE-APACHE
[apache-notice-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Code%20of%20Conduct-2.1-4169E1.svg
[cc-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/CODE_OF_CONDUCT.md
[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-DC143C.svg
[mbh-url]: https://github.com/DariuszDepta
[es-badge]: https://img.shields.io/badge/at-Engos_Software-32CD32.svg
[es-url]: https://engos.de
[repository-url]: https://github.com/EngosSoftware/normalized-line-endings

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

- [Apache License, Version 2.0][apache-url] (see [LICENSE-APACHE][apache-license-url] and [NOTICE][apache-notice-url])
- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or

at your option.

## Contribution

Any contributions to [normalized-line-endings][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
