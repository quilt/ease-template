// #![no_std]

extern crate alloc;

mod account;
mod address;
mod blob;
mod crypto;
mod transaction;

pub use account::{Account, RefAccount};
pub use address::Address;
pub use blob::RawBlob;
pub use transaction::{Transaction,SIGNATURE_LEN};
pub use crypto::PublicKey;

pub mod error {
    pub type Error = usize;
    pub const OK: usize = 0;
    pub const ERR: usize = 1;
}
