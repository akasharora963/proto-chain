use std::collections::BTreeMap;

type BlockNumber = u32;
type Nonce = u32;
type Account = String;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<Account, Nonce>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn increase_block_number(&mut self) -> Result<(), &'static str> {
        self.block_number = self
            .block_number
            .checked_add(1)
            .ok_or("Block Number Overflow")?;
        Ok(())
    }

    pub fn increase_nonce(&mut self, account: &Account) -> Result<(), &'static str> {
        let nonce = *self.nonce.get(account).unwrap_or(&0);
        let new_nonce = nonce.checked_add(1).ok_or("Nonce Overflow")?;
        self.nonce.insert(account.clone(), new_nonce);
        Ok(())
    }

    pub fn get_nonce(&self, account: &Account) -> Nonce {
        *self.nonce.get(account).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn init_system() {
        let system = super::Pallet::new();

        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn increase_block_number() {
        let mut system = super::Pallet::new();

        assert_eq!(system.block_number(), 0);

        let _result = system.increase_block_number();

        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn increase_nonce() {
        let mut system = super::Pallet::new();

        let account = String::from("alice");

        let _result = system.increase_nonce(&account);

        assert_eq!(system.get_nonce(&account), 1);
    }
}
