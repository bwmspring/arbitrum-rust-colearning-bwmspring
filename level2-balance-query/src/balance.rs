use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use std::str::FromStr;

/// 查询 Arbitrum 测试网地址的 ETH 余额
///
/// # 参数
/// * `address` - 要查询的以太坊地址字符串
///
/// # 返回值
/// 返回以 ETH 为单位的余额，如果查询失败则返回错误
pub async fn get_arbitrum_balance(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Arbitrum Sepolia 测试网 RPC URL
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    // 创建 provider
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    // 解析地址
    let address = Address::from_str(address)?;

    // 查询余额（返回 wei 单位）
    let balance_wei = provider.get_balance(address).await?;

    // 将 wei 转换为 ETH（1 ETH = 10^18 wei）
    let balance_eth = f64::from(balance_wei) / 1_000_000_000_000_000_000.0;

    Ok(balance_eth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_arbitrum_balance() {
        let test_address = "0xEFc67c7c50b5Ec45D76c63EeAb61b14E884E2Be4";

        match get_arbitrum_balance(test_address).await {
            Ok(balance) => {
                println!("Address {} has balance: {} ETH", test_address, balance);
                assert!(balance >= 0.0);
            }
            Err(e) => {
                println!("Error querying balance: {}", e);
            }
        }
    }
}
