#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::env;
// use std::path::PathBuf;
use dirs;
use std::fs;
use toml::Value;
use clap::{ Command, Arg, arg };
use ansi_term::Colour::{Red, Green, Blue};

const REGISTRIES: [(&str, &str, &str); 4] = [
    ("ustc", "git://mirrors.ustc.edu.cn/crates.io-index", "中国科学技术大学"),
    ("sjtu", "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/", "上海交通大学"),
    ("tuna", "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git", "清华大学"),
    ("rustcc", "https://code.aliyun.com/rustcc/crates.io-index.git", "rustcc社区"),
];

fn main() {
    // let rustc_version = env::var("RUSTC_VERSION").unwrap_or_else(|_| String::from("Unknown"));
    // println!("Rustc version: {}", rustc_version);
    // if let Ok(config_path) = get_config() {
    //     println!("{}", config_path);
    //     parse_toml(&config_path[..]);
    // }
    
    let m = Command::new("mypro")
        .about("Explains in brief what the commond does")
        .arg(
            Arg::new("in_file")
        )
        .subcommands([
            Command::new("list")
                .about("列出当前可用源"),
            Command::new("use")
                .about("使用指定源")
                .arg(arg!(<label> "前选择源名称"))
        ])
        .after_help("Longer ======= explanation to appear after the options when \
                    displaying the help information from --help or -h")
        .get_matches();
    match m.subcommand() {
        Some(("list", _)) => {
            for (tag, url, desc) in REGISTRIES {
                println!("{} | {}【{}】 ", Green.paint(pad_end(tag, 8, ' ')), desc, url)
            }
            // println!("{:?}", REGISTRIES)
        },
        Some(("use", sub_m)) => {
            println!("use commond: {:?}", sub_m)
        },
        _ => {
            println!("none")
        }
    }
}

fn get_config() -> Result<String, String> {
    let mut result: Result<String, String> = Err(String::from("Failed to get config file")); 
    if let Some(mut dir) = dirs::home_dir() {
        dir.push(".cargo");
        let home_dir = dir.to_str().unwrap();
        if let Ok(entries) = fs::read_dir(home_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.contains("config") {
                            println!("the config file is {}", file_name);
                            dir.push(file_name);
                            result = Ok(String::from(dir.to_str().unwrap()));
                        }
                    }
                }
            };
        }
        // println!("Path to .cargo directory: {:?}", home_dir);
    } else {
        println!("Failed to get home directory");
    }
    result
}

fn parse_toml(path: &str) {

    let toml_str = fs::read_to_string(path).expect("Failed to read file");
    // 解析 TOML 配置文件
    let toml_value: Value = toml::from_str(&toml_str).expect("Failed to parse TOML");

    // 处理解析后的 TOML 数据
    if let Some(table) = toml_value.as_table() {
        // println!("{:#?}", table);
        if let Some(value) = table.get("source") {
            println!("Value of key: {:?}", value);
        }
    } else {
        println!("somethin is wrong")
    }
}

fn pad_end(input: &str, total_length: usize, padding_char: char) -> String {
    let input_length = input.chars().count();
    if input_length >= total_length {
        input.to_string()
    } else {
        let padding_length = total_length - input_length;
        let padding = padding_char.to_string().repeat(padding_length);
        input.to_string() + &padding
    }
}