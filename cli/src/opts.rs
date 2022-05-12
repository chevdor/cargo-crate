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
}

/// The `info` command returns summarized information
#[derive(Parser, Debug)]
pub struct InfoOpts {
	/// One or more crate names
	#[clap(alias("name"), index = 1)]
	pub crate_name: Vec<String>,

	/// Do not show the details about all the older versions
	#[clap(short, long, alias("no_ver"))]
	pub no_versions: bool,
}

/// Opens the crate in a browser
#[derive(Parser, Debug)]
pub struct OpenOpts {
	/// The name of the crate to open in your browser
	#[clap(alias("name"), index = 1)]
	pub crate_name: String,
}
