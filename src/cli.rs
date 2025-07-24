use std::path::PathBuf;

use amaru_kernel::network::NetworkName;
use clap::Parser;

use crate::config::{get_config_dir, get_data_dir};

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(short, long, env = "AMARU_NETWORK", default_value = "preprod")]
    pub network: NetworkName,

    #[arg(short, long, value_name = "FLOAT", env = "AMARU_LEDGER_DB")]
    pub ledger_db: Option<PathBuf>,

    #[arg(short, long, value_name = "FLOAT", env = "AMARU_CHAIN_DB")]
    pub chain_db: Option<PathBuf>,
}

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn version() -> String {
    let author = clap::crate_authors!();

    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{VERSION_MESSAGE}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}
