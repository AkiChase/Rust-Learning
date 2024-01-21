use std::env;
use std::error::Error;
use std::fs;

// dyn Error是特征对象，Box是智能指针
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file_path)?;

    let res = if cfg.ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in res {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 因为返回值来自contents，生命周期一致

    // let mut res = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         res.push(line)
    //     }
    // }
    // res

    // 使用迭代器
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 因为返回值来自contents，生命周期一致

    let query = query.to_lowercase(); // 注意to_lowercase返回的是String类型，所以之后要使用引用&query

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // 传入的args为迭代器，没使用原来的std::env::Args，而是使用了特征约束的方式来描述
    // 返回Result
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok(); // 检查环境变量，有此变量则返回true

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// 测试驱动，测试先行（先写测试再写实现，再接入程序）

// 仅测试时才会编译（cfg还可以用于指定平台来编译）
#[cfg(test)]
mod test {
    use super::*; // 此use语句的作用域仅仅在test模块内

    // #[test] 是 Rust 中的测试属性
    #[test]
    fn one_test() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content))
    }
}
