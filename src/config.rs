use clap::Parser as _;
use once_cell::sync::Lazy;

pub static CFG: Lazy<Config> = Lazy::new(|| Config::parse());

#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
pub struct Config {
    #[clap(value_parser)]
    pub left_file: String,
    #[clap(value_parser)]
    pub right_file: String,
    #[clap(long, value_parser)]
    pub mode: String,
    #[clap(long)]
    pub remove_hashes: bool,
    #[clap(long, value_parser)]
    pub section_name: Option<String>,
    #[clap(long, value_parser)]
    pub left_block_index: Option<usize>,
    #[clap(long, value_parser)]
    pub right_block_index: Option<usize>,
    #[clap(long, value_parser)]
    pub summary_type: Option<String>,
    #[clap(long)]
    pub no_color: bool,
    #[clap(long, value_parser)]
    pub side_by_side_width: Option<usize>,
    #[clap(long)]
    pub only_diff: bool,
    #[clap(long)]
    pub only_adds: bool,
    #[clap(long)]
    pub only_dels: bool,
    #[clap(long)]
    pub only_dels_and_adds: bool,
}
