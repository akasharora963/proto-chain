use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	/* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims : BTreeMap<T::Content,T::Account>
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            claims : BTreeMap::new()
        }
	}

	pub fn get_claim(&self,claim : &T::Content) -> Option<&T::Account> {
		self.claims.get(claim)
	}

	pub fn create_claim(&mut self,caller: T::Account,claim : T::Content) -> DispatchResult {
		match self.get_claim(&claim) {
			Some(_) => Err("Claim Already Exists"),
			None => {
				self.claims.insert(claim, caller);
				Ok(())
			}
		}
	}

	pub fn revoke_claim(&mut self,caller: T::Account,claim : T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("Claim not found")?;

		if owner != &caller {
			return Err("Caller is not the owner of claim");
		}

		self.claims.remove(&claim);

		Ok(())
	}
}

pub enum Call<T: Config> {
    CreateClaim {claim: T::Content},
	RevokeClaim {claim: T::Content} 
}

impl <T:Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::Account;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> crate::support::DispatchResult {
		match call {
			Call::CreateClaim { claim } => self.create_claim(caller, claim)?,
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim)?
		}
		Ok(())
	}
}


#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = String;
	}

	impl crate::system::Config for TestConfig {
		type Account = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
	
		let mut proof = super::Pallet::<TestConfig>::new();

		let account1 = String::from("alice");

		let account2: String = String::from("bob");


		let content1 = String::from("CEO");

		let content2 = String::from("CMO");


		let _ = proof.create_claim(account1.clone(),content1.clone());

		assert_eq!(proof.get_claim(&content1),Some(&account1));

		let res = proof.revoke_claim(account2.clone(), content1.clone());

		assert_eq!(res,Err("Caller is not the owner of claim"));

		let res = proof.create_claim(account2.clone(),content1.clone());

		assert_eq!(res,Err("Claim Already Exists"));

		let res = proof.revoke_claim(account2, content2);

		assert_eq!(res,Err("Claim not found"));

		let res = proof.revoke_claim(account1, content1.clone());

		assert_eq!(res,Ok(()));
		assert_eq!(proof.get_claim(&content1),None);


	}
}
