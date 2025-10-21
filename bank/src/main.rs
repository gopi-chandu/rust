use std::vec;

#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}
impl Account {
    fn new(id: u32, holder: String) -> Self {
        return Account { id, holder, balance: 0 };
    }
    fn withdraw(&mut self, amount: &i32) -> i32 {
        self.balance -= amount;
        return self.balance;
    }
    fn deposit(&mut self, amount: &i32) -> i32 {
        self.balance += amount;
        return self.balance;
    }
    fn summary(&self) -> String {
        return format!("{} as balance of {}", self.holder, self.balance);
    }
}
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}
impl Bank {
    fn new() -> Self {
        return Bank { accounts: vec![] };
    }
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
    fn total_balance(&self) -> i32 {
        let sum;

        sum = self.accounts
            .iter()
            .map(|account| account.balance)
            .sum();
        return sum;
    }
    fn summary(&self) -> Vec<String> {
        // let accounts = &self.accounts;
        // let mut summary: Vec<String> = vec![];
        // for account in accounts {
        //     summary.push(account.summary());
        // }
        // return summary;

        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}
fn print_account(account: Account) {
    println!("{:#?}", account);
}
fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Harry"));
    let a: i32 = 100;
    let b: i32 = 10;
    account.deposit(&a);
    account.withdraw(&b);
    bank.add_account(account);

    println!("{:#?}", bank);
    let statement = bank.accounts[0].summary();
    println!("{}", statement);

    println!("{}", bank.total_balance());
    println!("{:#?}", bank.summary());
}
