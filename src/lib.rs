use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};

use bip39::{Mnemonic, Language, MnemonicType, Seed};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;

#[pyfunction]
pub fn bip39_to_mini_secret(phrase: &str, password: &str) -> PyResult<Vec<u8>> {
	let salt = format!("mnemonic{}", password);
	let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
	let mut result = [0u8; 64];

	pbkdf2::<Hmac<Sha512>>(mnemonic.entropy(), salt.as_bytes(), 2048, &mut result);

	Ok(result[..32].to_vec())
}

#[pyfunction]
pub fn bip39_generate(words: u32) -> PyResult<String> {

	let phrase = Mnemonic::new(MnemonicType::for_word_count(words as usize).unwrap(), Language::English).into_phrase();

	assert_eq!(phrase.split(" ").count(), words as usize);

	Ok(phrase.to_owned())
}

#[pyfunction]
pub fn bip39_to_seed(phrase: &str, password: &str) -> PyResult<Vec<u8>> {
	let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();

	Ok(Seed::new(&mnemonic, password)
		.as_bytes()[..32]
		.to_vec())
}

#[pyfunction]
pub fn bip39_validate(phrase: &str) -> bool {
	match Mnemonic::validate(phrase, Language::English) {
		Err(_) => false,
		_ => true
	}
}

#[pymodule]
fn bip39(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(bip39_to_mini_secret))?;
	m.add_wrapped(wrap_pyfunction!(bip39_generate))?;
	m.add_wrapped(wrap_pyfunction!(bip39_to_seed))?;
	m.add_wrapped(wrap_pyfunction!(bip39_validate))?;
    Ok(())
}
