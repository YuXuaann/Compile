use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use test::Test;
#[macro_use]
extern crate lazy_static;

mod grammar;
mod test;
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

fn test0() {
    let mut test = Test::new();
    test.add_rule('S', "S+T");
    test.add_rule('S', "T");
    test.add_rule('T', "T*F");
    test.add_rule('T', "F");
    test.add_rule('F', "(E)");
    test.special_add_rule('F', "id");
    test.set_start("S");
    test.show();
    // test.transfer_to_direct_left_recursion();
    // test.show();
    test.eliminate_left_recursion();
    test.show();

    // test.cal();
    // test.show();
}

fn test1() {
    let mut test = Test::new();
    test.add_rule('S', "Ac");
    test.add_rule('S', "c");
    test.add_rule('A', "Bb");
    test.add_rule('A', "b");
    test.add_rule('B', "Sa");
    test.add_rule('B', "a");
    test.set_start("S");
    test.show();
    test.transfer_to_direct_left_recursion();
    test.show();
    test.eliminate_left_recursion();
    test.show();
}

fn test2() {
    let mut test = Test::new();
    test.add_rule('S', "apple");
    test.add_rule('S', "apply");
    test.add_rule('S', "application");
    test.add_rule('S', "ball");
    test.add_rule('S', "bat");
    test.add_rule('S', "bath");
    test.add_rule('S', "Xb");
    test.add_rule('X', "ab");
    test.add_rule('X', "ac");
    test.add_rule('X', "ad");
    test.set_start("S");
    test.show();
    test.extract_common_left_factor();
    test.show();
}

fn test3() {
    let mut test = Test::new();
    test.add_rule('S', "AB");
    test.add_rule('A', "a");
    test.special_add_rule('A', "");
    test.add_rule('B', "b");
    test.set_start("S");
    test.show();
    test.show_cal();
}

fn test4() {
    let mut test = Test::new();
    test.add_rule('E', "E+T");
    test.add_rule('E', "T");
    test.add_rule('T', "T*F");
    test.add_rule('T', "F");
    test.add_rule('F', "(E)");
    test.special_add_rule('F', "id");
    test.set_start("E");
    test.show();
    // test.transfer_to_direct_left_recursion();
    // test.show();
    test.eliminate_left_recursion();
    test.show();

    test.show_cal();
}

fn main() {
    log_init();
    test0();
    test1();
    test2();
    test3();
    test4();
}
