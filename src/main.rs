use support::Dispatch;

mod balances;
mod proof;
mod support;
mod system;

mod types {
    use crate::support;

    pub type Account = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Content = String;
    pub type Extrinsic = support::Extrinsic<Account, crate::RunTimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RunTimeCall {
    Balances(balances::Call<RunTime>),
    ProofOfExistence(proof::Call<RunTime>),
}

impl system::Config for RunTime {
    type Account = types::Account;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for RunTime {
    type Balance = types::Balance;
}

impl proof::Config for RunTime {
    type Content = types::Content;
}

impl Dispatch for RunTime {
    type Caller = <RunTime as system::Config>::Account;
    type Call = RunTimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    // Dispatch allows us to identify which underlying module call we want to execute.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RunTimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RunTimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RunTime {
    system: system::Pallet<RunTime>,
    balances: balances::Pallet<RunTime>,
    proof_of_existence: proof::Pallet<RunTime>,
}

impl RunTime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        let _ = self.system.increase_block_number();

        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (count, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate()
        {
            let _ = self.system.increase_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, count, e
                )
            });
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = RunTime::new();

    let account1 = String::from("alice");

    let account2 = String::from("bob");

    let account3 = String::from("charlie");

    let content1 = String::from("CEO");

    let content2 = String::from("CMO");


    runtime.balances.set_balance(&account1, 200);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: account1.clone(),
                call: RunTimeCall::Balances(balances::Call::Transfer {
                    to: account2.clone(),
                    amount: 69,
                }),
            },
            support::Extrinsic {
                caller: account1.clone(),
                call: RunTimeCall::Balances(balances::Call::Transfer {
                    to: account3.clone(),
                    amount: 11,
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("Invalid Block");

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: account1.clone(),
                call: RunTimeCall::ProofOfExistence(proof::Call::CreateClaim { claim: content1 }),
            },
            support::Extrinsic {
                caller: account2.clone(),
                call: RunTimeCall::ProofOfExistence(proof::Call::CreateClaim { claim: content2 }),
            }
        ],
    };

    runtime.execute_block(block_2).expect("Invalid Block");


    // let _ = runtime
    //     .system
    //     .increase_block_number()
    //     .map_err(|e| println!("Error : {:?}", e));

    // let _ = runtime
    //     .system
    //     .increase_nonce(&account1)
    //     .map_err(|e| println!("Error : {:?}", e));

    // assert_eq!(runtime.system.block_number(), 1);

    // assert_eq!(runtime.system.get_nonce(&account1), 1);

    // assert_eq!(runtime.balances.get_balance(&account1), 100);

    // let _ = runtime
    //     .balances
    //     .transfer(&account1, &account2, 20)
    //     .map_err(|e| println!("Error : {:?}", e));

    // let _ = runtime
    //     .system
    //     .increase_nonce(&account1)
    //     .map_err(|e| println!("Error : {:?}", e));

    // assert_eq!(runtime.system.block_number(), 1);

    // assert_eq!(runtime.system.get_nonce(&account1), 2);

    // assert_eq!(runtime.balances.get_balance(&account1), 80);

    // let _ = runtime
    //     .balances
    //     .transfer(&account1, &account3, 30)
    //     .map_err(|e| println!("Error : {:?}", e));

    // assert_eq!(runtime.balances.get_balance(&account1), 50);

    // println!("Program Running...");

    println!("{:#?}", runtime);
}
