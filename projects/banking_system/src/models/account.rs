#[derive(Debug, Clone, Copy)] 
pub enum Currency {
    Naira,
    Dollar,
}

#[derive(Debug)]
pub struct Account {
    #[allow(dead_code)]
    pub id: usize,
    pub name: String,
    pub account_number: String,
    pub balance_naira: f64,
    pub balance_dollar: f64,
}

impl Account {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            account_number: format!("OCB{:08}", id),
            balance_naira: 0.0,
            balance_dollar: 0.0,
        }
    }
}

impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::Naira => "₦",
            Currency::Dollar => "$",
        }
    }
}