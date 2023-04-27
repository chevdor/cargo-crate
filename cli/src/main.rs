mod crate_input;
mod opts;
mod target;
mod types;

use clap::{crate_name, crate_version, Parser};
use color_eyre::Result;
use env_logger::Env;
use lib_cargo_crate::*;
use opts::*;
use std::{collections::HashMap, ffi::OsString, ops::Deref, process};
use webbrowser::{Browser, BrowserOptions};

// use crate::crate_input::*;
use crate::target::*;
use crate::types::*;

/// Main entry point
fn main() -> Result<()> {
	color_eyre::install()?;

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

			// let the_crate = &open_opts.crate_name;
			let crate_names: Vec<CrateName> =
				open_opts.crate_names.iter().flat_map(|s| s.names().expect("Issue getting names")).collect();
			let the_crates: Vec<&str> = crate_names.iter().map(|x| x.deref()).collect();

			let options = lib_cargo_crate::InfoOpts::default();

			let info = Info::new();
			let fetched = info.fetch(the_crates, &options);
			let data = fetched.unwrap();
			let data = data.first();
			let data = data.unwrap();

			let urls: HashMap<CrateName, String> = crate_names
				.iter()
				.map(|c| {
					let url = match target {
						Target::CratesIo => format!("https://crates.io/crates/{c}"),
						Target::Repository => data.krate.crate_data.repository.as_ref().unwrap().to_string(),
						Target::Homepage => data.krate.crate_data.homepage.as_ref().unwrap().to_string(),
						Target::Documentation => data
							.krate
							.crate_data
							.documentation
							.as_ref()
							.unwrap_or(&format!("https://docs.rs/{}", &c))
							.to_string(),
					};
					(c.clone(), url)
				})
				.collect();

			if !urls.is_empty() {
				println!("Opening {} urls in your browser:", urls.len());
				urls.iter().for_each(|(name, url)| {
					println!(" - {target}: {name:<20} at {url}");
					let mut browser_options = BrowserOptions::new();
					browser_options.with_target_hint(name);
					webbrowser::open_browser_with_options(Browser::Default, url, &browser_options)
						.expect("Problem while opening default browser");
				});
			} else {
				eprintln!("No crates/url found");
				process::exit(1)
			}
		}

		SubCommand::Info(info_opts) => {
			log::debug!("Running command 'info'");
			let crate_names: Vec<CrateName> = info_opts
				.crate_names
				.iter()
				.flat_map(|s| {
					let names = s.names();

					if names.is_err() {
						eprintln!("Error: {}", &names.err().unwrap());
						process::exit(1)
					}
					names.unwrap()
				})
				.collect();
			let crates: Vec<&str> = crate_names.iter().map(|x| x.deref()).collect();

			let display_opts = lib_cargo_crate::InfoOpts { json: opts.json, max_versions: info_opts.max_versions };
			let response = Info::new().fetch(crates, &display_opts);

			if let Ok(info) = response {
				Info::show(info, &display_opts);
			} else {
				eprintln!("Error: {:?}", response.err().unwrap());
				process::exit(1)
			}
		}

		SubCommand::Search(search_opts) => {
			log::debug!("Running command 'search'");
			log::debug!("Searching for {:?}", search_opts);

			let hits = Info::new().search(&search_opts.pattern, search_opts.limit).unwrap();

			if search_opts.raw {
				hits.iter().for_each(|c| {
					println!("{}", c.name);
				})
			} else {
				hits.iter().for_each(|c| {
					let mut desc = c.description.as_ref().unwrap_or(&String::from("")).to_string();
					desc = desc.replace('\n', " ");
					const MAX: usize = 80;
					if desc.len() > MAX {
						desc.truncate(MAX - 1);
						desc = format!("{desc}…");
					};
					let dl = c.downloads.to_string();
					println!("{:<30} {:>12}\t{}", c.name, dl, desc);
				})
			}
		}
	}

	Ok(())
}
