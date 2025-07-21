mod models;
mod banking;
mod ui;

use banking::service::BankingService;
use ui::{display, input};

fn main() {
    let mut banking_service = BankingService::new(1520.0); 

    loop {
        display::show_menu();
        let choice = input::get_input("Choose an option: ");

        match choice.as_str() {
            "1" => handle_create_account(&mut banking_service),
            "2" => handle_view_balance(&banking_service),
            "3" => handle_deposit(&mut banking_service),
            "4" => handle_withdraw(&mut banking_service),
            "5" => handle_transfer(&mut banking_service),
            "6" => handle_exchange_rate(&mut banking_service),
            "7" => handle_currency_conversion(&banking_service),
            "8" => {
                println!("👋 Thank you for banking with us!");
                break;
            }
            _ => println!("❌ Invalid option"),
        }
    }
}

fn handle_create_account(service: &mut BankingService) {
    let name = input::get_input("Enter account name: ");
    match service.create_account(name) {
        Ok(account) => {
            println!("✅ Account created successfully!");
            display::display_account(account);
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn handle_view_balance(service: &BankingService) {
    let account_number = input::get_input("Enter account number: ");
    match service.get_account(&account_number) {
        Ok(account) => display::display_account(account),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn handle_deposit(service: &mut BankingService) {
    let account_number = input::get_input("Enter account number: ");
    let amount = match input::get_number_input("Enter amount to deposit: ") {
        Ok(amt) => amt,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };
    let currency = match input::get_currency_input() {
        Ok(curr) => curr,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };

    match service.deposit(&account_number, amount, &currency) {
        Ok(balance) => {
            println!("✅ Deposit successful. New balance: {}{:.2}", currency.symbol(), balance);
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn handle_withdraw(service: &mut BankingService) {
    let account_number = input::get_input("Enter account number: ");
    let amount = match input::get_number_input("Enter amount to withdraw: ") {
        Ok(amt) => amt,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };
    let currency = match input::get_currency_input() {
        Ok(curr) => curr,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };

    match service.withdraw(&account_number, amount, &currency) {
        Ok(balance) => {
            println!("✅ Withdrawal successful. New balance: {}{:.2}", currency.symbol(), balance);
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn handle_transfer(service: &mut BankingService) {
    let from = input::get_input("Your account number: ");
    let to = input::get_input("Recipient account number: ");
    let is_same_bank = input::get_confirmation("Same bank? (y/n): ");
    
    let amount = match input::get_number_input("Amount: ") {
        Ok(a) => a,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };
    
    let currency = match input::get_currency_input() {
        Ok(c) => c,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };

    match service.transfer(&from, &to, amount, &currency, is_same_bank) {
        Ok(balance) => {
            println!("✅ Transfer successful. New balance: {}{:.2}", currency.symbol(), balance);
        }
        Err(e) => println!("❌ {}", e),
    }
}

fn handle_exchange_rate(service: &BankingService) {
    service.show_exchange_rate();
}

fn handle_currency_conversion(service: &BankingService) {
    let amount = match input::get_number_input("Enter amount to convert: ") {
        Ok(amt) => amt,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };

    println!("Convert from:");
    let from_currency = match input::get_currency_input() {
        Ok(curr) => curr,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };

    println!("Convert to:");
    let to_currency = match input::get_currency_input() {
        Ok(curr) => curr,
        Err(e) => {
            println!("❌ {}", e);
            return;
        }
    };

    let converted = service.convert_currency(amount, from_currency, to_currency);
    println!(
        "💱 {:.2} {} = {:.2} {} (Rate: 1 {} = {:.2} {})",
        from_currency.symbol(),
        amount,
        to_currency.symbol(),
        converted,
        from_currency.symbol(),
        service.exchange_rate,
        to_currency.symbol()
    );
}