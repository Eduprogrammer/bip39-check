use bip39_check::{validate, is_valid, ErrorKind};

#[test]
fn english_valid_12_words() {
    let s = "legal winner thank year wave sausage worth useful legal winner thank yellow";
    let r = validate(s).expect("deveria ser válido");
    assert_eq!(r.word_count, 12);
    assert_eq!(r.entropy_bits, Some(128));
    assert!(r.language.is_some());
    assert!(is_valid(s));
}

#[test]
fn invalid_word_count() {
    let s = "legal winner thank";
    let err = validate(s).unwrap_err();
    match err.kind {
        ErrorKind::InvalidWordCount { got } => assert_eq!(got, 3),
        _ => panic!("esperava InvalidWordCount, veio {:?}", err.kind),
    }
}

#[test]
fn unknown_word() {
    // última palavra alterada para uma que não existe
    let s = "legal winner thank year wave sausage worth useful legal winner thank yellowz";
    let err = validate(s).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::UnknownOrMixedWords));
}

#[test]
fn bad_checksum_same_language_words() {
    // todas as palavras existem em inglês, mas a ordem foi trocada => checksum inválido
    let s = "legal winner thank year wave sausage worth useful legal winner yellow thank";
    let err = validate(s).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::BadChecksum));
}
