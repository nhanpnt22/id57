// Package id57 generates and validates deterministic human-readable identifiers
// using the AIS ID57 alphabet and BLAKE3.
//
// Canonical flow:
//
//	BLAKE3(input) -> base-57 projection -> ID57 string
//
// Supported lengths: 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57.
//
// The 32-byte BLAKE3 digest is interpreted as a big-endian integer and
// repeatedly divided by 57. Each remainder maps to a character in the ID57
// alphabet. This preserves deterministic, cross-language stable output.
package id57

import (
	"errors"
	"fmt"
	"math/big"

	"lukechampine.com/blake3"
)

// Alphabet is the AIS ID57 character set: Base62 minus 0, o, O, I, and l.
const Alphabet = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz123456789"

var (
	alphabetSize = big.NewInt(int64(len(Alphabet)))
	alphabetSet  [256]bool

	ErrUnsupportedLength = errors.New("id57: unsupported length; must be 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, or 57")
	ErrInvalidCharset    = errors.New("id57: value contains a character outside the ID57 alphabet")
	ErrLengthMismatch    = errors.New("id57: value length does not match the requested length")
)

func init() {
	for index := 0; index < len(Alphabet); index++ {
		alphabetSet[Alphabet[index]] = true
	}
}

// IsSupportedLength reports whether length is one of the supported ID57 sizes.
func IsSupportedLength(length int) bool {
	switch length {
	case 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57:
		return true
	}

	return false
}

// ValidateLength returns ErrUnsupportedLength if length is not one of 2, 3, 4, 5, 6, 8, 10, 12, 16, 32, or 57.
func ValidateLength(length int) error {
	if !IsSupportedLength(length) {
		return fmt.Errorf("%w: got %d", ErrUnsupportedLength, length)
	}

	return nil
}

// FromDigest encodes digest bytes into a deterministic ID57 value of the
// requested supported length.
//
// A nil or empty digest is treated as zero and produces deterministic all-'A'
// output for the requested length.
func FromDigest(digest []byte, length int) (string, error) {
	if err := ValidateLength(length); err != nil {
		return "", err
	}

	number := new(big.Int).SetBytes(digest)
	modulus := new(big.Int)
	buffer := make([]byte, length)

	for index := 0; index < length; index++ {
		number.DivMod(number, alphabetSize, modulus)
		buffer[index] = Alphabet[modulus.Int64()]
	}

	return string(buffer), nil
}

// Generate hashes input with BLAKE3 and projects the digest into an ID57 value.
func Generate(input []byte, length int) (string, error) {
	if err := ValidateLength(length); err != nil {
		return "", err
	}

	digest := blake3.Sum256(input)
	return FromDigest(digest[:], length)
}

// MustGenerate is like Generate but panics on error.
func MustGenerate(input []byte, length int) string {
	value, err := Generate(input, length)
	if err != nil {
		panic(err)
	}

	return value
}

// GenerateString is a convenience wrapper around Generate for UTF-8 string input.
func GenerateString(input string, length int) (string, error) {
	return Generate([]byte(input), length)
}

// Validate checks that value has a supported length, exact character count, and
// only ID57 characters.
func Validate(value string, length int) error {
	if err := ValidateLength(length); err != nil {
		return err
	}

	if len(value) != length {
		return fmt.Errorf("%w: got %d, want %d", ErrLengthMismatch, len(value), length)
	}

	for index := 0; index < len(value); index++ {
		if !alphabetSet[value[index]] {
			return fmt.Errorf("%w: character %q at index %d", ErrInvalidCharset, value[index], index)
		}
	}

	return nil
}

// IsValid reports whether value is a valid ID57 value of the requested length.
func IsValid(value string, length int) bool {
	return Validate(value, length) == nil
}
