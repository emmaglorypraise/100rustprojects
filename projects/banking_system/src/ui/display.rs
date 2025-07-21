use crate::models::Account;

pub fn show_menu() {
    println!("\n🏦 Ocenic Bank Menu:");
    println!("1. Create Account");
    println!("2. View Balance");
    println!("3. Deposit");
    println!("4. Withdraw");
    println!("5. Transfer");
    println!("6. View Exchange Rate");
    println!("7. Currency Conversion");
    println!("8. Exit");
}

pub fn display_account(account: &Account) {
    println!("\nAccount Details:");
    println!("Account Number: {}", account.account_number);
    println!("Account Name: {}", account.name);
    println!("Naira Balance: ₦{:.2}", account.balance_naira);
    println!("Dollar Balance: ${:.2}", account.balance_dollar);
}