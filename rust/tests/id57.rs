use id57::{
    Alphabet, DefaultLength, FromDigest, Generate, GenerateString, IsSupportedLength, IsValid,
    Validate, ValidateLength,
};

#[test]
fn default_length_is_12_and_supported() {
    assert_eq!(DefaultLength, 12);
    assert!(IsSupportedLength(DefaultLength));
}

#[test]
fn supported_and_unsupported_lengths() {
    for length in [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57] {
        assert!(IsSupportedLength(length));
        assert!(ValidateLength(length).is_ok());
    }

    for length in [0usize, 1, 7, 9, 11, 13, 15, 17, 31, 33, 56, 58] {
        assert!(!IsSupportedLength(length));
        assert!(ValidateLength(length).is_err());
    }
}

#[test]
fn stable_vectors_match_go_and_js() {
    let cases = [
        (2, "wp"),
        (3, "wpU"),
        (4, "wpUm"),
        (5, "wpUmW"),
        (6, "wpUmWi"),
        (8, "wpUmWi5r"),
        (10, "wpUmWi5rpG"),
        (12, "wpUmWi5rpGTs"),
        (16, "wpUmWi5rpGTsyPrP"),
        (32, "wpUmWi5rpGTsyPrPErnfB9JavNGdi4ym"),
        (57, "wpUmWi5rpGTsyPrPErnfB9JavNGdi4ymja5dD6jHTxuhAAAAAAAAAAAAA"),
    ];

    for (length, want) in cases {
        assert_eq!(GenerateString("id57:stable:v1", length).unwrap(), want);
    }
}

#[test]
fn generate_parity_and_charset() {
    let input = "string-bytes-parity";
    let banned = ['0', 'o', 'O', 'I', 'l'];
    let bytes = input.as_bytes();

    for length in [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57] {
        let from_bytes = Generate(bytes, length).unwrap();
        let from_string = GenerateString(input, length).unwrap();

        assert_eq!(from_bytes, from_string);
        assert!(Validate(&from_bytes, length).is_ok());
        assert!(IsValid(&from_bytes, length));
        assert!(!from_bytes.chars().any(|ch| banned.contains(&ch)));
    }
}

#[test]
fn from_digest_zero_is_all_a() {
    let zeros = [0u8; 32];

    for length in [2, 3, 4, 5, 6, 8, 10, 12, 16, 32, 57] {
        assert_eq!(FromDigest(&zeros, length).unwrap(), "A".repeat(length));
    }
}

#[test]
fn validate_errors() {
    assert!(Validate("A0BCDE", 6).is_err());
    assert!(Validate("ABCDE", 6).is_err());
    assert!(Validate("ABCDEFG", 7).is_err());
    assert!(Validate("AB", 2).is_ok());
}

#[test]
fn alphabet_has_expected_shape() {
    assert_eq!(Alphabet.len(), 57);
    for ch in ['0', 'o', 'O', 'I', 'l'] {
        assert!(!Alphabet.contains(ch));
    }
}
