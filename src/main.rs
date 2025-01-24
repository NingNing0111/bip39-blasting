mod contract;
mod log;
mod mnemonic;
mod net_url;
mod write;

use crate::log::init_log;
use crate::mnemonic::mnemonic_init;
use crate::net_url::get_random_net_url;
use ::log::error;
use bip39_blasting::process_combination;
use dotenv::dotenv;
use eyre::Result;
use futures::stream::{self, StreamExt};
use itertools::Itertools;
use std::env;
use std::error::Error;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let _ = init_log().expect("log config error...");

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
            let url = get_random_net_url();
            process_combination(combination, url.as_str()).await;
        })
        .await;
    Ok(())
}
