# id57

Deterministic, human-readable identifiers for Rust using BLAKE3 and the AIS ID57 alphabet.

## Install

```sh
cargo add id57
```

## Import

```rust
use id57::{generate_string, is_valid};
```

Canonical flow:

```text
BLAKE3(input) -> base-57 projection -> ID57 string
```

Alphabet:

```text
ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz123456789
```

Supported lengths:

- `2`
- `3`
- `4`
- `5`
- `6`
- `8`
- `10`
- `12` *(default)*
- `16`
- `32`
- `57`

API:

```rust
pub const ALPHABET: &str
pub const DEFAULT_LENGTH: usize = 12
pub fn is_supported_length(length: usize) -> bool
pub fn validate_length(length: usize) -> Result<(), Error>
pub fn from_digest(digest: &[u8], length: usize) -> Result<String, Error>
pub fn generate(input: &[u8], length: usize) -> Result<String, Error>
pub fn generate_string(input: &str, length: usize) -> Result<String, Error>
pub fn validate(value: &str, length: usize) -> Result<(), Error>
pub fn is_valid(value: &str, length: usize) -> bool
```

Errors:

- `Error`
- `UnsupportedLengthError`
- `InvalidCharsetError`
- `LengthMismatchError`

Quick start:

```rust
use id57::{generate_string, is_valid};

fn main() -> Result<(), id57::Error> {
    let id = generate_string("user:123", 8)?;

    println!("{id}");
    println!("{}", is_valid(&id, 8));
    Ok(())
}
```

Release notes:

- Crate surface uses snake_case helpers with `Error` variants for invalid input and unsupported lengths.
- Package checks are validated with `cargo test` and `cargo package --allow-dirty --list`.
- Stable vectors are keyed by `id57:stable:v1` and cover all supported output lengths.

Stable vectors:

```text
id57:stable:v1 + 2  -> wp
id57:stable:v1 + 3  -> wpU
id57:stable:v1 + 4  -> wpUm
id57:stable:v1 + 5  -> wpUmW
id57:stable:v1 + 6  -> wpUmWi
id57:stable:v1 + 8  -> wpUmWi5r
id57:stable:v1 + 10 -> wpUmWi5rpG
id57:stable:v1 + 12 -> wpUmWi5rpGTs
id57:stable:v1 + 16 -> wpUmWi5rpGTsyPrP
id57:stable:v1 + 32 -> wpUmWi5rpGTsyPrPErnfB9JavNGdi4ym
id57:stable:v1 + 57 -> wpUmWi5rpGTsyPrPErnfB9JavNGdi4ymja5dD6jHTxuhAAAAAAAAAAAAA
```
