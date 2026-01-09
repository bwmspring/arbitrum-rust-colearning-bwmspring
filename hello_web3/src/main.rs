use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use alloy::sol;
use std::error::Error;

sol! {
    #[sol(rpc)]
    interface HelloWeb3 {
        function hello_web3() pure public returns (string memory);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // let block_number = provider.get_block_number().await?;
    // println!("Hello Web3 block number: {}", block_number);
    // print!("Hello Web3");

    // Instantiate the contract instance.
    let weth = address!("0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90");
    let contract = HelloWeb3::new(weth, provider);

    let message = contract.hello_web3().call().await?;

    println!("Message: {:?}", message);

    Ok(())
}
