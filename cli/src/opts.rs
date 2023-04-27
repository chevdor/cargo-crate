use crate::crate_input::CrateInput;
use clap::{crate_authors, crate_version, Parser, Subcommand};

///
#[derive(Parser, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true)]
	pub json: bool,

	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand, Debug)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Open(OpenOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Search(SearchOpts),
}

/// The `info` command returns summarized information
#[derive(Parser, Debug)]
pub struct InfoOpts {
	/// One or more crate, passed as name or path
	#[clap(alias("name"), index = 1)]
	pub crate_names: Vec<CrateInput>,

	/// Limit the number of versions that are displayed. You can push the limit using this flag.
	#[clap(short, long, alias("max"), default_value("10"))]
	pub max_versions: u64,
}

/// Opens the crate in a browser
#[derive(Parser, Debug)]
pub struct OpenOpts {
	/// The name(s) of the crate to open in your browser
	#[clap(alias("name"), index = 1)]
	pub crate_names: Vec<CrateInput>,

	/// We open crates.io by default, use this flag to open the repo instead
	#[clap(long, alias("repo"))]
	pub repository: bool,

	/// We open crates.io by default, use this flag to open the homepage instead
	#[clap(long, alias("home"))]
	pub homepage: bool,

	/// We open crates.io by default, use this flag to open the documentation instead
	#[clap(long, alias("doc"))]
	pub documentation: bool,
}

fn valid_page_size(v: &str) -> Result<u64, String> {
	let i = v.parse::<u64>().expect("Failed parsing number");
	if i <= 100 {
		return Ok(i);
	}
	Err(String::from("The page size must be 0..100"))
}

/// Search crates.io and return a list of crates matching your search pattern
#[derive(Parser, Debug)]
pub struct SearchOpts {
	/// You search pattern
	#[clap(index = 1)]
	pub pattern: String,

	/// Number of expected results: 0..100
	#[clap(short, long, default_value("12"), value_parser = valid_page_size)]
	pub limit: u64,

	/// Show only the list of crates, without extra information
	#[clap(short, long)]
	pub raw: bool,
}
