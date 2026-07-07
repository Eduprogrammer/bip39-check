# bip39-check

[![Crates.io](https://img.shields.io/crates/v/bip39-check.svg)](https://crates.io/crates/bip39-check)
[![Docs.rs](https://docs.rs/bip39-check/badge.svg)](https://docs.rs/bip39-check)
[![CI](https://github.com/Eduprogrammer/bip39-check/actions/workflows/ci.yml/badge.svg)](https://github.com/Eduprogrammer/bip39-check/actions)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

EN · PT

---

Breve descrição
- Simple BIP-39 mnemonic validator: language detection, word count, checksum verification.
- Does NOT derive seeds or keys (intended to validate or inspect mnemonic strings only).

Por que usar
- Útil para onboarding, QA, suporte e SDKs: valida mnemonics sem derivar ou armazenar material sensível.
- Fornece relatório claro: idioma detectado, contagem de palavras e entropia estimada.

Features
- Detect language (English, Spanish, Portuguese, etc.)
- Word count checks (12/15/18/21/24)
- Checksum verification per BIP-39
- Unicode NFKD normalization
- Lightweight: no seed or key derivation

Instalação
Adicione no seu Cargo.toml:
```toml
[dependencies]
bip39-check = "0.1"
```

Usage - Quickstart (Rust)
```rust
use bip39_check::{validate, is_valid};

let mnemonic = "legal winner thank year wave sausage worth useful legal winner thank yellow";
assert!(is_valid(mnemonic));

let report = validate(mnemonic).unwrap();
println!(
    "language: {:?}, words: {}, entropy: {} bits",
    report.language,
    report.word_count,
    report.entropy_bits.unwrap_or(0)
);
```

Run tests / dev
```bash
# run tests
cargo test

# format and lint checks locally
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

API (resumo)
- is_valid(&str) -> bool
- validate(&str) -> Result<Report, Error)
Consulte docs.rs para a documentação completa: https://docs.rs/bip39-check

Examples
- Veja exemplos básicos na pasta `examples/` (se existir) e a documentação em docs.rs.

Contributing
- Abra issues para bugs ou feature requests.
- Fork -> branch -> commit -> PR.
- Rode `cargo fmt` e `cargo clippy` antes de abrir PR.
- Veja CONTRIBUTING.md para mais detalhes.

Security
- O pacote não deve ser usado para derivar ou armazenar seeds ou chaves.
- Reporte vulnerabilidades por issue privada ou contato no perfil.

License / Licença
- MIT OR Apache-2.0 (arquivo LICENSE será adicionado neste PR com MIT por padrão)

Contact
- Maintainer: Eduprogrammer (https://github.com/Eduprogrammer)

Changelog
- Veja CHANGELOG.md para histórico de versões.
