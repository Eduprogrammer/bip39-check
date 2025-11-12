//! # bip39-check
//!
//! EN — Simple BIP-39 mnemonic validator: detects language, checks word count and checksum.
//! Does not derive seeds/keys. Useful for onboarding, QA, support and SDKs.
//!
//! PT — Validador simples de mnemonics BIP-39: detecta idioma, confere contagem e checksum.
//! Não deriva seed/chaves. Útil para onboarding, QA, suporte e SDKs.
//!
//! ## Example / Exemplo
//! ```rust
//! use bip39_check::{validate, is_valid};
//!
//! let ok = "legal winner thank year wave sausage worth useful legal winner thank yellow";
//! assert!(is_valid(ok));
//!
//! let report = validate(ok).unwrap();
//! println!("language: {:?}, words: {}, entropy: {} bits",
//!          report.language, report.word_count, report.entropy_bits.unwrap());
//! ```

use unicode_normalization::UnicodeNormalization;

/// EN — Supported BIP-39 wordlists (enable languages via `bip39` features).
/// PT — Wordlists BIP-39 suportadas (habilite idiomas via features do `bip39`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Spanish,
    French,
    Italian,
    Portuguese,
    Japanese,
    Korean,
    ChineseSimplified,
    ChineseTraditional,
    Czech,
}

impl Language {
    /// EN — All supported languages (mirrors enabled `bip39` features).
    /// PT — Todos os idiomas suportados (espelha as features habilitadas do `bip39`).
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Spanish,
            Language::French,
            Language::Italian,
            Language::Portuguese,
            Language::Japanese,
            Language::Korean,
            Language::ChineseSimplified,
            Language::ChineseTraditional,
            Language::Czech,
        ]
    }

    /// EN — Convert to `bip39::Language`.
    /// PT — Converte para `bip39::Language`.
    fn to_bip39(self) -> bip39::Language {
        match self {
            Language::English => bip39::Language::English,
            Language::Spanish => bip39::Language::Spanish,
            Language::French => bip39::Language::French,
            Language::Italian => bip39::Language::Italian,
            Language::Portuguese => bip39::Language::Portuguese,
            Language::Japanese => bip39::Language::Japanese,
            Language::Korean => bip39::Language::Korean,
            Language::ChineseSimplified => bip39::Language::SimplifiedChinese,
            Language::ChineseTraditional => bip39::Language::TraditionalChinese,
            Language::Czech => bip39::Language::Czech,
        }
    }
}

/// EN — Successful validation report.
/// PT — Relatório de validação bem-sucedida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// EN Detected language (if any). PT Idioma detectado (se houver).
    pub language: Option<Language>,
    /// EN Number of words. PT Número de palavras.
    pub word_count: usize,
    /// EN Entropy size in bits (128/160/192/224/256). PT Bits de entropia.
    pub entropy_bits: Option<usize>,
    /// EN NFKD-normalized phrase. PT Frase normalizada (NFKD).
    pub normalized: String,
}

/// EN — Validation error with kind and optional details.
/// PT — Erro de validação com tipo e detalhes opcionais.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckError {
    pub kind: ErrorKind,
    pub details: Option<String>,
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;
        match &self.kind {
            // Mantemos mensagens em PT — diretas e úteis para o usuário final.
            Empty => write!(f, "mnemonic vazio")?,
            InvalidWordCount { got } => write!(f, "contagem de palavras inválida: {}", got)?,
            UnknownOrMixedWords => write!(f, "palavras desconhecidas ou mistura de idiomas")?,
            BadChecksum => write!(f, "checksum inválido")?,
            UnicodeSuspicious => write!(f, "unicode suspeito após normalização")?,
        }
        if let Some(d) = &self.details {
            write!(f, " ({})", d)?;
        }
        Ok(())
    }
}

impl std::error::Error for CheckError {}

/// EN — Error categories returned by `validate`.
/// PT — Categorias de erro retornadas por `validate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Empty,
    InvalidWordCount { got: usize },
    UnknownOrMixedWords,
    BadChecksum,
    UnicodeSuspicious,
}

/// EN Validate a BIP-39 mnemonic.
/// - Normalizes to NFKD
/// - Checks word count {12,15,18,21,24}
/// - Tries enabled languages and verifies checksum
/// - Returns `Report` or `CheckError`
///
/// PT Valida um mnemonic BIP-39.
/// - Normaliza para NFKD
/// - Checa contagem {12,15,18,21,24}
/// - Tenta idiomas habilitados e verifica checksum
/// - Retorna `Report` ou `CheckError`
pub fn validate(phrase: &str) -> Result<Report, CheckError> {
    // 1) Normalize to NFKD / Normaliza para NFKD
    let normalized = phrase.nfkd().collect::<String>().trim().to_string();
    if normalized.is_empty() {
        return Err(CheckError { kind: ErrorKind::Empty, details: None });
    }

    // 2) Check word count / Checa contagem de palavras
    let words: Vec<&str> = normalized.split_whitespace().collect();
    let wc = words.len();
    const ALLOWED: [usize; 5] = [12, 15, 18, 21, 24];
    if !ALLOWED.contains(&wc) {
        return Err(CheckError {
            kind: ErrorKind::InvalidWordCount { got: wc },
            details: Some("válidos: 12, 15, 18, 21, 24".into()),
        });
    }

    // 3) Basic suspicious Unicode check / Checagem básica de Unicode suspeito
    if normalized.chars().any(|c| {
        (c as u32) < 0x20 || (0x202A..=0x202E).contains(&(c as u32))
    }) {
        return Err(CheckError { kind: ErrorKind::UnicodeSuspicious, details: None });
    }

    // 4) Try each language and verify checksum / Testa idiomas e verifica checksum
    let mut saw_checksum_err = false;
    for lang in Language::all() {
        match bip39::Mnemonic::parse_in_normalized(lang.to_bip39(), &normalized) {
            Ok(m) => {
                let entropy_bits = m.to_entropy().len() * 8;
                return Ok(Report {
                    language: Some(*lang),
                    word_count: wc,
                    entropy_bits: Some(entropy_bits),
                    normalized,
                });
            }
            Err(e) => {
                // Try to distinguish unknown words vs checksum error for better messages
                // Tenta diferenciar palavras desconhecidas de checksum ruim para mensagem melhor
                if e.to_string().to_lowercase().contains("checksum") {
                    saw_checksum_err = true;
                }
            }
        }
    }

    if saw_checksum_err {
        Err(CheckError { kind: ErrorKind::BadChecksum, details: None })
    } else {
        Err(CheckError { kind: ErrorKind::UnknownOrMixedWords, details: None })
    }
}

/// EN Fast boolean check using `validate`.
/// PT Verificação booleana rápida usando `validate`.
pub fn is_valid(phrase: &str) -> bool {
    validate(phrase).is_ok()
}
