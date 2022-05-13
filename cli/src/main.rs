mod opts;

use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use lib_cargo_crate::*;
use opts::*;
use std::ffi::OsString;
use webbrowser::{Browser, BrowserOptions};

#[derive(Debug)]
enum Target {
	CratesIo,
	Repository,
	Homepage,
	Documentation,
}

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
			let target: Target = if open_opts.repository {
				Target::Repository
			} else if open_opts.homepage {
				Target::Homepage
			} else if open_opts.documentation {
				Target::Documentation
			} else {
				Target::CratesIo
			};

			let the_crate = &open_opts.crate_name;
			let the_crates = vec![the_crate.as_ref()];
			let options = lib_cargo_crate::InfoOpts::default();

			let info = Info::new();
			let fetched = info.fetch(the_crates, &options);
			let data = fetched.unwrap();
			let data = data.first();
			let data = data.unwrap();

			let url = match target {
				Target::CratesIo => format!("https://crates.io/crates/{}", the_crate),
				Target::Repository => data.krate.crate_data.repository.as_ref().unwrap().to_string(),
				Target::Homepage => data.krate.crate_data.homepage.as_ref().unwrap().to_string(),
				Target::Documentation => data
					.krate
					.crate_data
					.documentation
					.as_ref()
					.unwrap_or(&format!("https://docs.rs/{}", &the_crate))
					.to_string(),
			};

			log::debug!("Opening {:?} from {:?}", &target, &url);
			let mut browser_options = BrowserOptions::new();
			browser_options.with_target_hint(&open_opts.crate_name);
			webbrowser::open_browser_with_options(Browser::Default, &url, &browser_options)
				.expect("Problem while opening default browser");
		}

		SubCommand::Info(info_opts) => {
			log::debug!("Running command 'info'");
			let crates: Vec<&str> = info_opts.crate_name.iter().map(|s| s as &str).collect();

			let display_opts = lib_cargo_crate::InfoOpts { json: opts.json, max_versions: info_opts.max_versions };
			let data = Info::new().fetch(crates, &display_opts).unwrap();
			Info::show(data, &display_opts);
		}

		SubCommand::Search(search_opts) => {
			log::debug!("Running command 'search'");
			log::debug!("Searching for {:?}", search_opts);

			let hits = Info::new().search(&search_opts.pattern, search_opts.limit).unwrap();
			hits.iter().for_each(|c| {
				println!("{}", c.name);
			})
		}
	}

	Ok(())
}
