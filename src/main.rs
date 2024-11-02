mod balances;
mod system;

#[derive(Debug)]
pub struct RunTime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl RunTime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}
fn main() {
    let mut runtime = RunTime::new();

    let account1 = String::from("alice");

    let account2 = String::from("bob");

    let account3 = String::from("charlie");

    runtime.balances.set_balance(&account1, 100);

    let _ = runtime
        .system
        .increase_block_number()
        .map_err(|e| println!("Error : {:?}", e));

    let _ = runtime
        .system
        .increase_nonce(&account1)
        .map_err(|e| println!("Error : {:?}", e));

    assert_eq!(runtime.system.block_number(), 1);

    assert_eq!(runtime.system.get_nonce(&account1), 1);

    assert_eq!(runtime.balances.get_balance(&account1), 100);

    let _ = runtime
        .balances
        .transfer(&account1, &account2, 20)
        .map_err(|e| println!("Error : {:?}", e));

    let _ = runtime
        .system
        .increase_nonce(&account1)
        .map_err(|e| println!("Error : {:?}", e));

    assert_eq!(runtime.system.block_number(), 1);

    assert_eq!(runtime.system.get_nonce(&account1), 2);

    assert_eq!(runtime.balances.get_balance(&account1), 80);

    let _ = runtime
        .balances
        .transfer(&account1, &account3, 30)
        .map_err(|e| println!("Error : {:?}", e));

    assert_eq!(runtime.balances.get_balance(&account1), 50);

    println!("Program Running...");

    println!("{:#?}",runtime);
}
