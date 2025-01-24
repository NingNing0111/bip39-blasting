use alloy::primitives::{Address, Uint};
use alloy::providers::RootProvider;
use alloy::sol;
use alloy::transports::http::{Client, Http};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "src/abi/IWETH9.json"
);
const USDT_ADDRESS: &str = "0xFB122130C4d28860dbC050A8e024A71a558eB0C1";
pub async fn get_balance_ustd(
    wallet_address: Address,
    provider: RootProvider<Http<Client>>,
) -> Result<Uint<256, 4>, Box<dyn std::error::Error>> {
    // Return Result
    let usdt_address: Address = USDT_ADDRESS
        .parse()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?; // Handle parse error

    let contract = IWETH9::new(usdt_address, provider);
    let usdt_balance = match contract.balanceOf(wallet_address).call().await {
        Ok(b) => b,
        Err(e) => {
            // Handle contract call errors
            return Err(Box::new(e) as Box<dyn std::error::Error>);
        }
    };
    Ok(usdt_balance._0) // Return result as Ok
}
