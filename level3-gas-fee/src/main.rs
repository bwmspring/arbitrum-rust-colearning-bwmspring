mod gas_fee;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match gas_fee::get_gas_info().await {
        Ok((gas_price, gas_price_gwei, gas_limit, estimated_fee)) => {
            println!("Gas 价格: {} wei", gas_price);
            println!("Gas 价格: {} gwei", gas_price_gwei);
            println!("基础转账 Gas 限额: {} Gas", gas_limit);
            println!("预估转账 Gas 费用: {:.8} ETH", estimated_fee);
        }
        Err(e) => {
            println!("Gas 信息获取失败: {}", e);
        }
    }

    Ok(())
}
