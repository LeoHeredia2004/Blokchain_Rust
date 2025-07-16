use crate::rsa;
use crate::rsa::{PublicKey, PrivateKey};

#[derive(Debug)]
pub struct Wallet {
    pub name: String,
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

impl Wallet{
    pub fn new(name: String) -> Self{
        let (public_key, private_key) = rsa::generate_keypair();

        Wallet {
            name,
            public_key,
            private_key,
        }
    }
}