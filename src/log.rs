use chrono::Utc;
use eyre::Result;
use fern::Dispatch;
use std::env;
use std::result::Result::Ok;

pub fn init_log() -> Result<(), Box<dyn std::error::Error>> {
    let error_log_path = match env::var("ERROR_LOG_PATH") {
        Ok(l) => l,
        Err(_) => String::from("error.log"),
    };
    let app_log_path = match env::var("MAIN_LOG_PATH") {
        Ok(l) => l,
        Err(_) => String::from("app.log"),
    };

    // 配置App日志输出到文件
    let file_dispatch = Dispatch::new()
        .chain(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(app_log_path)?,
        )
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S"), // 时间戳
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info);

    // 设置错误日志输出格式
    let error_log_dispatch = Dispatch::new()
        .chain(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(error_log_path)?,
        )
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S"), // 时间戳
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Error);
    // 设置控制台输出
    let console_log_dispatch = Dispatch::new()
        .chain(std::io::stdout()) // 输出到控制台
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S"), // 时间戳
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info); // 控制台只打印 info 级别及以上日志
                                        // 配置日志输出到控制台和文件
    fern::Dispatch::new()
        .chain(console_log_dispatch) // 控制台输出
        .chain(file_dispatch) // 文件输出
        .chain(error_log_dispatch)
        .apply()?;

    Ok(())
}
