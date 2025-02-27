pub mod contract;
pub mod log;
pub mod net_url;
pub mod write;

use crate::contract::get_balance_ustd;
use crate::write::{write_moneny_wallet, write_wallet};
use ::log::{error, info};
use alloy::primitives::utils::format_units;
use alloy::primitives::Uint;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::{coins_bip39::English, MnemonicBuilder};
use rand::{self, Rng};
use std::time::Duration;
use tokio::time::sleep;

// 助记词处理
pub async fn process_combination(combination: Vec<String>, url: &str) {
    let combined = combination.join(" ");

    let rpc_url = url.parse().unwrap();

    let provider = ProviderBuilder::new().on_http(rpc_url);
    let index = 0u32;
    let word = combined.as_str().trim();

    // 处理 MnemonicBuilder
    let builder = match MnemonicBuilder::<English>::default()
        .phrase(word)
        .index(index)
    {
        Ok(b) => b,
        Err(_) => {
            return; // 如果错误，跳过当前组合，继续处理下一个
        }
    };

    // 处理 `build` 返回的 `Result`
    let wallet = match builder.build() {
        Ok(w) => w,
        Err(_) => {
            return; // 如果错误，跳过当前组合，继续处理下一个
        }
    };

    let eth = match provider.get_balance(wallet.address()).await {
        Ok(b) => b,
        Err(_) => {
            // 生成一个 0 到 5 秒之间的随机延迟
            let mut rng = rand::thread_rng();
            // 0到5秒之间，步长为0.1
            let random_value = rng.gen_range(0..=50) as f64 * 0.1;
            // 将秒转换为毫秒
            let delay_duration = (random_value * 1000.0) as u64;
            sleep(Duration::from_millis(delay_duration)).await;
            match provider.get_balance(wallet.address()).await {
                Ok(b) => b,
                Err(e) => {
                    error!("{}", e);
                    // 可能是接口请求异常 跳过这次错误
                    return;
                }
            }
        }
    };
    let usdt = match get_balance_ustd(wallet.address(), provider).await {
        Ok(b) => b,
        Err(_) => Uint::from(0),
    };

    let usdt_str = format_units(usdt, 18).unwrap();
    let eth_str = format_units(eth, 18).unwrap();

    let eth: f64 = eth_str.parse().unwrap();
    let usdt: f64 = usdt_str.parse().unwrap();

    let mnemonic = format!("助力词:{}", word);
    let wallet_add = format!("钱包地址:{}", wallet.address());
    let balance = format!("余额: {:.6} ETH - {:.6} USDT", eth, usdt);

    if eth > 0.0 || usdt > 0.0 {
        write_moneny_wallet(mnemonic.as_str())
            .expect(format!("write message to file fail. message:{}", mnemonic).as_str());
        write_moneny_wallet(wallet_add.as_str())
            .expect(format!("write message to file fail. message:{}", wallet_add).as_str());
        write_moneny_wallet(balance.as_str())
            .expect(format!("write message to file fail. message:{}", balance).as_str());
    } else {
        write_wallet(mnemonic.as_str())
            .expect(format!("write message to file fail. message:{}", mnemonic).as_str());
        write_wallet(wallet_add.as_str())
            .expect(format!("write message to file fail. message:{}", wallet_add).as_str());
        write_wallet(balance.as_str())
            .expect(format!("write message to file fail. message:{}", balance).as_str());
    }

    info!("{}", mnemonic);
    info!("{}", wallet_add);
    info!("{}", balance);
}
