use rand::prelude::SliceRandom;
use std::env;
/**
 * 尽可能随机返回URL
 */
pub fn get_random_net_url() -> String {
    let net_urls = env::var("NET_URL").expect("NET_URL must be set");

    let net_urls: Vec<&str> = net_urls.split(",").collect();

    // 确保 URL 列表不为空
    if net_urls.is_empty() {
        panic!("NET_URL environment variable is empty");
    }

    // 使用 rand crate 随机选择一个 URL
    let mut rng = rand::thread_rng();
    let random_url = net_urls
        .choose(&mut rng)
        .expect("Failed to choose a random URL");

    // 返回选中的 URL
    random_url.to_string()
}
