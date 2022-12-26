use std::fs;
use ethers::{contract::Abigen};

fn main() -> eyre::Result<()> {
    let mut args = std::env::args();
    args.next().unwrap(); // skip program name

    let contract_name = args.next().unwrap_or_else(|| "ImageCell".to_owned());
    let solidity_abi_path = args.next().unwrap_or_else(|| "solidity_abi/ImageCell.abi.json".to_owned());

    let solidity_abi = fs::read_to_string(&solidity_abi_path).expect("file not found");

    let bindings = Abigen::new(&contract_name, solidity_abi)?.generate()?;
    bindings.write_to_file("rust_abi/abi.rs")?;

    Ok(())
}
