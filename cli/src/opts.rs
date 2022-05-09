use clap::{crate_authors, crate_version, Parser, Subcommand};

///
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Output as json
	#[clap(short, long, global = true)]
	pub json: bool,

	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	Info(InfoOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Open(OpenOpts),
}

/// The `info` command returns summarized information
#[derive(Parser)]
pub struct InfoOpts {
	/// todo
	#[clap(alias("name"), index = 1)]
	pub crate_name: String,
}

/// Opens the crate in a browser
#[derive(Parser)]
pub struct OpenOpts {
	/// todo
	#[clap(alias("name"), index = 1)]
	pub crate_name: String,
}
