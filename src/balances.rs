use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::system;

pub trait Config: system::Config {
    type Balance: Ord + Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::Account, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // Sets the account balance to amount
    pub fn set_balance(&mut self, account: &T::Account, amount: T::Balance) {
        self.balances.insert(account.clone(), amount);
    }

    // Gets the account balance
    // return zero if no balance is there
    pub fn get_balance(&self, account: &T::Account) -> T::Balance {
        *self.balances.get(account).unwrap_or(&T::Balance::zero())
    }

    // transfers the amount from 'from' account to 'to' account
    pub fn transfer(
        &mut self,
        from: &T::Account,
        to: &T::Account,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let from_balance = self.get_balance(from);

        let to_balance = self.get_balance(to);

        if from_balance < amount {
            return Err("Insufficient Balance");
        }

        let new_from_balance = from_balance - amount;

        let new_to_balance = to_balance.checked_add(&amount).ok_or("Balance Overflow")?;

        self.set_balance(from, new_from_balance);

        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

pub enum Call<T: Config> {
    Transfer{to: T::Account,amount: T::Balance}, 
}

impl <T:Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::Account;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(&caller, &to, amount)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::u128;

    use crate::system;

    struct TestConfig;

    impl system::Config for TestConfig {
        type Account = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances:super::Pallet<TestConfig> = super::Pallet::new();

        let account = String::from("alice");

        assert_eq!(balances.get_balance(&account), 0);
        balances.set_balance(&account, 100);

        assert_eq!(balances.get_balance(&account), 100);
    }

    #[test]
    fn check_transfer() {
        let mut balances:super::Pallet<TestConfig> = super::Pallet::new();

        let account1 = String::from("alice");

        let account2 = String::from("bob");

        assert_eq!(balances.get_balance(&account1), 0);

        assert_eq!(balances.get_balance(&account2), 0);

        balances.set_balance(&account1, 100);

        let _result = balances.transfer(&account1, &account2, 50);

        assert_eq!(balances.get_balance(&account1), 50);

        assert_eq!(balances.get_balance(&account2), 50);
    }

    #[test]
    fn insufficient_balance_transfer() {
        let mut balances:super::Pallet<TestConfig> = super::Pallet::new();

        let account1 = String::from("alice");

        let account2 = String::from("bob");

        assert_eq!(balances.get_balance(&account1), 0);

        assert_eq!(balances.get_balance(&account2), 0);

        balances.set_balance(&account1, 100);

        let result = balances.transfer(&account1, &account2, 120);

        assert_eq!(result, Err("Insufficient Balance"));

        assert_eq!(balances.get_balance(&account1), 100);

        assert_eq!(balances.get_balance(&account2), 0);
    }

    #[test]
    fn overflow_balance_transfer() {
        let mut balances:super::Pallet<TestConfig> = super::Pallet::new();

        let account1 = String::from("alice");

        let account2 = String::from("bob");

        assert_eq!(balances.get_balance(&account1), 0);

        assert_eq!(balances.get_balance(&account2), 0);

        balances.set_balance(&account1, 100);

        balances.set_balance(&account2, u128::MAX);

        let result = balances.transfer(&account1, &account2, 10);

        assert_eq!(result, Err("Balance Overflow"));

        assert_eq!(balances.get_balance(&account1), 100);

        assert_eq!(balances.get_balance(&account2), u128::MAX);
    }
}
