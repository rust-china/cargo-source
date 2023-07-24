use ansi_term::Colour::Green;
use cargo_source::*;
use clap::{arg, Command};

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] != "source" {
        args.insert(1, "source".to_string())
    }

    let mut m = init_command();
    match m.try_get_matches_from_mut(args) {
        Ok(arg_matched) => match arg_matched.subcommand() {
            Some(("source", arg_matches)) => match arg_matches.subcommand() {
                Some(("list", _)) => ls(),
                Some(("use", sub_m)) => {
                    if let Some(c) = sub_m.get_one::<String>("source") {
                        switch(c);
                    }
                }
                Some(("add", sub_m)) => {
                    let registry_name = sub_m.get_one::<String>("name").unwrap();
                    let registry_url = sub_m.get_one::<String>("url").unwrap();
                    insert_registry(registry_name, registry_url);
                }
                _ => m.print_help().unwrap(),
            },
            _ => m.print_help().unwrap(),
        },
        _ => m.print_help().unwrap(),
    }
}

fn init_command() -> Command {
    Command::new("cargo")
        .about("crate 源切换工具")
        // .arg(
        //     Arg::new("in_file")
        // )
        .subcommand(
            Command::new("source").subcommands([
                Command::new("list").about("列出当前可用源").alias("ls"),
                Command::new("use")
                    .about("使用指定源")
                    .arg(arg!(<source> "前选择源名称").required(true)),
                Command::new("add")
                    .about("添加源")
                    .arg(arg!(<name> "源名称").required(true))
                    .arg(arg!(<url> "源地址").required(true)),
            ]),
        )
        .after_help(
            "Longer ======= explanation to appear after the options when \
                displaying the help information from --help or -h",
        )
    // .get_matches()
}

fn ls() {
    let cargo_config = CargoConfig::new().unwrap();
    println!("推荐源：");
    for (tag, url, desc) in REGISTRIES {
        println!(
            "  {} | {} | {} ",
            Green.paint(pad_end(tag, 10, ' ')),
            url,
            desc
        )
    }
    println!("\n-------------------------------------------\n");

    println!("本地配置源：");

    let replace_with = match cargo_config
        .document
        .as_table()
        .get("source")
        .unwrap()
        .get("crates-io")
        .unwrap()
        .get("replace-with")
    {
        Some(v) => v.as_str().unwrap(),
        None => "crates-io",
    };

    cargo_config.registries.iter().for_each(|(name, registry)| {
        let tag = if name == replace_with {
            format!("* {}", name)
        } else {
            format!("  {}", name)
        };
        println!(
            "{} | {}",
            pad_end(&tag, 12, ' '),
            registry.trim_matches('\"')
        )
    });
    println!("\n 说明：*表示当前使用的源，可以通过cargo source use xxxx 来切换源")
}

fn switch(registry: &str) {
    let cargo_config = CargoConfig::new();
    match cargo_config {
        Ok(mut result) => result.check_registry(registry),
        Err(err) => println!("{}", err),
    }
}

fn insert_registry(name: &str, url: &str) {
    let cargo_config = CargoConfig::new();
    match cargo_config {
        Ok(mut result) => {
            result.insert_registry(name, url);
            if let Err(err) = result.write_to_file() {
                println!("{err}")
            }
        }
        Err(err) => println!("{}", err),
    }
    println!("{name}, {url}")
}
