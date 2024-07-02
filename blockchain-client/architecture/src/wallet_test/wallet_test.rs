use crate::wallet::wallet::Wallet;

#[test]
fn test_print_wallet() {
    let wallet = Wallet::new();
    println!("{}", wallet);
}