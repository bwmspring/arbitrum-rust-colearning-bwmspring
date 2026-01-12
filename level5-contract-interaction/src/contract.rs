use alloy::providers::{Provider, ProviderBuilder};
use alloy::sol;
use anyhow::Result;

// ERC20 ABI - 标准ERC20接口
sol! {
    #[sol(rpc)]
    contract ERC20 {
        function name() external view returns (string);
        function symbol() external view returns (string);
        function decimals() external view returns (uint8);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
    }
}

pub struct ContractInteractor {
    provider: alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy::providers::RootProvider,
    >,
    contract_address: alloy::primitives::Address,
}

impl ContractInteractor {
    /// 创建合约交互器
    pub async fn new(rpc_url: &str, contract_address: &str) -> Result<Self> {
        // 创建 HTTP 提供者
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

        // 解析合约地址
        let contract_address = contract_address.parse()?;

        Ok(Self {
            provider,
            contract_address,
        })
    }

    /// 获取代币名称
    pub async fn get_name(&self) -> Result<String> {
        let contract = ERC20::new(self.contract_address, &self.provider);
        let name = contract.name().call().await?;
        Ok(name)
    }

    /// 获取代币符号
    pub async fn get_symbol(&self) -> Result<String> {
        let contract = ERC20::new(self.contract_address, &self.provider);
        let symbol = contract.symbol().call().await?;
        Ok(symbol)
    }

    /// 获取代币小数位数
    pub async fn get_decimals(&self) -> Result<u8> {
        let contract = ERC20::new(self.contract_address, &self.provider);
        let decimals = contract.decimals().call().await?;
        Ok(decimals)
    }

    /// 获取总供应量
    pub async fn get_total_supply(&self) -> Result<String> {
        let contract = ERC20::new(self.contract_address, &self.provider);
        let total_supply: alloy::primitives::U256 = contract.totalSupply().call().await?;
        Ok(total_supply.to_string())
    }

    /// 获取账户余额
    pub async fn get_balance(&self, address: &str) -> Result<String> {
        let contract = ERC20::new(self.contract_address, &self.provider);
        let account: alloy::primitives::Address = address.parse()?;
        let balance: alloy::primitives::U256 = contract.balanceOf(account).call().await?;
        Ok(balance.to_string())
    }

    /// 获取当前区块号
    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self.provider.get_block_number().await?;
        Ok(block_number)
    }

    /// 显示合约基本信息
    pub async fn display_contract_info(&self) -> Result<()> {
        println!("=== 合约基本信息 ===");
        println!("合约地址: {}", self.contract_address);
        println!("当前区块号: {}", self.get_block_number().await?);

        println!("\n=== ERC20 代币信息 ===");
        println!("代币名称: {}", self.get_name().await?);
        println!("代币符号: {}", self.get_symbol().await?);
        println!("小数位数: {}", self.get_decimals().await?);
        println!("总供应量: {}", self.get_total_supply().await?);

        Ok(())
    }
}
