mod config;

use clap::Parser;
use crate::config::Config;
 
#[derive(Parser, Debug)]
#[clap(about = "Myceli, a spacey IPFS node")]
struct Args {
    /// Path to config file
    config_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let cfg: Config = Config::parse(args.config_path).expect("Config parsing failed");

    println!("cfg: {cfg:?}");
}