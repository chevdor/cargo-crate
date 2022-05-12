mod opts;

use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use lib_cargo_crate::*;
use opts::*;
use std::ffi::OsString;
use webbrowser::{Browser, BrowserOptions};

/// Main entry point
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let cmd_name = OsString::from(crate_name!().replace("cargo-", ""));
	// Filter the command out so we can use the command as cargo crate as well as cargo-crate
	let args = std::env::args_os().filter(|a| a != &cmd_name);
	let opts: Opts = Opts::parse_from(args);

	log::info!("Running {} v{}", crate_name!(), crate_version!());
	log::debug!("opts {:?}", opts);

	match opts.subcmd {
		SubCommand::Open(open_opts) => {
			log::debug!("Running command 'open' for {:?}", open_opts);
			let url = format!("https://crates.io/crates/{}", open_opts.crate_name);

			let mut browser_options = BrowserOptions::new();
			browser_options.with_target_hint(&open_opts.crate_name);
			webbrowser::open_browser_with_options(Browser::Default, &url, &browser_options)
				.expect("Problem while opening default browser");
		}

		SubCommand::Info(info_opts) => {
			log::debug!("Running command 'info'");
			let crates: Vec<String> = info_opts.crate_name;
			let display_opts = lib_cargo_crate::InfoOpts {
				json: opts.json,
				no_versions: info_opts.no_versions,
			};
			let data = Info::new().fetch(crates, &display_opts).unwrap();
			Info::show(data, &display_opts);
		}
	}

	Ok(())
}
