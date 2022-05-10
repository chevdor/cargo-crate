mod opts;

use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use lib_cargo_crate::*;
use opts::*;
use webbrowser::{Browser, BrowserOptions};

/// Main entry point
fn main() -> color_eyre::Result<()> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	let opts: Opts = Opts::parse();
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
			let display_opts = lib_cargo_crate::InfoOpts { json: opts.json };
			let data = Info::new().fetch(crates, &display_opts).unwrap();
			Info::show(data, &display_opts);
		}
	}

	Ok(())
}
