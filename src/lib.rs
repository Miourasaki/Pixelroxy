use std::env;
use log::{Level};


pub mod config {
    pub struct Configure {
        pub host: String,
        pub port: u16,
    }

    impl Configure {
        pub fn get(args: Vec<String>) -> Self {

            let mut host: String = String::from("0.0.0.0");
            let mut port: u16 = 25565;

            for (i, arg) in args.iter().enumerate() {
                match arg.as_str() {
                    "-h" => host = args[i+1].clone(),
                    "-p" => port = args[i + 1].clone().parse().unwrap_or(7100),
                    _ => ()
                }
            }


            Configure {
                host,
                port,
            }
        }
    }



}


pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    env::set_var("RUST_LOG", "debug,trace");
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {


            writeln!(
                buf,
                "\u{1B}[0;{bg}m{} \u{1B}[{color}m[{}]\u{1B}[0;0m {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                &record.args(),
                bg = get_level_background(record.level()),
                color = get_level_color(record.level())
            )
        })
        .init();
    // error!("日志系统初始化完成.");
    // warn!("日志系统初始化完成.");
    // info!("日志系统初始化完成.");
    // debug!("日志系统初始化完成.");
    // trace!("日志系统初始化完成.");

}

fn get_level_color(level: Level) -> u8 {
    match level {
        Level::Error => 31,
        Level::Warn => 33,
        Level::Info => 35,
        Level::Debug => 31,
        Level::Trace => 34
    }
}
fn get_level_background(level: Level) -> u8 {
    match level {
        Level::Error => 41,
        Level::Warn => 43,
        Level::Info => 46,
        Level::Debug => 40,
        Level::Trace => 46
    }
}



pub mod server;
pub mod libs;
