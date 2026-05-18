import 'dart:convert';
import 'dart:typed_data';

import 'package:blake3_dart/blake3_dart.dart';

const String alphabet =
    'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz123456789';
const int defaultLength = 12;

const String Alphabet = alphabet;
const int DefaultLength = defaultLength;

const Set<int> _supportedLengths = <int>{2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57};
final Set<String> _alphabetSet = alphabet.split('').toSet();

abstract class Id57Exception implements Exception {
  const Id57Exception(this.message);

  final String message;

  @override
  String toString() => message;
}

class UnsupportedLengthException extends Id57Exception {
  const UnsupportedLengthException(this.length)
    : super(
        'id57: unsupported length; must be 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, or 57; got $length',
      );

  final int length;
}

class LengthMismatchException extends Id57Exception {
  const LengthMismatchException(this.got, this.want)
    : super(
        'id57: value length does not match the requested length: got $got, want $want',
      );

  final int got;
  final int want;
}

class InvalidCharsetException extends Id57Exception {
  const InvalidCharsetException(this.character, this.index)
    : super(
        "id57: value contains a character outside the ID57 alphabet: character '$character' at index $index",
      );

  final String character;
  final int index;
}

bool isSupportedLength(int length) => _supportedLengths.contains(length);

bool IsSupportedLength(int length) => isSupportedLength(length);

void validateLength(int length) {
  if (!isSupportedLength(length)) {
    throw UnsupportedLengthException(length);
  }
}

void ValidateLength(int length) => validateLength(length);

BigInt _bytesToBigInt(List<int> bytes) {
  var number = BigInt.zero;
  for (final byte in bytes) {
    number = (number << 8) + BigInt.from(byte);
  }
  return number;
}

String fromDigest(List<int> digest, int length) {
  validateLength(length);

  var number = digest.isEmpty ? BigInt.zero : _bytesToBigInt(digest);
  final base = BigInt.from(alphabet.length);
  final buffer = List<String>.filled(length, 'A');

  for (var i = 0; i < length; i++) {
    final remainder = (number % base).toInt();
    buffer[i] = alphabet[remainder];
    number ~/= base;
  }

  return buffer.join();
}

String FromDigest(List<int> digest, int length) => fromDigest(digest, length);

String generate(List<int> input, int length) {
  validateLength(length);
  return fromDigest(blake3(Uint8List.fromList(input), 32), length);
}

String Generate(List<int> input, int length) => generate(input, length);

String generateString(String input, int length) {
  return generate(utf8.encode(input), length);
}

String GenerateString(String input, int length) =>
    generateString(input, length);

void validate(String value, int length) {
  validateLength(length);

  if (value.length != length) {
    throw LengthMismatchException(value.length, length);
  }

  for (var index = 0; index < value.length; index++) {
    final character = String.fromCharCode(value.codeUnitAt(index));
    if (!_alphabetSet.contains(character)) {
      throw InvalidCharsetException(character, index);
    }
  }
}

void Validate(String value, int length) => validate(value, length);

bool isValid(String value, int length) {
  try {
    validate(value, length);
    return true;
  } on Id57Exception {
    return false;
  }
}

bool IsValid(String value, int length) => isValid(value, length);
