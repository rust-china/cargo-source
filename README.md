![Crates.io](https://img.shields.io/crates/v/cargo-source)
![Crates.io](https://img.shields.io/crates/d/cargo-source)

# cargo-source
`cargo-source` can make it easy and fast to switch between different crates registries.

## Install
```
$ cargo install cargo-source
```
## Example
```
$ cargo source list
Recommended registries：
  rsproxy    | https://rsproxy.cn/crates.io-index | 字节 
  ustc       | git://mirrors.ustc.edu.cn/crates.io-index | 中国科学技术大学 
  sjtu       | https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/ | 上海交通大学 
  tuna       | https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git | 清华大学 
  rustcc     | https://code.aliyun.com/rustcc/crates.io-index.git | rustcc社区
```

```
$ cargo source use ustc
Registry has been replaced with ustc
```
## Usage
```
Commands:
  list  List all the registries
  use   Change registry to registry
  add   Add one custom registry
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 贡献代码
### 本地运行
  cargo run [subcomd]
  例如：cargo run ls