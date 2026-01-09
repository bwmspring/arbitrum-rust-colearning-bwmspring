mod balance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从命令行参数获取地址，如果没有提供则使用默认地址
    // let address = env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| "0xEFc67c7c50b5Ec45D76c63EeAb61b14E884E2Be4".to_string());

    let address = "0xEFc67c7c50b5Ec45D76c63EeAb61b14E884E2Be4";

    println!("查询地址 {} 在 Arbitrum 测试网上的 ETH 余额...", address);

    match balance::get_arbitrum_balance(&address).await {
        Ok(balance) => {
            println!("余额查询成功!");
            println!("address: {}", address);
            println!("balance: {:.6} ETH", balance);

            if balance == 0.0 {
                println!("该地址当前没有 ETH 余额");
            }
        }
        Err(e) => {
            println!("余额查询失败: {}", e);
        }
    }

    Ok(())
}
