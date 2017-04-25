//! # Crypto util functions

use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};

/// Keccak-256 crypto hash length in bytes
pub const KECCAK256_BYTES: usize = 32;

/// Calculate Keccak-256 crypto hash
pub fn keccak256(data: &[u8]) -> [u8; KECCAK256_BYTES] {
    let mut sha3 = Sha3::new(Sha3Mode::Keccak256);
    sha3.input(data);

    let mut hash = [0u8; KECCAK256_BYTES];
    sha3.result(&mut hash);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_serialize::hex::FromHex;
    use util::*;

    macro_rules! bytes {
        ($hex: expr) => ({
            to_arr(&$hex.from_hex().unwrap())
        })
    }

    #[test]
    fn should_calculate_keccak256() {
        assert_eq!(keccak256(b"hello world!"),
                   bytes!("57caa176af1ac0433c5df30e8dabcd2ec1af1e92a26eced5f719b88458777cd6"));
    }
}
