bip39-check

EN · PT below

Simple BIP-39 mnemonic validator: language detection, word count & checksum.
Does not derive seeds/keys.

Why

Great for onboarding, QA, support, and SDKs — validate without touching sensitive material.

Clear reports: detected language, word count, entropy bits.

Quickstart
use bip39_check::{validate, is_valid};

let ok = "legal winner thank year wave sausage worth useful legal winner thank yellow";
assert!(is_valid(ok));

let report = validate(ok).unwrap();
println!("language: {:?}, words: {}, entropy: {} bits",
         report.language, report.word_count, report.entropy_bits.unwrap());

Install
[dependencies]
bip39-check = "0.1"

License

MIT OR Apache-2.0

PT

Validador simples de mnemonics BIP-39: detecção de idioma, contagem e checksum.
Não deriva seed/chaves.

Por quê

Ótimo para onboarding, QA, suporte e SDKs — valida sem tocar em material sensível.

Relatório claro: idioma detectado, número de palavras, entropia (bits).

Uso rápido
use bip39_check::{validate, is_valid};

let ok = "legal winner thank year wave sausage worth useful legal winner thank yellow";
assert!(is_valid(ok));

let report = validate(ok).unwrap();
println!("idioma: {:?}, palavras: {}, entropia: {} bits",
         report.language, report.word_count, report.entropy_bits.unwrap());

Instalação
[dependencies]
bip39-check = "0.1"

Licença

MIT OR Apache-2.0