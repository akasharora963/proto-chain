use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

pub trait Config {
    type Account: Ord + Clone;
    type BlockNumber: Zero + One + CheckedAdd + Copy;
    type Nonce: Zero + One + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::Account, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn increase_block_number(&mut self) -> Result<(), &'static str> {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .ok_or("Block Number Overflow")?;
        Ok(())
    }

    pub fn increase_nonce(&mut self, account: &T::Account) -> Result<(), &'static str> {
        let nonce = *self.nonce.get(account).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce.checked_add(&T::Nonce::one()).ok_or("Nonce Overflow")?;
        self.nonce.insert(account.clone(), new_nonce);
        Ok(())
    }

    pub fn get_nonce(&self, account: &T::Account) -> T::Nonce {
        *self.nonce.get(account).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod test {

    struct TestConfig;

    impl super::Config for TestConfig{
        type Account = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let system: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn increase_block_number() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();

        assert_eq!(system.block_number(), 0);

        let _result = system.increase_block_number();

        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn increase_nonce() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();

        let account = String::from("alice");

        let _result = system.increase_nonce(&account);

        assert_eq!(system.get_nonce(&account), 1);
    }
}
