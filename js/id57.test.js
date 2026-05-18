import { test } from 'node:test';
import assert from 'node:assert/strict';
import {
  Alphabet,
  DefaultLength,
  IsSupportedLength,
  ValidateLength,
  FromDigest,
  Generate,
  GenerateString,
  Validate,
  IsValid,
  isSupportedLength,
  validateLength,
  fromDigest,
  generate,
  generateString,
  validate,
  isValid,
} from './id57.js';

test('DefaultLength is 12 and supported', () => {
  assert.equal(DefaultLength, 12);
  assert.ok(IsSupportedLength(DefaultLength));
  assert.ok(isSupportedLength(DefaultLength));
});

test('API aliases match the Go-style exports', () => {
  assert.equal(isSupportedLength, IsSupportedLength);
  assert.equal(validateLength, ValidateLength);
  assert.equal(fromDigest, FromDigest);
  assert.equal(generate, Generate);
  assert.equal(generateString, GenerateString);
  assert.equal(validate, Validate);
  assert.equal(isValid, IsValid);
});

test('isSupportedLength: supported', () => {
  for (const l of [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
    assert.ok(IsSupportedLength(l), `expected ${l} to be supported`);
    assert.ok(isSupportedLength(l), `expected ${l} to be supported`);
  }
});

test('isSupportedLength: unsupported', () => {
  for (const l of [0, 1, 7, 9, 11, 13, 15, 17, 31, 33, 56, 58, -1]) {
    assert.ok(!IsSupportedLength(l), `expected ${l} to be unsupported`);
    assert.ok(!isSupportedLength(l), `expected ${l} to be unsupported`);
  }
});

test('validateLength: does not throw for supported lengths', () => {
  for (const l of [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
    assert.doesNotThrow(() => ValidateLength(l));
    assert.doesNotThrow(() => validateLength(l));
  }
});

test('validateLength: throws for unsupported lengths', () => {
  for (const l of [0, 1, 7, 9, 11, 13, 15, 17, 31, 33, 56, 58, -1]) {
    assert.throws(() => ValidateLength(l), /unsupported length/);
    assert.throws(() => validateLength(l), /unsupported length/);
  }
});

const STABLE_VECTORS = [
  { length: 2, want: 'wp' },
  { length: 3, want: 'wpU' },
  { length: 4, want: 'wpUm' },
  { length: 5, want: 'wpUmW' },
  { length: 6, want: 'wpUmWi' },
  { length: 8, want: 'wpUmWi5r' },
  { length: 10, want: 'wpUmWi5rpG' },
  { length: 12, want: 'wpUmWi5rpGTs' },
  { length: 16, want: 'wpUmWi5rpGTsyPrP' },
  { length: 32, want: 'wpUmWi5rpGTsyPrPErnfB9JavNGdi4ym' },
  { length: 57, want: 'wpUmWi5rpGTsyPrPErnfB9JavNGdi4ymja5dD6jHTxuhAAAAAAAAAAAAA' },
];

test('stable vectors: GenerateString', () => {
  for (const { length, want } of STABLE_VECTORS) {
    const gotGoStyle = GenerateString('id57:stable:v1', length);
    const gotJsStyle = generateString('id57:stable:v1', length);
    assert.equal(gotGoStyle, want, `length ${length}`);
    assert.equal(gotJsStyle, want, `length ${length}`);
  }
});

test('generate parity and charset', () => {
  const input = 'string-bytes-parity';
  const banned = new Set('0oOIl');
  const bytes = new TextEncoder().encode(input);

  for (const l of [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
    const fromBytesGo = Generate(bytes, l);
    const fromBytesJs = generate(bytes, l);
    const fromStrGo = GenerateString(input, l);
    const fromStrJs = generateString(input, l);

    assert.equal(fromBytesGo, fromBytesJs, `length ${l}: Generate vs generate`);
    assert.equal(fromStrGo, fromStrJs, `length ${l}: GenerateString vs generateString`);
    assert.equal(fromBytesGo, fromStrGo, `length ${l}: bytes vs string`);

    for (const ch of fromBytesGo) {
      assert.ok(!banned.has(ch), `length ${l}: output contains banned char '${ch}'`);
    }

    assert.doesNotThrow(() => Validate(fromBytesGo, l), `length ${l}: Validate`);
    assert.doesNotThrow(() => validate(fromBytesGo, l), `length ${l}: validate`);
  }
});

test('fromDigest: zero digest is deterministic all-A', () => {
  const zeros = new Uint8Array(32);
  for (const l of [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]) {
    const gotGo = FromDigest(zeros, l);
    const gotJs = fromDigest(zeros, l);
    assert.equal(gotGo, 'A'.repeat(l), `length ${l}`);
    assert.equal(gotJs, 'A'.repeat(l), `length ${l}`);
  }
});

test('validate: invalid charset', () => {
  assert.throws(() => Validate('A0BCDE', 6), /ID57 alphabet/);
  assert.throws(() => validate('A0BCDE', 6), /ID57 alphabet/);
});

test('validate: length mismatch', () => {
  assert.throws(() => Validate('ABCDE', 6), /length does not match/);
  assert.throws(() => validate('ABCDE', 6), /length does not match/);
});

test('validate: unsupported length', () => {
  assert.throws(() => Validate('ABCDEFG', 7), /unsupported length/);
  assert.throws(() => validate('ABCDEFG', 7), /unsupported length/);
});

test('validate: valid length 2 passes', () => {
  assert.doesNotThrow(() => Validate('AB', 2));
  assert.doesNotThrow(() => validate('AB', 2));
});

test('isValid: true for valid, false for invalid', () => {
  assert.ok(IsValid('AB', 2));
  assert.ok(isValid('AB', 2));
  assert.ok(!IsValid('A0', 2));
  assert.ok(!isValid('A0', 2));
  assert.ok(!IsValid('A', 2));
  assert.ok(!isValid('A', 2));
  assert.ok(!IsValid('AB', 7));
  assert.ok(!isValid('AB', 7));
});

test('Alphabet has exactly 57 characters', () => {
  assert.equal(Alphabet.length, 57);
});

test('Alphabet contains no banned characters', () => {
  for (const ch of '0oOIl') {
    assert.ok(!Alphabet.includes(ch), `Alphabet contains banned char '${ch}'`);
  }
});