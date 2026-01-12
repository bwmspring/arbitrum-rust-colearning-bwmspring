use dotenv::dotenv;
use std::env;

mod transaction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    println!("Arbitrum 测试网 ETH 转账脚本");

    // 从环境变量获取私钥
    let private_key =
        env::var("PRIVATE_KEY").map_err(|_| "请在 .env 文件中设置 PRIVATE_KEY 环境变量")?;

    // 验证私钥格式
    // Arbitrum Sepolia 测试网私钥可以不带 0x 前缀（64位）或带 0x 前缀（66位）
    let is_valid_private_key = if private_key.starts_with("0x") {
        private_key.len() == 66 // 0x + 64位
    } else {
        private_key.len() == 64 // 64位，不带前缀
    };

    if !is_valid_private_key {
        return Err("私钥格式错误，应为 64 位十六进制字符串，可选择是否带 0x 前缀".into());
    }

    // 获取接收地址（可以从环境变量或命令行参数获取）
    let to_address = env::var("TO_ADDRESS")
        .unwrap_or_else(|_| "0xdce8BfF7A85f70Bb8fE0d0F09DF434D972E3FDA5".to_string());

    // 获取转账金额（可以从环境变量或命令行参数获取）
    let amount_str = env::var("AMOUNT_ETH").unwrap_or_else(|_| "0.0001".to_string());

    let amount_eth: f64 = amount_str
        .parse()
        .map_err(|_| "转账金额格式错误，应为有效的数字")?;

    if amount_eth <= 0.0 {
        return Err("转账金额必须大于 0".into());
    }

    println!("Arbitrum Sepolia 测试网转账信息确认：");
    println!("   接收地址: {}", to_address);
    println!("   转账金额: {} ETH", amount_eth);
    println!();

    // 等待用户确认
    println!("按 Enter 键继续执行转账，或 Ctrl+C 取消...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // 执行转账
    println!("正在执行转账...");
    match transaction::send_eth_transaction(&private_key, &to_address, amount_eth).await {
        Ok(tx_hash) => {
            println!();
            println!("转账成功!");
            println!("   交易哈希: {}", tx_hash);
            println!("   可在 https://sepolia.arbiscan.io 中查看交易详情");
        }
        Err(e) => {
            println!();
            println!("转账失败：{}", e);
            return Err(e);
        }
    }

    Ok(())
}
