# id57

Deterministic, human-readable identifiers for Dart using BLAKE3 and the AIS ID57 alphabet.

## Install

```sh
dart pub add id57
```

## Import

```dart
import 'package:id57/id57.dart';
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

```dart
String generate(List<int> input, int length)
String generateString(String input, int length)
String fromDigest(List<int> digest, int length)
void validateLength(int length)
void validate(String value, int length)
bool isValid(String value, int length)

const int defaultLength = 12
```

Exceptions:

- `UnsupportedLengthException`
- `LengthMismatchException`
- `InvalidCharsetException`

Quick start:

```dart
void main() {
  final id = generateString('user:123', 8);

  print(id);
  print(isValid(id, 8));
}
```

Release notes:

- Dart exposes camelCase helpers plus capitalized aliases for parity with the other branches.
- Package checks are validated with `dart test` and `dart pub publish --dry-run`.
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
