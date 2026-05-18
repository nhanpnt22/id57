import 'dart:convert';
import 'dart:typed_data';

import 'package:id57/id57.dart';
import 'package:test/test.dart';

void main() {
  test('DefaultLength is 12 and supported', () {
    expect(DefaultLength, 12);
    expect(IsSupportedLength(DefaultLength), isTrue);
    expect(isSupportedLength(DefaultLength), isTrue);
  });

  test('supported and unsupported lengths', () {
    for (final length in [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
      expect(IsSupportedLength(length), isTrue);
      expect(isSupportedLength(length), isTrue);
      expect(() => ValidateLength(length), returnsNormally);
      expect(() => validateLength(length), returnsNormally);
    }

    for (final length in [0, 1, 7, 9, 11, 13, 15, 17, 31, 33, 56, 58, -1]) {
      expect(IsSupportedLength(length), isFalse);
      expect(isSupportedLength(length), isFalse);
      expect(
        () => ValidateLength(length),
        throwsA(isA<UnsupportedLengthException>()),
      );
      expect(
        () => validateLength(length),
        throwsA(isA<UnsupportedLengthException>()),
      );
    }
  });

  const stableVectors = [
    (2, 'wp'),
    (3, 'wpU'),
    (4, 'wpUm'),
    (5, 'wpUmW'),
    (6, 'wpUmWi'),
    (8, 'wpUmWi5r'),
    (10, 'wpUmWi5rpG'),
    (12, 'wpUmWi5rpGTs'),
    (16, 'wpUmWi5rpGTsyPrP'),
    (32, 'wpUmWi5rpGTsyPrPErnfB9JavNGdi4ym'),
    (57, 'wpUmWi5rpGTsyPrPErnfB9JavNGdi4ymja5dD6jHTxuhAAAAAAAAAAAAA'),
  ];

  test('stable vectors: generateString', () {
    for (final vector in stableVectors) {
      final length = vector.$1;
      final want = vector.$2;
      expect(GenerateString('id57:stable:v1', length), want);
      expect(generateString('id57:stable:v1', length), want);
    }
  });

  test('generate parity and charset', () {
    const input = 'string-bytes-parity';
    const banned = '0oOIl';
    final bytes = Uint8List.fromList(utf8.encode(input));

    for (final length in [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
      final fromBytesGo = Generate(bytes, length);
      final fromBytesJs = generate(bytes, length);
      final fromStrGo = GenerateString(input, length);
      final fromStrJs = generateString(input, length);

      expect(fromBytesGo, fromBytesJs);
      expect(fromStrGo, fromStrJs);
      expect(fromBytesGo, fromStrGo);

      for (final ch in fromBytesGo.split('')) {
        expect(banned.contains(ch), isFalse);
      }

      expect(() => Validate(fromBytesGo, length), returnsNormally);
      expect(() => validate(fromBytesGo, length), returnsNormally);
      expect(IsValid(fromBytesGo, length), isTrue);
      expect(isValid(fromBytesGo, length), isTrue);
    }
  });

  test('fromDigest: zero digest is deterministic all-A', () {
    final zeros = Uint8List(32);

    for (final length in [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
      expect(FromDigest(zeros, length), 'A' * length);
      expect(fromDigest(zeros, length), 'A' * length);
    }
  });

  test('validate errors', () {
    expect(
      () => Validate('A0BCDE', 6),
      throwsA(isA<InvalidCharsetException>()),
    );
    expect(
      () => validate('A0BCDE', 6),
      throwsA(isA<InvalidCharsetException>()),
    );
    expect(() => Validate('ABCDE', 6), throwsA(isA<LengthMismatchException>()));
    expect(() => validate('ABCDE', 6), throwsA(isA<LengthMismatchException>()));
    expect(
      () => Validate('ABCDEFG', 7),
      throwsA(isA<UnsupportedLengthException>()),
    );
    expect(
      () => validate('ABCDEFG', 7),
      throwsA(isA<UnsupportedLengthException>()),
    );
    expect(() => Validate('AB', 2), returnsNormally);
    expect(() => validate('AB', 2), returnsNormally);
  });

  test('alphabet has expected shape', () {
    expect(Alphabet.length, 57);
    for (final ch in '0oOIl'.split('')) {
      expect(Alphabet.contains(ch), isFalse);
    }
  });
}
