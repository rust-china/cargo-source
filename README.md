# cargo-source
`cargo-source` can make it easy easy and fast to switch between different crates registries.

## Install
```
$ cargo install cargo-source
```
## Example
```
$ cargo source list
推荐源：
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
  list  列出当前可用源
  use   使用指定源
  add   添加源
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
