mod contract;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Arbitrum 测试网合约交互脚本");

    // Arbitrum Sepolia 测试网 RPC URL
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    // 使用 ARB 测试代币合约地址
    let contract_address = "0xc275B23C035a9d4EC8867b47f55427E0bDCe14cB";

    println!("RPC URL: {}", rpc_url);
    println!("合约地址: {}", contract_address);
    println!();

    // 创建合约交互器
    println!("正在连接到 Arbitrum Sepolia 测试网...");
    let interactor = contract::ContractInteractor::new(rpc_url, contract_address).await?;

    // 显示合约基本信息
    match interactor.display_contract_info().await {
        Ok(_) => {
            println!("\n合约信息查询成功！");
        }
        Err(e) => {
            println!("\n合约信息查询失败：{}", e);
            return Err(e.into());
        }
    }

    // 可选：查询特定地址的代币余额
    println!("\n=== 额外功能：查询代币余额 ===");
    let test_addresses = vec![
        "0x0000000000000000000000000000000000000000", // 零地址
        "0xD4a0e0E8d51C4cDfC3B98993BB608CE6E0E19c24", // 测试地址
    ];

    for address in test_addresses {
        match interactor.get_balance(address).await {
            Ok(balance) => {
                println!(
                    "地址 {} 的代币余额: {} {}",
                    address,
                    balance,
                    interactor
                        .get_symbol()
                        .await
                        .unwrap_or_else(|_| "TOKEN".to_string())
                );
            }
            Err(e) => {
                println!("查询地址 {} 余额失败: {}", address, e);
            }
        }
    }

    Ok(())
}
