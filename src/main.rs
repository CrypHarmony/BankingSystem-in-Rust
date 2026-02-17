// Banking System in Rust.

// Define a Trait for Account operations
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self);
}
// D// Banking System in Rust.

// Define a Trait for Account operations
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self);
}
// Define a Struct for BankAccount
struct BankAccount {
    account_number: i32,
    holder_name: String,
    balance: f64,
}
// Implement the Acount Trait for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!(" Account {} ({}):Deposited: {}. New balance: {}", amount,self.account_number , self.holder_name, self.balance);
    }
// Function to withdraw money from th account. It checks if the balance is sufficient before allowing the withdrawal.
    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!(" Account {} ({}):Withdrew: {}. New balance: {}", amount,  self.account_number, self.holder_name,self.balance);
        } else {
            println!("Account {} ({}):Insufficient funds. Current balance: {}",  self.account_number, self.holder_name, self.balance);
        }
    }

// Function to check the current balance of the account.
    fn balance(&mut self) {
        println!(" Account {} ({}):Current balance: {}",  self.account_number, self.holder_name,self.balance);
    }
}
// Main function to demonstrate the banking system. It creates two bank accounts, performs some deposits and withdrawals, and checks the balances.
    fn main () {
        let mut my_account1 = BankAccount {
            account_number: 123456,
            holder_name: String::from("Samuel"),
            balance : 1000.0,
        };
        my_account1.deposit(5000.0);
        my_account1.balance();

        let mut my_account2 = BankAccount {
            account_number: 654321,
            holder_name: String::from("Harmony"),
            balance : 2000.0,
        };

        my_account2.withdraw(1000.0);
        my_account2.balance();
        }

        

        // Key Concepts to notice:

        // 1. A Trait in Rust is similar to a contract/interface in other programming
        // languages. IT defines a set of methods that a type must implement to be considered as implementing that trait.
        // It defines what functions  must exist, but not how they work.
        // Meaning any type that implements Account have {deposit,withdraw,balance} methods.

        // &mut self: this function is allowed to modify the object.
        // Because deposit,withdraw  changes balance and balance functions may access internal data.

        //  the Struct BankAccount is our data model(object/structure). It repersents a real bank account with with field like 
        // account_number, holder_name and balance. It is the concrete implementation of the Account trait.

        // impl Account for BankAccount{:This means BankAccount follows the rules of Account trait.
        // So now BankAccount must implement: deposit, withdraw, and balance methods.

        //self.balance += amount; this line means adds money to the balance,self used to precise that we are in the current instance BankAccount
        // Add money to the account and show the new balance.

        // if self.balance >= amount{this checks if there is enough money to withdraw}
        // if there is enough money, it subtracts the amount from the balance and shows new the balance.
        // if not, it shows a message that there are insufficient funds.
        // it means, only allow withdrawal if balance is enough.

        //In  the main function, we create two bank accounts for Samuel and Harmony, perform some deposits and withdrawals,
        // and check their balances to demonstrate how the banking system works.
    

