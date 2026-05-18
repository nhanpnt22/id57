# id57

Deterministic, human-readable identifiers using BLAKE3 and the AIS ID57 alphabet.

## Install

```bash
dart pub add id57
```

## Usage

```dart
import 'package:id57/id57.dart';

void main() {
  final value = GenerateString('hello world', DefaultLength);
  Validate(value, DefaultLength);
  print(IsValid(value, DefaultLength));
}
```

## API

The package exposes both Dart-idiomatic names and Go-style aliases for the same behavior.

- `alphabet` / `Alphabet`
- `defaultLength` / `DefaultLength`
- `isSupportedLength` / `IsSupportedLength`
- `validateLength` / `ValidateLength`
- `fromDigest` / `FromDigest`
- `generate` / `Generate`
- `generateString` / `GenerateString`
- `validate` / `Validate`
- `isValid` / `IsValid`

Supported lengths are 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, and 57.