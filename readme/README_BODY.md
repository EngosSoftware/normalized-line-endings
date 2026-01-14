**Line endings normalizer**

## Overview

This crate provides an iterator over characters with normalized line endings,
meaning all valid line endings in the input are converted to a single newline
character: `\n` (U+000A), like this:

- `\n` → `\n`
- `\r` → `\n`
- `\r\n` → `\n`
 
The normalized iterator can be created using standalone function `normalized`
or by calling the method `normalized` on any iterator over characters.

## Examples

### Using standalone function

```rust
use normalized_line_endings::normalized;

fn using_standalone_function_should_work() {
  let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
  assert_eq!(
    "This is a string \n with \n some \n\n random newlines\n\n\n",
    normalized(input.chars()).collect::<String>()
  );
}
```

### Using trait extension

```rust
use normalized_line_endings::Normalized;

fn using_trait_extension_should_work() {
  let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
  assert_eq!(
    "This is a string \n with \n some \n\n random newlines\n\n\n",
    input.chars().normalized().collect::<String>()
  );
}
```
