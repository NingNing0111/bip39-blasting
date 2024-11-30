use eyre::Result;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::result::Result::Ok;

fn write_to_file(content: &str, path: String) -> Result<(), Box<dyn std::error::Error>> {
    // 打开文件并设置为追加模式
    let file = OpenOptions::new()
        .create(true) // 如果文件不存在，则创建文件
        .append(true) // 设置为追加模式
        .open(path)?;

    let mut writer = BufWriter::new(file);

    // 追加写入内容 换行写入
    writeln!(writer, "{}", content)?;

    // 确保数据被写入
    writer.flush()?;

    Ok(())
}
pub fn write_moneny_wallet(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result_out_path = match env::var("RESULT_OUT_PATH") {
        Ok(l) => l,
        Err(_) => String::from("resutl_wallet.txt"),
    };

    write_to_file(content, result_out_path)
}

pub fn write_wallet(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let valid_wallet = match env::var("VALID_WALLET") {
        Ok(l) => l,
        Err(_) => String::from("valid_wallet.txt"),
    };

    write_to_file(content, valid_wallet)
}
