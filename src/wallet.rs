use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Credit,
    Debit,
}

pub struct Transaction {
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub timestamp: DataTime<Utc>,
    pub description : String,
    pub balance_after: f64
}

pub struct Wallet {
    pub name: String ,
    pub balance : f64,
    pub transactions : Vec<Transaction>,
}


impl  Wallet {
    pub fn new(name:String) -> Self{
        Self { name, balance: 0.0, transactions: Vec::new() }
    }

    pub fn credit(&mut self, amount:f64, description: String) -> Result<(), String>{
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
    }
}