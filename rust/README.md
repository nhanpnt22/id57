# id57

Deterministic, human-readable identifiers using BLAKE3 and the AIS ID57 alphabet.

## Install

```bash
cargo add id57
```

## Usage

```rust
use id57::{DefaultLength, GenerateString, IsValid, Validate};

fn main() -> Result<(), id57::Error> {
    let value = GenerateString("hello world", DefaultLength)?;

    Validate(&value, DefaultLength)?;
    assert!(IsValid(&value, DefaultLength));

    Ok(())
}
```

## API

The crate exposes both idiomatic Rust names and Go-style aliases for the same functions and constants.

- `ALPHABET` / `Alphabet`
- `DEFAULT_LENGTH` / `DefaultLength`
- `is_supported_length` / `IsSupportedLength`
- `validate_length` / `ValidateLength`
- `from_digest` / `FromDigest`
- `generate` / `Generate`
- `generate_string` / `GenerateString`
- `validate` / `Validate`
- `is_valid` / `IsValid`

Supported lengths are 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, and 57.