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
    pub description: String,
    pub balance_after: f64,
}

pub struct Wallet {
    pub name: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

impl Wallet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            balance: 0.0,
            transactions: Vec::new(),
        }
    }

    pub fn credit(&mut self, amount: f64, description: String) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        };

        self.balance += amount;

        let transaction = Transaction {
            transaction_type: TransactionType::Credit,
            amount,
            timestamp: Utc::now(),
            description,
            balance_after: self.balance,
        };

        self.transactions.push(transaction);

        Ok(())
    }

    pub fn debit(&mut self, amount: f64, description: String) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        if amount > self.balance {
            return Err(format!(
                "Insufficient funds. Current balance: ${:.2}",
                self.balance
            ));
        }

        self.balance -= amount;

        let transaction = Transaction {
            transaction_type: TransactionType::Credit,
            amount,
            timestamp: Utc::now(),
            description,
            balance_after: self.balance,
        };

        self.transactions.push(transaction);

        Ok(())
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_transactions(&self) -> &Vec<Transaction>{
        &self.transactions
    }

    pub fn save(&self, filename: &str) -> io::Result<()>{
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }
    
    pub fn load(filename: &str) -> io::Result<Self> {
        let data = fs::read_to_string(filename)?;
        let wallet: Wallet = serde_json::from_str(&data)?;
        Ok(wallet)
    }


}
