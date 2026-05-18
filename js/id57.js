import { blake3 } from '@noble/hashes/blake3';

/** AIS ID57 alphabet: Base62 minus 0, o, O, I, and l. */
export const Alphabet = 'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz123456789';

/** Default ID57 output length. */
export const DefaultLength = 12;

const ALPHABET_SIZE = BigInt(Alphabet.length);
const SUPPORTED = new Set([2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57]);
const ALPHABET_SET = new Set(Alphabet);

/**
 * Reports whether length is one of the supported ID57 sizes.
 * @param {number} length
 * @returns {boolean}
 */
export function IsSupportedLength(length) {
  return SUPPORTED.has(length);
}

/**
 * Throws if length is not a supported ID57 size.
 * @param {number} length
 */
export function ValidateLength(length) {
  if (!IsSupportedLength(length)) {
    throw new Error(
      `id57: unsupported length; must be 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, or 57; got ${length}`
    );
  }
}

/**
 * Encodes a BLAKE3 digest into a deterministic ID57 value.
 * A zero-length or all-zero digest produces all-'A' output.
 * @param {Uint8Array} digest
 * @param {number} length
 * @returns {string}
 */
export function FromDigest(digest, length) {
  ValidateLength(length);
  let n = digest.reduce((acc, b) => acc * 256n + BigInt(b), 0n);
  const buf = new Array(length);
  for (let i = 0; i < length; i++) {
    buf[i] = Alphabet[Number(n % ALPHABET_SIZE)];
    n = n / ALPHABET_SIZE;
  }
  return buf.join('');
}

/**
 * Hashes input with BLAKE3 and projects the digest into an ID57 value.
 * @param {Uint8Array} input
 * @param {number} length
 * @returns {string}
 */
export function Generate(input, length) {
  ValidateLength(length);
  return FromDigest(blake3(input), length);
}

/**
 * Convenience wrapper around Generate for UTF-8 string input.
 * @param {string} input
 * @param {number} length
 * @returns {string}
 */
export function GenerateString(input, length) {
  return Generate(new TextEncoder().encode(input), length);
}

/**
 * Checks that value has the correct length and only ID57 characters.
 * Throws a descriptive Error on the first violation.
 * @param {string} value
 * @param {number} length
 */
export function Validate(value, length) {
  ValidateLength(length);
  if (value.length !== length) {
    throw new Error(
      `id57: value length does not match the requested length: got ${value.length}, want ${length}`
    );
  }
  for (const ch of value) {
    if (!ALPHABET_SET.has(ch)) {
      throw new Error(
        `id57: value contains a character outside the ID57 alphabet: character '${ch}'`
      );
    }
  }
}

/**
 * Reports whether value is a valid ID57 value of the requested length.
 * @param {string} value
 * @param {number} length
 * @returns {boolean}
 */
export function IsValid(value, length) {
  try {
    Validate(value, length);
    return true;
  } catch {
    return false;
  }
}

export {
  IsSupportedLength as isSupportedLength,
  ValidateLength as validateLength,
  FromDigest as fromDigest,
  Generate as generate,
  GenerateString as generateString,
  Validate as validate,
  IsValid as isValid,
};