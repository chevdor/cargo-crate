use crate::api_reponse::*;
use crate::info_opts::InfoOpts;
use crate::WrappedVersion;
use chrono_humanize::HumanTime;
use crates_io_api::{Crate, CratesQuery, Error, Sort, SyncClient};
use std::time::Duration;

pub struct Info {
	client: SyncClient,
}

impl Default for Info {
	fn default() -> Self {
		Info::new()
	}
}

impl Info {
	pub fn new() -> Self {
		let client = SyncClient::new("cargo-crate", Duration::from_millis(10)).expect("failed getting client");

		Self { client }
	}

	/// Fetch all the information about the crate
	pub fn fetch(&self, crates: Vec<&str>, _opts: &InfoOpts) -> anyhow::Result<Vec<ApiResponse>> {
		// TODO: check out full_crate
		Ok(crates
			.iter()
			.map(|krate| {
				let response: Result<ApiResponse, Error> =
					Ok(ApiResponse { krate: self.client.get_crate(krate)?, owners: self.client.crate_owners(krate)? });
				response
			})
			.collect::<Result<Vec<_>, _>>()?)
	}

	pub fn show(response: Vec<ApiResponse>, opts: &InfoOpts) {
		if opts.json {
			Info::show_json(&response);
		} else {
			Info::show_txt(&response, Some(opts));
		}
	}

	/// Print a formatted output to the console.
	pub fn show_txt(response: &[ApiResponse], opts: Option<&InfoOpts>) {
		let col_size = 16;
		let emoji_size = col_size - 1;
		response.iter().for_each(|r| {
			println!("{:<emoji_size$} {:<}", "🦀 Crate:", r.krate.crate_data.name,);

			if let Some(h) = r.krate.crate_data.homepage.as_ref() {
				println!("{:<col_size$} {:<}", "Homepage:", h);
			}

			if let Some(h) = r.krate.crate_data.repository.as_ref() {
				println!("{:<col_size$} {:<}", "Repository:", h);
			}

			if let Some(h) = r.krate.crate_data.documentation.as_ref() {
				println!("{:<col_size$} {:<}", "Documentation:", h);
			}

			match r.owners.len() {
				1 => println!(
					"{:<col_size$} {:<}",
					"Owner:",
					r.owners.first().expect("Missing user").name.as_ref().unwrap_or(&String::from("n/a"))
				),
				x if x > 1 => {
					print!("{:<col_size$} ", "Owners:");
					r.owners.iter().for_each(|user| print!("{}, ", user.name.as_ref().unwrap()));
					println!();
				}
				_ => {}
			}

			if let Some(latest) = r.krate.versions.first() {
				let latest = WrappedVersion::from(latest).version;

				let publisher_name = match &latest.published_by {
					Some(user) => match &user.name {
						Some(name) => name.into(),
						_ => String::from("n/a"),
					},
					_ => String::from("n/a"),
				};
				println!(
					"{:<col_size$} v{} by {} {} with {} downloads",
					"Latest:",
					latest.num,
					publisher_name,
					HumanTime::from(latest.updated_at),
					latest.downloads
				);
			}

			// println!("categories =\t{:?}", krate.categories);
			// println!("keywords =\t{:?}", krate.keywords);

			if let Some(options) = opts {
				if options.max_versions > 0 {
					// println!("Versions:");
					println!(
						"  {version:<9}\t{time:<16}\t{size:<10}\t{publisher:<20}\t{downloads:<10}\t{yanked:>8}",
						version = "VERSION",
						time = "UPDATED",
						size = "SIZE",
						publisher = "PUBLISHED BY",
						downloads = "DOWNLOADS",
						yanked = "YANKED",
					);

					r.krate
						.versions
						.iter()
						.enumerate()
						.take_while(|(i, _v)| i < &(options.max_versions as usize))
						.for_each(|(_i, v)| {
							let wv: WrappedVersion = WrappedVersion::from(v);
							println!("{wv}");
						});
					let total: u64 = r.krate.versions.len().try_into().unwrap_or(u64::MAX);
					if total > options.max_versions {
						println!("... and {} more", total - options.max_versions)
					}
				}
			}

			println!();
		});
	}

	/// Show the reponse as a json object. If there is a single object,
	/// it will be taken out of the array. An json array is returned oherwise.
	pub fn show_json(response: &Vec<ApiResponse>) {
		if response.len() == 1 {
			println!("{}", serde_json::to_string_pretty(&response.first()).unwrap());
		} else {
			println!("{}", serde_json::to_string_pretty(&response).unwrap());
		}
	}

	pub fn search(&self, pattern: &str, page_size: u64) -> anyhow::Result<Vec<Crate>> {
		let q = CratesQuery::builder().sort(Sort::Alphabetical).search(pattern).page_size(page_size).build();
		let crates = self.client.crates(q)?;

		Ok(crates.crates)
	}
}

#[cfg(test)]
mod test_super {
	use super::*;

	#[test]
	fn test_fetch() {
		let crates: Vec<&str> = vec!["cargo-crate", "sshq"];
		let opts = InfoOpts::default();

		let res = Info::new().fetch(crates, &opts);
		assert!(res.is_ok());
		assert_eq!(2, res.unwrap().len());
	}

	#[test]
	fn test_search() {
		let pattern = "cargo-crate";

		let crates = Info::new().search(pattern, 32).unwrap();
		println!("crates = {:?}", &crates.len());
		crates.iter().for_each(|c| println!("- {}", c.name));
		assert!(crates.len() > 10);
	}
}
