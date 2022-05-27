use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(flatten)]
    pub r#type: TransactionType,
    pub client: u16,
    pub tx: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum TransactionType {
    Deposit { amount: f64 },
    Withdrawal { amount: f64 },
    Dispute,
    Resolve,
    Chargeback,
}

impl TransactionType {
    pub fn amount(&self) -> &f64 {
        match &self {
            Self::Deposit { amount } => amount,
            _ => &0.0,
        }
    }
}
