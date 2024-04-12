use std::{fs::File, io};

use crate::Result;

pub(crate) fn hash_blake3(mut file: File) -> Result<[u8; 32]> {
	let mut hasher = blake3::Hasher::new();
	io::copy(&mut file, &mut hasher)?;
	let result = hasher.finalize();
	Ok(result.as_bytes().to_owned())
}

pub(crate) fn hash_blake3_str(file: File) -> Result<Box<str>> {
	let hash = hash_blake3(file)?;
	Ok(hex::encode(hash).into_boxed_str())
}

// use sha1::{Digest, Sha1};
// pub(crate) fn verify_sha1(mut file: File, sha1: &[u8]) -> Result<bool> {
// 	let mut hasher = Sha1::new();
// 	io::copy(&mut file, &mut hasher)?;
// 	let result = &hasher.finalize()[..];
// 	Ok(sha1 == result)
// }

pub(crate) fn verify_blake3(file: File, blake: &[u8]) -> Result<bool> {
	let hash = hash_blake3(file)?;
	Ok(hash == blake)
}

pub(crate) fn verify_blake3_str(file: File, blake: &str) -> Result<bool> {
	verify_blake3(file, &hex::decode(blake)?)
}
