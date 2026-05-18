package id57

import (
	"errors"
	"strings"
	"testing"
)

func TestIsSupportedLength(t *testing.T) {
	supported := []int{2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57}
	unsupported := []int{0, 1, 7, 9, 11, 13, 15, 17, 31, 33, 56, 58, -1}

	for _, length := range supported {
		if !IsSupportedLength(length) {
			t.Fatalf("IsSupportedLength(%d) = false, want true", length)
		}
	}

	for _, length := range unsupported {
		if IsSupportedLength(length) {
			t.Fatalf("IsSupportedLength(%d) = true, want false", length)
		}
	}
}

func TestValidateLength(t *testing.T) {
	for _, length := range []int{2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57} {
		if err := ValidateLength(length); err != nil {
			t.Fatalf("ValidateLength(%d): unexpected error: %v", length, err)
		}
	}

	for _, length := range []int{0, 1, 7, 9, 11, 13, 15, 17, 31, 33, 56, 58, -1} {
		err := ValidateLength(length)
		if !errors.Is(err, ErrUnsupportedLength) {
			t.Fatalf("ValidateLength(%d): got %v, want ErrUnsupportedLength", length, err)
		}
	}
}

func TestGenerateStableVectors(t *testing.T) {
	cases := []struct {
		input  string
		length int
		want   string
	}{
		{input: "id57:stable:v1", length: 2, want: "wp"},
		{input: "id57:stable:v1", length: 3, want: "wpU"},
		{input: "id57:stable:v1", length: 4, want: "wpUm"},
		{input: "id57:stable:v1", length: 5, want: "wpUmW"},
		{input: "id57:stable:v1", length: 6, want: "wpUmWi"},
		{input: "id57:stable:v1", length: 8, want: "wpUmWi5r"},
		{input: "id57:stable:v1", length: 10, want: "wpUmWi5rpG"},
		{input: "id57:stable:v1", length: 12, want: "wpUmWi5rpGTs"},
		{input: "id57:stable:v1", length: 16, want: "wpUmWi5rpGTsyPrP"},
		{input: "id57:stable:v1", length: 32, want: "wpUmWi5rpGTsyPrPErnfB9JavNGdi4ym"},
		{input: "id57:stable:v1", length: 57, want: "wpUmWi5rpGTsyPrPErnfB9JavNGdi4ymja5dD6jHTxuhAAAAAAAAAAAAA"},
	}

	for _, testCase := range cases {
		got, err := GenerateString(testCase.input, testCase.length)
		if err != nil {
			t.Fatalf("GenerateString(%q, %d): %v", testCase.input, testCase.length, err)
		}

		if got != testCase.want {
			t.Fatalf("GenerateString(%q, %d) = %q, want %q", testCase.input, testCase.length, got, testCase.want)
		}
	}
}

func TestGenerateParityAndCharset(t *testing.T) {
	const input = "string-bytes-parity"
	const banned = "0oOIl"

	for _, length := range []int{2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57} {
		fromBytes, err := Generate([]byte(input), length)
		if err != nil {
			t.Fatalf("Generate(_, %d): %v", length, err)
		}

		fromString, err := GenerateString(input, length)
		if err != nil {
			t.Fatalf("GenerateString(_, %d): %v", length, err)
		}

		if fromBytes != fromString {
			t.Fatalf("length %d: Generate=%q GenerateString=%q", length, fromBytes, fromString)
		}

		if strings.ContainsAny(fromBytes, banned) {
			t.Fatalf("Generate(_, %d): output %q contains banned characters", length, fromBytes)
		}

		if err := Validate(fromBytes, length); err != nil {
			t.Fatalf("Validate(%q, %d): %v", fromBytes, length, err)
		}
	}
}

func TestFromDigestZeroDeterministic(t *testing.T) {
	zeros := make([]byte, 32)

	for _, length := range []int{2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57} {
		got, err := FromDigest(zeros, length)
		if err != nil {
			t.Fatalf("FromDigest(_, %d): %v", length, err)
		}

		if got != strings.Repeat("A", length) {
			t.Fatalf("FromDigest(zeros, %d) = %q, want %q", length, got, strings.Repeat("A", length))
		}
	}
}

func TestValidateErrors(t *testing.T) {
	if err := Validate("A0BCDE", 6); !errors.Is(err, ErrInvalidCharset) {
		t.Fatalf("Validate invalid charset: got %v, want ErrInvalidCharset", err)
	}

	if err := Validate("ABCDE", 6); !errors.Is(err, ErrLengthMismatch) {
		t.Fatalf("Validate length mismatch: got %v, want ErrLengthMismatch", err)
	}

	if err := Validate("ABCDEFG", 7); !errors.Is(err, ErrUnsupportedLength) {
		t.Fatalf("Validate unsupported length: got %v, want ErrUnsupportedLength", err)
	}

	if err := Validate("AB", 2); err != nil {
		t.Fatalf("Validate valid length 2: unexpected error: %v", err)
	}
}

func TestMustGeneratePanicsOnInvalidLength(t *testing.T) {
	defer func() {
		if recovered := recover(); recovered == nil {
			t.Fatal("MustGenerate with invalid length did not panic")
		}
	}()

	_ = MustGenerate([]byte("x"), 7)
}
