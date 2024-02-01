use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransactionSolPayload {
    pub sol_to_send: String,
    pub to_pubkey: String,
}