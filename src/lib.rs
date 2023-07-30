use std::{fs::{self, File}, io::Write, error::Error};
use toml_edit::{Document, Item, Table, value};
use ansi_term::Colour::Green;
use clap::{arg, Command};

pub type R<T> = Result<T, Box<dyn Error>>;

pub const REGISTRIES: [(&str, &str, &str); 5] = [
    ("rsproxy", "https://rsproxy.cn/crates.io-index", "字节"),
    ("ustc", "git://mirrors.ustc.edu.cn/crates.io-index", "中国科学技术大学"),
    ("sjtu", "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/", "上海交通大学"),
    ("tuna", "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git", "清华大学"),
    ("rustcc", "https://code.aliyun.com/rustcc/crates.io-index.git", "rustcc社区"),
];

const TMPL: &str = r#"
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
"#;
pub fn pad_end(input: &str, total_length: usize, padding_char: char) -> String {
    let input_length = input.chars().count();
    if input_length >= total_length {
        input.to_string()
    } else {
        let padding_length = total_length - input_length;
        let padding = padding_char.to_string().repeat(padding_length);
        input.to_string() + &padding
    }
}

pub struct CargoConfig {
    pub document: Document,
    pub registries: Vec<(String, String)>,
    path: String
}

impl CargoConfig {
    pub fn new() -> R<Self> {
        let mut path = CargoConfig::get_config().unwrap_or("".to_string());
        let mut toml_str = match path.len() {
            0 => {
                path = CargoConfig::gen_config_dir()?;
                TMPL.to_string()
            },
            _ => fs::read_to_string(&path)?
        };

        if toml_str.len() == 0 { toml_str = TMPL.to_string() }
        let doc = toml_str.parse::<Document>()?;

        let source_option = doc.as_table().get("source");
        let mut registries: Vec<(String, String)> = Vec::new();
        if let Some(source) = source_option {
            source.as_table().unwrap().iter().for_each(|(key, val)| {
                if let Some(v) = val.get("registry") {
                    registries.push((key.to_string(), v.to_string()))
                }
            });
        }
        Ok(
            CargoConfig {
                document: doc,
                registries,
                path
            }
        )
    }

    pub fn check_registry(&mut self, registry: &str) {
        if registry == "crates-io" {
            let doc = &mut self.document;
            let crates_io = doc["source"]["crates-io"].as_table_mut().unwrap();
            crates_io.remove("replace-with");
            self.write_to_file().unwrap();
            println!("Registry has been replaced with {}", registry);
            return
        }
        let in_local_config = self.registries.iter().any(|item| item.0 == registry);
        if in_local_config {
            let doc = &mut self.document;
            doc["source"]["crates-io"]["replace-with"] = value(registry);
            self.write_to_file().unwrap();
            println!("Registry has been replaced with {}", registry);
            return
        } 

        let mut url = String::from("");
        let in_recommendation_list = REGISTRIES.into_iter().any(|item| {
            if item.0 == registry {
                url.push_str(item.1);
                true
            } else {
                false
            }
        });
        if in_recommendation_list {
            self.insert_registry(registry, &url);
            let doc = &mut self.document;
            doc["source"]["crates-io"]["replace-with"] = value(registry);
            self.write_to_file().unwrap();
            println!("Registry has been replaced with {}", registry);
            return
        } 
        println!("there is no any registry named {} in recommendation list.", registry);
    }

    pub fn write_to_file(&self) -> R<()>{
        let updated_toml = self.document.to_string();
        let mut file = File::create(&self.path)?;
        file.write_all(updated_toml.as_bytes())?;
        Ok(())
    }

    pub fn insert_registry(&mut self, name: &str, url: &str) {
        let mut new_table = Table::new();
        new_table["registry"] = value(url);
        self.document["source"][name] = Item::Table(new_table);
    }
    
    pub fn get_config() -> R<String> {
        let dir = dirs::home_dir().ok_or("No home directory")?;
        let mut dir = dir.to_str().unwrap().to_string();
        dir.push_str("/.cargo/");
        let mut entries = fs::read_dir(&dir)?;
        let exist = entries.any(|entry| {
            let file_name = entry.unwrap().file_name();
            if file_name.to_str().unwrap().contains("config") {
                dir.push_str(file_name.to_str().unwrap());
                true
            } else {
                false
            }
        });
        if exist { Ok(dir) } else { Ok("".to_string())}
    }

    pub fn gen_config_dir() -> R<String> {
        let dir = dirs::home_dir().ok_or("No home directory")?;
        let mut dir = dir.to_str().unwrap().to_string();
        dir.push_str("/.cargo/config");
        Ok(dir)
    }

}

pub struct Cli {
    pub command: Command,
}

impl Default for Cli {
    fn default() -> Self {
        let command = Command::new("cargo-source")
            .version("0.0.31")
            .about("crates 源切换工具")
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
            ]);
            // .after_help(
            //     "If you find 【cargo-source】 is useful, or you are a experienced Rust developer, or you have the interest in the project, then welcome to submit PRs and help maintain 【cargo-source】. \n \
            //     ",
            // );
        Self { command }
    }
}

impl Cli {
    pub fn run_command(&mut self, args: Vec<String>) -> R<()> {
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
    fn ls(&self) -> R<()> {
        let cargo_config = CargoConfig::new().unwrap();
        println!("Recommended registries：");
        for (tag, url, desc) in REGISTRIES {
            println!(
                "  {} | {} | {} ",
                Green.paint(pad_end(tag, 10, ' ')),
                url,
                desc
            )
        }

        let default_registry = value("crates-io");
        let replace_with = cargo_config
            .document
            .as_table()
            .get("source")
            .ok_or("No source config")?
            .get("crates-io")
            .ok_or("No crates-io config")?
            .get("replace-with")
            .unwrap_or(&default_registry)
            .as_str()
            .unwrap();
        if cargo_config.registries.len() > 0 {
            println!("\n-------------------------------------------\nLocal config registries：");
        }
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


