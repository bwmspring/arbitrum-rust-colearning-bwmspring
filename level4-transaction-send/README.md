# Level 4: Arbitrum 测试网 ETH 转账

基于 alloy 编写的完整 ETH 转账脚本，实现从 A 地址到 B 地址的 Arbitrum Sepolia 测试网 ETH 转账。

## 功能特性

- ✅ **地址校验**：验证接收者地址格式正确性
- ✅ **Gas 费设置**：自动获取实时 Gas 价格并设置合适限额
- ✅ **交易签名**：使用私钥安全签名交易
- ✅ **交易发送**：发送交易到 Arbitrum 测试网
- ✅ **交易哈希获取**：返回交易哈希用于查询
- ✅ **私钥安全**：通过环境变量管理，不在代码中硬编码
- ✅ **余额检查**：确保发送者有足够的 ETH 支付转账和 Gas 费

## 环境要求

- Rust 1.70+
- 互联网连接（访问 Arbitrum Sepolia 测试网）

## 安装和配置

1. **克隆项目并进入目录**：
   ```bash
   cd level4-transaction-send
   ```

2. **安装依赖**：
   ```bash
   cargo build
   ```

3. **配置环境变量**：

   复制环境变量示例文件：
   ```bash
   cp env-example.txt .env
   ```

   编辑 `.env` 文件，设置你的私钥和其他参数：
   ```env
   # 必需：发送者私钥（测试网专用，不要使用主网私钥）
   PRIVATE_KEY=0x你的私钥

   # 可选：接收者地址（默认为测试地址）
   TO_ADDRESS=0x接收者地址

   # 可选：转账金额（ETH，默认为 0.001）
   AMOUNT_ETH=0.001
   ```

   **安全提醒**：
   - 私钥格式：`0x` 开头的 64 位十六进制字符串
   - 仅使用测试网私钥，不要使用主网私钥
   - 确保 `.env` 文件不会被提交到版本控制

## 使用方法

1. **运行转账脚本**：
   ```bash
   cargo run
   ```

2. **按照提示操作**：
   - 脚本会显示转账信息供确认
   - 按 Enter 键继续，或 Ctrl+C 取消

3. **查看结果**：
   - 成功时显示交易哈希
   - 可在 [Arbitrum Sepolia 区块浏览器](https://sepolia.arbiscan.io/) 查看交易详情

## 代码结构

```
src/
├── main.rs          # 主程序入口，环境变量处理和用户交互
└── transaction.rs   # 转账核心逻辑，包含地址校验、Gas设置、签名发送
```

## 核心函数

### `send_eth_transaction`
执行完整的 ETH 转账流程：

```rust
pub async fn send_eth_transaction(
    from_private_key: &str,
    to_address: &str,
    amount_eth: f64,
) -> Result<String, Box<dyn std::error::Error>>
```

**参数**：
- `from_private_key`: 发送者私钥
- `to_address`: 接收者地址
- `amount_eth`: 转账金额（ETH）

**返回值**：交易哈希字符串

### 辅助函数

- `validate_address()`: 校验地址格式
- `eth_to_wei()`: ETH 到 wei 单位转换
- `check_sufficient_balance()`: 检查账户余额
- `get_gas_info()`: 获取 Gas 价格和限额

## 测试

运行单元测试：
```bash
cargo test
```

## 安全注意事项

1. **私钥保护**：
   - 不要在代码中硬编码私钥
   - 使用环境变量管理私钥
   - 仅在测试网使用测试私钥

2. **网络安全**：
   - 使用 HTTPS RPC 端点
   - 验证所有地址和金额

3. **资金安全**：
   - 确认接收地址正确
   - 检查账户余额充足
   - 这是测试网，不会损失真实资金

## 故障排除

**常见错误**：
- `私钥格式错误`：确保私钥为 64 位十六进制字符串，以 `0x` 开头
- `Insufficient balance`：账户余额不足以支付转账金额 + Gas 费
- `地址格式错误`：确保接收地址为有效的以太坊地址格式

**获取测试网 ETH**：
访问 [Arbitrum Sepolia 水龙头](https://sepoliafaucet.com/) 获取测试 ETH。

## 技术栈

- **alloy**: 以太坊交互库
- **tokio**: 异步运行时
- **dotenv**: 环境变量管理
- **Arbitrum Sepolia**: 测试网络