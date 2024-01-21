use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {

    // 对 build 返回的 `Result` 进行处理
    // unwrap_or_else无错误则解析ok否则执行闭包中的函数
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // 错误输出到stderr，可以借此区分错误日志和正常日志（但一般没必要）
        process::exit(1); // 不使用panic
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}"); // 错误输出到stderr
        process::exit(1);
    }
}
