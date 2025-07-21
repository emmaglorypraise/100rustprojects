use crate::models::{Account, Currency};

pub struct BankingService {
    accounts: Vec<Account>,
    next_id: usize,
    pub exchange_rate: f64, // 1 USD = X Naira
}

impl BankingService {
    pub fn new(exchange_rate: f64) -> Self {
        Self {
            accounts: Vec::new(),
            next_id: 1,
            exchange_rate,
        }
    }

    pub fn create_account(&mut self, name: String) -> Result<&Account, String> {
        let account = Account::new(self.next_id, name);
        self.accounts.push(account);
        self.next_id += 1;
        Ok(&self.accounts.last().unwrap())
    }

    pub fn get_account(&self, account_number: &str) -> Result<&Account, String> {
        self.accounts
            .iter()
            .find(|a| a.account_number == account_number)
            .ok_or("Account not found".to_string())
    }

    pub fn get_account_mut(&mut self, account_number: &str) -> Result<&mut Account, String> {
        self.accounts
            .iter_mut()
            .find(|a| a.account_number == account_number)
            .ok_or("Account not found".to_string())
    }

    pub fn deposit(
        &mut self,
        account_number: &str,
        amount: f64,
        currency: &Currency,
    ) -> Result<f64, String> {
        let account = self.get_account_mut(account_number)?;
        match currency {
            Currency::Naira => account.balance_naira += amount,
            Currency::Dollar => account.balance_dollar += amount,
        }
        Ok(self.get_balance(account_number, currency)?)
    }

    pub fn withdraw(
        &mut self,
        account_number: &str,
        amount: f64,
        currency: &Currency,
    ) -> Result<f64, String> {
        let account = self.get_account_mut(account_number)?;
        match currency {
            Currency::Naira => {
                if account.balance_naira < amount {
                    return Err("Insufficient Naira balance".to_string());
                }
                account.balance_naira -= amount;
            }
            Currency::Dollar => {
                if account.balance_dollar < amount {
                    return Err("Insufficient Dollar balance".to_string());
                }
                account.balance_dollar -= amount;
            }
        }
        Ok(self.get_balance(account_number, currency)?)
    }

    fn validate_transfer(
        &self,
        from: &str,
        to: &str,
        amount: f64,
        currency: &Currency,
        is_same_bank: bool,
    ) -> Result<(), String> {
        self.get_account(from)?;
        
        if !is_same_bank {
            return Err("❌ Outbound transfers not allowed".into());
        }
        
        self.get_account(to)?;
        
        let balance = self.get_balance(from, currency)?;
        if balance < amount {
            return Err(format!(
                "❌ Insufficient {} balance: {:.2}",
                currency.symbol(),
                balance
            ));
        }
        
        Ok(())
    }

    pub fn transfer(
    &mut self,
    from: &str,
    to: &str,
    amount: f64,
    currency: &Currency,
    is_same_bank: bool,
) -> Result<f64, String> {
    self.validate_transfer(from, to, amount, currency, is_same_bank)?;
    
    self.withdraw(from, amount, currency)?;
    self.deposit(to, amount, currency)?;
    
    self.get_balance(from, currency)
}

    pub fn get_balance(&self, account_number: &str, currency: &Currency) -> Result<f64, String> {
        let account = self.get_account(account_number)?;
        Ok(match currency {
            Currency::Naira => account.balance_naira,
            Currency::Dollar => account.balance_dollar,
        })
    }

    pub fn convert_currency(&self, amount: f64, from: Currency, to: Currency) -> f64 {
        match (from, to) {
            (Currency::Dollar, Currency::Naira) => amount * self.exchange_rate,
            (Currency::Naira, Currency::Dollar) => amount / self.exchange_rate,
            _ => amount, 
        }
    }

    pub fn show_exchange_rate(&self) {
        println!("Current exchange rate: 1 USD = ₦{:.2}", self.exchange_rate);
    }

}