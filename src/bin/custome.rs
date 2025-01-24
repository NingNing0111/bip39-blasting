use std::env;

use bip39_blasting::{log::init_log, net_url::get_random_net_url, process_combination};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _ = init_log().expect("log config error...");

    let custome_word = env::var("CUSTOME_WORDS").ok().unwrap();
    let words = custome_word.split(",").collect::<Vec<&str>>();
    // 转为Vec<String>类型
    let words = words.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    process_combination(words, get_random_net_url().as_str()).await;
}
