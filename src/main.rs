use ::log::{error, info};
use alloy::primitives::utils::format_units;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::{coins_bip39::English, MnemonicBuilder};
use dotenv::dotenv;
use eyre::Result;
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use log::init_log;
use mnemonic::mnemonic_init;
use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::Semaphore;
use write::{write_moneny_wallet, write_wallet};
mod log;
mod mnemonic;
mod write;

// 助记词处理
async fn process_combination(combination: Vec<String>, net_url: &str) {
    let combined = combination.join(" ");

    let rpc_url = net_url.parse().unwrap();

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

    let balance = match provider.get_balance(wallet.address()).await {
        Ok(b) => b,
        Err(_) => {
            sleep(Duration::from_millis(1500)); // 延迟重试
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

    let eth_str = format_units(balance, 18).unwrap();
    let eth: f64 = eth_str.parse().unwrap();

    let mnemonic = format!("助力词:{}", word);
    let wallet_add = format!("钱包地址:{}", wallet.address());
    let balance = format!("余额: {:.6} ETH", eth);

    if eth > 0.0 {
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

    info!("助记词:{}", word);
    info!("钱包地址:{}", wallet.address());
    info!("余额: {:.6} ETH", eth);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let _ = init_log().expect("log config error...");

    let net_url = env::var("NET_URL").expect("NET_URL must be set");
    // MAX_CONCURRENT
    let max_concurrent: usize = env::var("MAX_CONCURRENT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(|| {
            error!("Using default value for MAX_CONCURRENT 10 due to missing or invalid input.");
            10
        });
    let semaphore = Semaphore::new(max_concurrent);

    let word_list = mnemonic_init();

    let combinations = word_list.into_iter().combinations(12);

    // 并行执行
    stream::iter(combinations)
        .for_each_concurrent(max_concurrent, |combination| async {
            let _permit = semaphore.acquire().await.unwrap();
            process_combination(combination, &net_url).await;
        })
        .await;
    Ok(())
}
