
#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id, 
            holder,
            balance: 0,
        }
    }

    fn summary(&self) -> String {
        format!("Account ID: {}, Holder: {}, Balance: {}", self.id, self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }
}
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdraw(200).unwrap();

    println!("{}", account.summary());

    bank.add_account(account);

    println!("{:#?}", bank);
}
