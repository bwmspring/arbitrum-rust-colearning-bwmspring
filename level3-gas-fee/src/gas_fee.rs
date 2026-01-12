use alloy::{
    primitives::utils::format_units,
    providers::{Provider, ProviderBuilder},
};

/// 获取 Arbitrum 测试网的实时 Gas 价格
///
/// # 返回值
/// 返回 Gas 价格（以 wei 为单位），如果查询失败则返回错误
pub async fn get_arbitrum_gas_price() -> Result<u128, Box<dyn std::error::Error>> {
    // Arbitrum Sepolia 测试网 RPC URL
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    // 创建 provider
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    // 获取当前 Gas 价格
    let wei_per_gas = provider.get_gas_price().await?;

    Ok(wei_per_gas)
}

/// 获取基础转账的 Gas 限额
///
/// # 返回值
/// 返回基础转账的 Gas 限额（使用行业标准值 21000）
/// 21,000 is for sending Ether, not anything else.
pub fn get_basic_transfer_gas_limit() -> u64 {
    21_000
}

/// 计算预估转账 Gas 费用
///
/// # 参数
/// * `gas_price` - Gas 价格（wei 单位）
/// * `gas_limit` - Gas 限额
///
/// # 返回值
/// 返回预估 Gas 费用（以 ETH 为单位）
pub fn calculate_gas_fee(gas_price: u128, gas_limit: u64) -> f64 {
    // Gas 费计算公式：Gas 费 = Gas 价格 × Gas 限额
    let gas_fee_wei = gas_price * gas_limit as u128;

    // 将 wei 转换为 ETH（1 ETH = 10^18 wei）
    let gas_fee_eth = gas_fee_wei as f64 / 1_000_000_000_000_000_000.0;

    return gas_fee_eth;
}

/// 获取完整的 Gas 信息和费用计算
///
/// # 返回值
/// 返回包含 Gas 价格、Gas 限额和预估费用的元组，如果查询失败则返回错误
pub async fn get_gas_info() -> Result<(u128, f64, u64, f64), Box<dyn std::error::Error>> {
    // 获取实时 Gas 价格
    let gas_price = get_arbitrum_gas_price().await?;

    // Convert the gas price to Gwei.
    let gas_price_gwei = format_units(gas_price, "gwei")?.parse::<f64>()?;

    // 获取基础转账 Gas 限额
    let gas_limit = get_basic_transfer_gas_limit();

    // 计算预估 Gas 费用
    let estimated_fee = calculate_gas_fee(gas_price, gas_limit);

    Ok((gas_price, gas_price_gwei, gas_limit, estimated_fee))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_arbitrum_gas_price() {
        match get_arbitrum_gas_price().await {
            Ok(gas_price) => {
                println!("Current gas price: {} wei", gas_price);
                assert!(gas_price > 0);
            }
            Err(e) => {
                println!("Error getting gas price: {}", e);
            }
        }
    }

    #[test]
    fn test_get_basic_transfer_gas_limit() {
        let gas_limit = get_basic_transfer_gas_limit();
        assert_eq!(gas_limit, 21_000);
    }

    #[test]
    fn test_calculate_gas_fee() {
        // 测试数据：假设 Gas 价格为 10 gwei，Gas 限额为 21000
        let gas_price = 10_000_000_000; // 10 gwei = 10^10 wei
        let gas_limit = 21_000;

        let fee = calculate_gas_fee(gas_price, gas_limit);

        // 预期结果：10^10 * 21000 / 10^18 = 2.1 * 10^-4 ETH = 0.00021 ETH
        let expected_fee = 0.00021;
        assert!((fee - expected_fee).abs() < 1e-10);

        println!("Gas fee calculation test passed: {} ETH", fee);
    }

    #[tokio::test]
    async fn test_get_gas_info() {
        match get_gas_info().await {
            Ok((gas_price, gas_price_gwei, gas_limit, estimated_fee)) => {
                println!("Gas Price: {} wei", gas_price);
                println!("Gas Price: {} gwei", gas_price_gwei);
                println!("Gas Limit: {}", gas_limit);
                println!("Estimated Fee: {} ETH", estimated_fee);

                assert!(gas_price > 0);
                assert!(gas_price_gwei > 0.0);
                assert_eq!(gas_limit, 21_000);
                assert!(estimated_fee > 0.0);
            }
            Err(e) => {
                println!("Error getting gas info: {}", e);
            }
        }
    }
}
