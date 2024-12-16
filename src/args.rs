use clap::{Parser, ValueHint};
use std::path::PathBuf;
use std::str::FromStr;

const DEFAULT_RESULT_PATH: &str = "./result.ass";

#[derive(Parser, Debug, Clone)]
pub struct NameToSsaPath {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Parser, Debug)]
pub struct AppArgs {
    /// Path to result subtitles file
    #[arg(short, default_value = DEFAULT_RESULT_PATH, value_hint = ValueHint::FilePath )]
    pub result_path: PathBuf,
    /// Path to chapters.xml file
    #[clap(value_hint = ValueHint::FilePath)]
    pub chapters_path: PathBuf,
    /// Path to main subtitle file
    #[clap(value_hint = ValueHint::FilePath)]
    pub subtitles_path: PathBuf,
    /// Space separated pairs of name=path-to-ssa-file
    #[clap(value_parser = parse_name)]
    pub names_to_ssa_pathes: Vec<NameToSsaPath>,
}

pub fn get_args() -> AppArgs {
    AppArgs::parse()
}

fn parse_name(line: &str) -> Result<NameToSsaPath, String> {
    let split = line
        .split_once("=")
        .ok_or_else(|| format!("Cant split pair {}", line))?;

    let path = PathBuf::from_str(split.1).map_err(|_| format!("Cant parse path {}", split.1))?;

    let res = NameToSsaPath {
        name: split.0.to_owned(),
        path,
    };

    Ok(res)
}
