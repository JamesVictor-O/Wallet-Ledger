mod wallet;

use std::io::{self, Write};
use wallet::Wallet;

const WALLET_FILE: &str = "wallet.json";

fn main() {
    println!("=================================");
    println!("   WALLET LEDGER APPLICATION");
    println!("=================================\n");

    let mut wallet = load_or_create_wallet();

    loop {
        display_menu();

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => create_new_wallet(&mut wallet),
            "2" => credit_funds(&mut wallet),
            "3" => debit_funds(&mut wallet),
            "4" => view_balance(&wallet),
            "5" => {
                save_wallet(&wallet);
                println!("\nThank you for using Wallet Ledger. Goodbye!");
                break;
            }
            _ => println!("\n Invalid choice. Please try again.\n"),
        }
    }
}

fn display_menu() {
    println!("┌─────────────────────────────┐");
    println!("│         MAIN MENU           │");
    println!("├─────────────────────────────┤");
    println!("│ 1. Create New Wallet        │");
    println!("│ 2. Credit Funds             │");
    println!("│ 3. Debit Funds              │");
    println!("│ 4. View Balance             │");
    println!("│ 5. Save and Exit            │");
    println!("└─────────────────────────────┘\n");
}

fn create_new_wallet(wallet: &mut Wallet) {
    println!("\n--- Create New Wallet ---");

    let confirm = get_user_input("Are you sure? (yes/no): ");

    if confirm.trim().to_lowercase() == "yes" {
        let name = get_user_input("Enter new wallet name: ");
        *wallet = Wallet::new(name.trim().to_string());
        println!(" New wallet '{}' created successfully!\n", wallet.name);
    } else {
        println!("Operation cancelled.\n");
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn load_or_create_wallet() -> Wallet {
    match Wallet::load(WALLET_FILE) {
        Ok(wallet) => {
            println!(" Loaded existing wallet: {}\n", wallet.name);
            wallet
        }
        Err(_) => {
            println!("No existing wallet found. Creating a new wallet...\n");
            let name = get_user_input("Enter wallet name: ");
            Wallet::new(name.trim().to_string())
        }
    }
}

fn credit_funds(wallet: &mut Wallet) {
    println!("\n--- Credit Funds ---");

    let amount_str = get_user_input("Enter amount to credit: $");
    let amount: f64 = match amount_str.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid amount.\n");
            return;
        }
    };

    let description = get_user_input("Enter description: ");

    match wallet.credit(amount, description.trim().to_string()) {
        Ok(_) => {
            println!("Successfully credited ${:.2}", amount);
            println!("  New balance: ${:.2}\n", wallet.get_balance());
            save_wallet(wallet);
        }
        Err(e) => println!("Error: {}\n", e),
    }
}

fn debit_funds(wallet: &mut Wallet) {
    println!("\n--- Debit Funds ---");

    let amount_str = get_user_input("Enter amount to debit: $");
    let amount: f64 = match amount_str.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid amount.\n");
            return;
        }
    };

    let description = get_user_input("Enter description: ");

    match wallet.debit(amount, description.trim().to_string()) {
        Ok(_) => {
            println!("✓ Successfully debited ${:.2}", amount);
            println!("  New balance: ${:.2}\n", wallet.get_balance());
            save_wallet(wallet);
        }
        Err(e) => println!("Error: {}\n", e),
    }
}

fn view_balance(wallet: &Wallet) {
    println!("\n--- Current Balance ---");
    println!("Wallet: {}", wallet.name);
    println!("Balance: ${:.2}\n", wallet.get_balance());
}

fn save_wallet(wallet: &Wallet) {
    if let Err(e) = wallet.save(WALLET_FILE) {
        println!("Warning: Failed to save wallet: {}", e);
    }
}
