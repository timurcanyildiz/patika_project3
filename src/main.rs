pub trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self);

}

pub struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64
}

impl Account for BankAccount{
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("You deposited {}. New balance is: {}", amount,self.balance);
    }
    fn balance(&mut self) {
        println!("Current balance in account number: {} is: {}",self.account_number, self.balance);
    }
    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!(
                "Withdrawn {}. New balance is: {}",
                amount, self.balance
            );
        } else if amount > self.balance {
            println!("Insufficient funds. Cannot withdraw {}.", amount);
        }

    }
}

fn main() {
  
  let mut first_account = BankAccount{account_number:1015226,holder_name:String::from("Halil HASAR"),balance:4412.65};
  let mut second_account = BankAccount{account_number:3567881,holder_name:String::from("Timurcan YILDIZ"),balance:10455.20};

  first_account.balance();
  first_account.deposit(255.0);
  

  second_account.balance();
  second_account.withdraw(20000.25);
  second_account.withdraw(320.0);


}
