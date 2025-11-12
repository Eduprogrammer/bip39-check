use std::env;
use bip39_check::validate;

fn main() {
    let phrase = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if phrase.trim().is_empty() {
        eprintln!("uso: cargo run --example validate -- \"<mnemonic>\"");
        std::process::exit(2);
    }

    match validate(&phrase) {
        Ok(r) => {
            println!("✅ Válida!");
            println!("Idioma: {:?}\nPalavras: {}\nEntropia: {} bits\nNormalizada: {}",
                     r.language, r.word_count, r.entropy_bits.unwrap_or_default(), r.normalized);
        }
        Err(e) => {
            eprintln!("❌ Inválida: {e}");
            std::process::exit(1);
        }
    }
}
