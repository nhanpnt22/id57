# @nhanpnt22/id57

Deterministic, human-readable identifiers using BLAKE3 and the AIS ID57 alphabet.

## Install

```bash
npm install @nhanpnt22/id57
```

## Usage

```js
import { GenerateString, Validate, IsValid, DefaultLength } from '@nhanpnt22/id57';

const value = GenerateString('hello world', DefaultLength);

Validate(value, DefaultLength);
console.log(IsValid(value, DefaultLength));
```

## API

The package exports both Go-style names and JS aliases for the same functions:

- `Alphabet`
- `DefaultLength`
- `IsSupportedLength` / `isSupportedLength`
- `ValidateLength` / `validateLength`
- `FromDigest` / `fromDigest`
- `Generate` / `generate`
- `GenerateString` / `generateString`
- `Validate` / `validate`
- `IsValid` / `isValid`

Supported lengths are 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, and 57.
