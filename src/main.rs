use clap::{AppSettings, Clap};

// 定义 HTTPie 的CLI的主入口，包含若干子命令
// 下面 /// 的注释是文档， calp 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how to use it?
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Nsoids" <msoids@mail.com>)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同 HTTP 方法， 目前只支持 get/post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    url: String,
}

#[derive(Clap, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn main () {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}