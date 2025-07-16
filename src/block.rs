use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use hex;

use crate::sha;
use crate::transaction::Transaction;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index:u32,
    pub timestamp:DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash:String,
    pub hash:String,
    pub nonce:u32
}


impl Block {

    pub fn new(
        index:u32,
        transactions: Vec<Transaction>,
        previous_hash:String
    ) -> Self{
        Block{
            index: index,
            timestamp: chrono::Utc::now(),
            transactions: transactions,
            previous_hash: previous_hash,
            hash: String::new(),
            nonce: 0
        }
    }


    pub fn calculate_hash(&self) -> String {
        let mut to_hash = format!(
            "{}{}{}{}",
            self.index,
            self.timestamp.to_rfc3339(),
            self.previous_hash,
            self.nonce
        );

        for tx in &self.transactions {
            let tx_json = serde_json::to_string(tx).expect("Erro ao serializar transação");
            to_hash.push_str(&tx_json);
        }

        let bytes_hash = to_hash.as_bytes();
        
        let hash = sha::sha256(bytes_hash);

        hex::encode(hash)
    }

}
