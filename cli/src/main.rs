mod opts;

use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
// use log::*;
use opts::*;

/// Main entry point
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();
	log::info!("Running {} v{}", crate_name!(), crate_version!());

	match opts.subcmd {
		SubCommand::Open(_get_opts) => {
			log::debug!("OPEN");
		}

		SubCommand::Info(_info_opts) => {
			log::debug!("INFO");
		}
	}

	Ok(())
}
