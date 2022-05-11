use std::path::PathBuf;

use clap::Parser;

use firefox_cookie::*;

/// Firefox の cookies.sqlite を cookies.txt に変換する。
#[derive(Debug, Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path_cookies: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cookies = read_cookies_db(cli.path_cookies)?;

    // コメント行がないとパースに失敗するツールがあるとのこと。
    // 念のため Firefox の cookies.txt アドオンと形式を合わせておく。
    // ref: https://github.com/lennonhill/cookies-txt
    println!("# Netscape HTTP Cookie File");
    println!("# https://curl.haxx.se/rfc/cookie_spec.html");
    println!("# This is a generated file! Do not edit.");
    println!();

    for cookie in &cookies {
        println!("{cookie}");
    }

    Ok(())
}
