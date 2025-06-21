/*

    1.1 - BANK ACCOUNT

    Declare a public struct [BankAccount] containing public fields String [account_holder], f64 [balance], bool [locked],
    and f64 [interest_rate].

    Implement the following methods with the following signatures:

    pub fn new(holder: &str) -> Self;
    > Instansiate and return a new bank account with a specified account holder. The balance begins at a default of 0.00,
    > is not locked, and has a 1% interest rate.

    pub fn is_locked(&self) -> bool;
    > Returns whether the account is locked or not.

    pub fn lock(&mut self);
    > Locks the account

    pub fn balance_query(&self) -> f64;
    > Returns the current balance of the account.

    pub fn deposit(&mut self, amount: f64);
    > Adds [amount] to the current [balance] of the bank account.

    pub fn withdraw(&mut self, amount: f64) -> bool;
    > If [amount] is less than or equal to [balance], subtract the amount from the balance and return true. Otherwise,
    > return false.

    pub fn compound(&mut self);
    > Increase the account's [balance] by its [interest_rate]. For example, if there is a balance of P1,000 with an interest
    > rate of 1%, the new balance will be P1,010.

    pub fn decay(&mut self) -> f64;
    > Decrease the current [balance] by a fixed 10%, returning the amount that has been taken from the account.

    You may add extra fields or helper methods as you see fit.

*/

pub struct BankAccount {
    // add the fields here...
}

impl BankAccount {
    // add the methods here...
}

/*

    1.2 - BANK

    This time, create a struct for a [Bank] which manages and handles different [BankAccount]s. You must have a collection
    field for [accounts] which could either be a Vec<BankAccount> or HashMap<String, BankAccount>, depending on your preference. There
    must also be a f64 [treasury] field.

    Implement the following methods with the following signatures:

    pub fn new() -> Self;
    > Constructs a new bank with no existing accounts and an empty treasury.

    pub fn add_user(&mut self, user: BankAccount);
    > Adds a new [user] to the collection of users.

    pub fn handle_monthly_transactions(&mut self, transactions: Vec<(&str, f64)>);
    > Takes a vector of [transactions] that occur in a month. An account holder (&str) makes either a deposit or withdrawal from
    > their account (depending on whether the (f64) is positive or negative). If an account has not made a deposit in 18 months
    > or more, their account becomes locked. There is no mechanism to unlock accounts that have been locked.

    pub fn bi_annual_evaluation(&mut self);
    > Perform an action on each account. Compound any unlocked accounts and decay any locked accounts, feeding any decayed amounts
    > to the [treasury].

    pub fn generate_report(&self) -> Vec<(&str, f64)>;
    > Return a vector of tuples representing each account holder's name and their current balance.

    You may add extra fields or helper methods as you see fit.

*/

pub struct Bank {
    // add the fields here...
}

impl Bank {
    // add the methods here...
}