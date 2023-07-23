use std::{fs::{self, File}, io::Write, error::Error};
use toml_edit::{Document, Item, Table, value};

pub const REGISTRIES: [(&str, &str, &str); 4] = [
    ("ustc", "git://mirrors.ustc.edu.cn/crates.io-index", "中国科学技术大学"),
    ("sjtu", "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/", "上海交通大学"),
    ("tuna", "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git", "清华大学"),
    ("rustcc", "https://code.aliyun.com/rustcc/crates.io-index.git", "rustcc社区"),
];

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
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let path = CargoConfig::get_config()?;
    let toml_str = fs::read_to_string(&path)?;
    let doc = toml_str.parse::<Document>()?;
    let source_option = doc.as_table().get("source");
    let mut registries: Vec<(String, String)> = Vec::new();
    if let Some(source) = source_option {
      source.as_table().unwrap().iter().for_each(|(key, val)| {
        registries.push((key.to_string(), val["registry"].as_str().unwrap().to_string()))
      });
    }
    Ok(CargoConfig {
      document: doc,
      registries,
      path
    })
  }

  pub fn check_registry(&mut self, registry: &str) {
    let in_local_config = self.registries.iter().any(|item| item.0 == registry);
    if in_local_config {
      let doc = &mut self.document;
      doc["source"]["crates-io"]["replace-with"] = value(registry);
      self.write_to_file().unwrap();
      println!("将crate源切换到: {}", registry);
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
      println!("将crate源切换到: {}", registry);
    } 
    println!("注册表中不存在：{}", registry);
  }

  pub fn write_to_file(&self) -> Result<(), Box<dyn Error>>{
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
  pub fn get_config() -> Result<String, Box<dyn Error>> {
    let mut result = String::from(""); 
    let dir = dirs::home_dir().ok_or("找不到主目录")?;
    let mut dir = dir.to_str().unwrap().to_string();
    dir.push_str("/.cargo/");
    let mut entries = fs::read_dir(&dir).expect("tttt");
    let exist = entries.any(|entry| {
      let file_name = entry.unwrap().file_name();
      if file_name.to_str().unwrap().contains("config") {
        dir.push_str(file_name.to_str().unwrap());
        true
      } else {
        false
      }
    });
    if exist {
      result.push_str(&dir)
    }
    Ok(result)
  }
}
