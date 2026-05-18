# id57

Deterministic, human-readable identifiers for JavaScript using BLAKE3 and the AIS ID57 alphabet.

## Install

```sh
npm install @nhanpnt22/id57
```

## Import

```js
import { GenerateString, IsValid } from '@nhanpnt22/id57';
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

```js
const Alphabet
const DefaultLength = 12
function IsSupportedLength(length)
function ValidateLength(length)
function FromDigest(digest, length)
function Generate(input, length)
function GenerateString(input, length)
function Validate(value, length)
function IsValid(value, length)
```

Quick start:

```js
import { GenerateString, IsValid } from '@nhanpnt22/id57';

const id = GenerateString('user:123', 8);

console.log(id);
console.log(IsValid(id, 8));
```

Release notes:

- JS exports stay in PascalCase to match the package surface.
- Package checks are validated with `node --test` and `npm pack --dry-run`.
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
