pub mod secrets;
pub mod git;

pub use rops::file::{RopsFile, state::{EncryptedFile, DecryptedFile}, format::YamlFileFormat};
pub use rops::cryptography::{cipher::AES256GCM, hasher::SHA512};

pub type EncryptedRopsFile = RopsFile<EncryptedFile<AES256GCM, SHA512>, YamlFileFormat>;
pub type DecryptedRopsFile = RopsFile<DecryptedFile<SHA512>, YamlFileFormat>;
