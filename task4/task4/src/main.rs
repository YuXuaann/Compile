use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

mod grammar;
mod symbol;
mod sysy;
mod test;

fn log_init() {
    Builder::new()
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            let level = record.level();
            let color_code = match level {
                log::Level::Info => "\x1b[32m",  // 绿色
                log::Level::Warn => "\x1b[33m",  // 黄色
                log::Level::Error => "\x1b[31m", // 红色
                log::Level::Debug => "\x1b[34m", // 蓝色
                log::Level::Trace => "\x1b[35m", // 紫色
            };
            writeln!(buf, "{}{:>5}:\x1b[0m {}", color_code, level, record.args())
        })
        .init();
}

fn main() {
    log_init();
    test::test_main();
}
