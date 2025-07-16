use chrono::{Utc};
use serde::{Deserialize, Serialize};
use crate::rsa::{PublicKey, PrivateKey};
use crate::rsa;
use crate::sha;
use serde_json;
use std::convert::TryInto;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction{
    pub recipient_public_key: PublicKey, 
    pub value: u64,
    pub timestamp: chrono::DateTime<Utc>,
    pub sender_public_key: PublicKey,
    pub signature: Vec<u64>,
}

impl Transaction {
    pub fn new(
        recipient_public_key: PublicKey,
        sender_public_key: PublicKey,
        value: u64,
    ) -> Self{
        Transaction {
            recipient_public_key,
            value,
            timestamp: Utc::now(),
            sender_public_key,
            signature: Vec::new(),
        }
    }

    pub fn calculate_hash_transaction(&self) -> [u8; 32]{
        let sender_public_key_json = serde_json::to_string(&self.sender_public_key).expect("Erro ao serializar chave pública");
        let recipient_public_key_json = serde_json::to_string(&self.recipient_public_key).expect("Erro ao serializar chave pública do destinatário");
        
        let mut to_hash = format!(
            "{}{}{}{}",
            recipient_public_key_json,
            self.value,
            self.timestamp.to_rfc3339(),
            sender_public_key_json,
        );

        let bytes_hash = to_hash.as_bytes();
        sha::sha256(bytes_hash)

    }

    pub fn sign(&mut self, private_key: &PrivateKey){
        let hash_bytes = self.calculate_hash_transaction();

        let message_u64: Vec<u64> = hash_bytes.iter().map(|&b| b as u64).collect();

        let signature_vec_u64 = rsa::encrypt_string(&message_u64, private_key.d, private_key.n);

        self.signature = signature_vec_u64;
    }

    pub fn verify_signature(&self) -> bool {
    let original_hash_bytes = self.calculate_hash_transaction();
    let original_hash_u64: Vec<u64> = original_hash_bytes.iter().map(|&b| b as u64).collect();

    let verified_hash_u64 = rsa::encrypt_string(
        &self.signature,
        self.sender_public_key.e, // expoente público
        self.sender_public_key.n, // módulo
    );

    // --- LINHAS DE DEBUG CRUCIAIS ---
    println!("\n--- DEBUG: Verificando Assinatura ---");
    println!("  Remetente PK (JSON): {:?}", serde_json::to_string(&self.sender_public_key).unwrap_or_default());
    println!("  Hash Original (u64): {:?}", original_hash_u64);
    println!("  Assinatura (u64):    {:?}", self.signature);
    println!("  Hash Verificado (u64): {:?}", verified_hash_u64);
    let is_valid = original_hash_u64 == verified_hash_u64;
    println!("  Assinatura Válida? {}", is_valid);
    println!("--- FIM DEBUG ---\n");
    // --- FIM LINHAS DE DEBUG ---

    is_valid
}

}



