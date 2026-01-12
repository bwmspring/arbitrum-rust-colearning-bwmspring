use alloy::{
    network::TransactionBuilder,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use std::str::FromStr;

/// 发送 ETH 转账交易到 Arbitrum 测试网
///
/// # 参数
/// * `from_private_key` - 发送者私钥（十六进制字符串）
/// * `to_address` - 接收者地址字符串
/// * `amount_eth` - 转账金额（ETH单位）
///
/// # 返回值
/// 返回交易哈希，如果转账失败则返回错误
pub async fn send_eth_transaction(
    from_private_key: &str,
    to_address: &str,
    amount_eth: f64,
) -> Result<String, Box<dyn std::error::Error>> {
    // 1. 地址校验
    let to_address = validate_address(to_address)?;

    // 2. 转换私钥为签名器
    let signer = PrivateKeySigner::from_str(from_private_key)?;

    // 3. 设置 RPC URL (Arbitrum Sepolia 测试网) 并配置签名器
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let wallet = alloy::signers::local::LocalSigner::from(signer.clone());
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .wallet(wallet)
        .connect_http(rpc_url.parse()?);

    // 4. 将 ETH 金额转换为 wei
    let amount_wei = eth_to_wei(amount_eth)?;

    // 5. 获取发送者地址
    let from_address = signer.address();

    // 6. 检查发送者余额
    check_sufficient_balance(&provider, from_address, amount_wei).await?;

    // 7. 获取 Gas 信息 (EIP-1559)
    let (max_fee_per_gas, max_priority_fee_per_gas, gas_limit) = get_gas_info(&provider).await?;

    // 8. 估算总费用
    let total_cost = amount_wei + U256::from(max_fee_per_gas) * U256::from(gas_limit);
    check_sufficient_balance(&provider, from_address, total_cost).await?;

    // 9. 获取 nonce
    let nonce = provider.get_transaction_count(from_address).await?;

    // 10. 获取链 ID
    let chain_id = provider.get_chain_id().await?;

    // 11. 构建交易（不设置 from，因为 signer 会自动设置）
    let tx = TransactionRequest::default()
        .with_to(Address(*to_address))
        .with_nonce(nonce)
        .with_chain_id(chain_id)
        .with_value(amount_wei)
        .with_max_fee_per_gas(max_fee_per_gas)
        .with_max_priority_fee_per_gas(max_priority_fee_per_gas)
        .with_gas_limit(gas_limit);

    // 12. 发送交易（provider 会自动使用配置的签名器签名）
    let pending_tx = provider.send_transaction(tx).await?;

    // 13. 获取交易哈希
    let tx_hash = format!("{:#x}", pending_tx.tx_hash());

    println!("Transaction sent successfully!");
    println!("Transaction hash: {}", tx_hash);

    Ok(tx_hash)
}

/// 校验地址格式
fn validate_address(address: &str) -> Result<Address, Box<dyn std::error::Error>> {
    let addr = Address::from_str(address)?;
    Ok(addr)
}

/// 将 ETH 转换为 wei
fn eth_to_wei(amount_eth: f64) -> Result<U256, Box<dyn std::error::Error>> {
    if amount_eth <= 0.0 {
        return Err("Amount must be positive".into());
    }

    // 1 ETH = 10^18 wei
    let _wei_per_eth = U256::from(1_000_000_000_000_000_000u128);
    let amount_wei = U256::from((amount_eth * 1_000_000_000_000_000_000.0) as u128);

    Ok(amount_wei)
}

/// 检查账户是否有足够的余额
async fn check_sufficient_balance(
    provider: &impl Provider,
    address: Address,
    required_amount: U256,
) -> Result<(), Box<dyn std::error::Error>> {
    let balance = provider.get_balance(address).await?;
    if balance < required_amount {
        return Err(format!(
            "Insufficient balance. Required: {} wei, Available: {} wei",
            required_amount, balance
        )
        .into());
    }
    Ok(())
}

/// 获取 Gas 信息
async fn get_gas_info(
    provider: &impl Provider,
) -> Result<(u128, u128, u64), Box<dyn std::error::Error>> {
    // 获取实时 Gas 价格
    let gas_price = provider.get_gas_price().await?;

    // 获取当前的区块信息以获取 base fee（用于 EIP-1559 计算）
    use alloy::eips::BlockId;
    use alloy::rpc::types::BlockNumberOrTag;

    let block_id = BlockId::Number(BlockNumberOrTag::Latest);
    let latest_block = provider.get_block(block_id).await?;
    let base_fee = latest_block
        .as_ref()
        .and_then(|block| block.header.base_fee_per_gas)
        .map(|fee| fee as u128)
        .unwrap_or(gas_price);

    // 设置 max_priority_fee_per_gas
    let max_priority_fee_per_gas = 2_000_000_000u128; // 2 gwei

    // 设置 max_fee_per_gas，确保不低于 base_fee + priority_fee，并设置最小阈值
    let min_max_fee = base_fee + max_priority_fee_per_gas;
    let max_fee_per_gas = std::cmp::max(20_000_000_000u128, std::cmp::max(gas_price, min_max_fee)); // 至少 20 gwei

    // 基础转账的 Gas 限额 (Arbitrum 网络需要更高的 gas limit)
    let gas_limit = 100_000;

    println!("Gas Price: {} wei", gas_price);
    println!("Base Fee: {} wei", base_fee);
    println!("Max Priority Fee Per Gas: {} wei", max_priority_fee_per_gas);
    println!("Max Fee Per Gas: {} wei", max_fee_per_gas);
    println!("Gas Limit: {}", gas_limit);

    Ok((max_fee_per_gas, max_priority_fee_per_gas, gas_limit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_address() {
        // 有效的地址
        let valid_address = "0xEFc67c7c50b5Ec45D76c63EeAb61b14E884E2Be4";
        assert!(validate_address(valid_address).is_ok());

        // 无效的地址
        let invalid_address = "0xinvalid";
        assert!(validate_address(invalid_address).is_err());
    }

    #[test]
    fn test_eth_to_wei() {
        // 1 ETH = 10^18 wei
        let amount_wei = eth_to_wei(1.0).unwrap();
        assert_eq!(amount_wei, U256::from(1_000_000_000_000_000_000u128));

        // 0.1 ETH = 10^17 wei
        let amount_wei = eth_to_wei(0.1).unwrap();
        assert_eq!(amount_wei, U256::from(100_000_000_000_000_000u128));

        // 无效金额
        assert!(eth_to_wei(0.0).is_err());
        assert!(eth_to_wei(-1.0).is_err());
    }
}
