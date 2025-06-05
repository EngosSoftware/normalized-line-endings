# Normalized line endings

[![Crates.io][crates-badge]][crates-url]
![Code coverage][coverage-badge]
![build Linux][build-badge-linux]
![build Windows][build-badge-windows]
![build MacOs][build-badge-macos]
![build MacOs arm64][build-badge-macos-arm64]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]
[![Made by Human][mbh-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/normalized-line-endings.svg
[crates-url]: https://crates.io/crates/normalized-line-endings
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/LICENSE
[apache-notice-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/NOTICE
[build-badge-linux]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-linux.yml/badge.svg
[build-badge-windows]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-windows.yml/badge.svg
[build-badge-macos]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-macos.yml/badge.svg
[build-badge-macos-arm64]: https://github.com/EngosSoftware/normalized-line-endings/actions/workflows/build-macos-arm64.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Code%20coverage-100%25-green.svg
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/EngosSoftware/normalized-line-endings/blob/main/CODE_OF_CONDUCT.md
[mbh-badge]: https://img.shields.io/badge/Made_by-HUMAN-d35400.svg
[repository-url]: https://github.com/EngosSoftware/normalized-line-endings

## Overview

The **Normalized Line Endings** crate provides an iterator over characters
with normalized line endings, meaning all valid line endings in the input
are converted to a single newline character: `\n` (U+000A).

For example:
- `\n` → `\n`
- `\r` → `\n`
- `\r\n` → `\n`
 
The normalized iterator can be created using standalone function `normalized`
or by calling the method `normalized` on any iterator over characters.

This work was inspired by [normalize-line-endings](https://crates.io/crates/normalize-line-endings) crate.

## Examples

### Using standalone function

```rust
use normalized_line_endings::normalized;

let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
assert_eq!(
  "This is a string \n with \n some \n\n random newlines\n\n\n",
  normalized(input.chars()).collect::<String>()
);
```

### Using trait extension

```rust
use normalized_line_endings::Normalized;

let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
assert_eq!(
  "This is a string \n with \n some \n\n random newlines\n\n\n",
  input.chars().normalized().collect::<String>()
);
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**normalized-line-endings**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.

---

Brought to you with 💙 by [Engos Software](https://engos.de)
