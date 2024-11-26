use env_logger::Builder;
use log::{info, LevelFilter};
use rules::SYSY;
use std::io::Write;

mod grammar;
mod rules;
mod symbols;
mod trie;

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

    let mut sysy = SYSY.lock().unwrap();
    info!("SysY raw grammar:");
    sysy.show();

    sysy.extract_common_left_factor();
    info!("Extract common left factor succeed!");
    println!();
    sysy.transfer_to_direct_left_recursion();
    info!("Transfer to direct left recursion succeed!");
    println!();
    sysy.eliminate_left_recursion();
    info!("Eliminate left recursion succeed!");
    println!();

    sysy.show();

    sysy.show_cal();
    sysy.show_select();
    sysy.show_table();

    info!("SysY simplified grammar:");
    sysy.show();
    info!("SysY constructed successfully!");

    let path = "./SysY/test.sy";
    let result = sysy.identity(path);
    if result.is_ok() {
        println!("Identity success!");
    } else {
        println!("Error: {:?}", result.err().unwrap());
    }
}
