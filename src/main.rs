use ansi_term::Colour::Green;
use cargo_source::*;
use clap::{arg, Command};

fn main() -> anyhow::Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "source" {
        args.remove(1);
    }
    let mut cli = Cli::default();
    if let Err(err) = cli.run_command(args) {
        println!("{}", err.to_string());
    }
    Ok(())
}

struct Cli {
    command: Command,
}

impl Default for Cli {
    fn default() -> Self {
        let command = Command::new("cargo-source")
            .version("0.0.0")
            .about("crate 源切换工具")
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommands([
                Command::new("list").about("列出当前可用源").alias("ls"),
                Command::new("use")
                    .about("使用指定源")
                    .arg(arg!(<source> "前选择源名称").required(true)),
                Command::new("add")
                    .about("添加源")
                    .arg(arg!(<name> "源名称").required(true))
                    .arg(arg!(<url> "源地址").required(true)),
            ])
            .after_help(
                "Longer ======= explanation to appear after the options when \
                displaying the help information from --help or -h",
            );
        Self { command }
    }
}

impl Cli {
    fn run_command(&mut self, args: Vec<String>) -> anyhow::Result<()> {
        match self.command.try_get_matches_from_mut(args)?.subcommand() {
            Some(("list", _)) => self.ls()?,
            Some(("use", sub_m)) => {
                if let Some(c) = sub_m.get_one::<String>("source") {
                    self.switch(c);
                }
            }
            Some(("add", sub_m)) => {
                let registry_name = sub_m.get_one::<String>("name").unwrap();
                let registry_url = sub_m.get_one::<String>("url").unwrap();
                self.insert_registry(registry_name, registry_url);
            }
            _ => (),
        }
        Ok(())
    }
    fn ls(&self) -> anyhow::Result<()> {
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

        let replace_with = cargo_config
            .document
            .as_table()
            .get("source")
            .ok_or_else(|| anyhow::anyhow!("no source config"))?
            .get("crates-io")
            .ok_or_else(|| anyhow::anyhow!("no crates-io config"))?
            .get("replace-with")
            .ok_or_else(|| anyhow::anyhow!("no replace-with config"))?
            .as_str()
            .unwrap();

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
        println!("\n 说明：*表示当前使用的源，可以通过cargo source use xxxx 来切换源");

        Ok(())
    }

    fn switch(&self, registry: &str) {
        let cargo_config = CargoConfig::new();
        match cargo_config {
            Ok(mut result) => result.check_registry(registry),
            Err(err) => println!("{}", err),
        }
    }

    fn insert_registry(&self, name: &str, url: &str) {
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
}
