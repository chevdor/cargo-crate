use chrono_humanize::HumanTime;
use crate::api_reponse::*;
use crate::info_opts::InfoOpts;
use crate::WrappedVersion;
use crates_io_api::{Error, SyncClient};
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
	pub fn fetch(&self, crates: Vec<String>, _opts: &InfoOpts) -> anyhow::Result<Vec<ApiResponse>> {
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
		let col_size = 12;
		let emoji_size = col_size -1;
		response.iter().for_each(|r| {
			println!("{:>emoji_size$} {:<}", "🦀 Crate:", r.krate.crate_data.name,);

			if let Some(h) = r.krate.crate_data.homepage.as_ref() {
				println!("{:>col_size$} {:<}", "Homepage:", h);
			}

			if let Some(h) = r.krate.crate_data.repository.as_ref() {
				println!("{:>col_size$} {:<}", "Repository:", h);
			}

			match r.owners.len() {
				1 => println!("{:>col_size$} {:<}", "Owner:", r.owners.first().unwrap().name.as_ref().unwrap()),
				x if x > 1 => {
					print!("{:>col_size$} ", "Owners:");
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
					"{:>col_size$} v{} by {} {} with {} downloads", "Latest:",
					latest.num,
					publisher_name,
					HumanTime::from(latest.updated_at),
					latest.downloads
				);
			}

			// println!("categories =\t{:?}", krate.categories);
			// println!("keywords =\t{:?}", krate.keywords);

			if let Some(options) = opts {
				if !options.no_versions {
					println!("Versions:");
					r.krate.versions.iter().for_each(|v| {
						let wv: WrappedVersion = WrappedVersion::from(v);
						println!("{}", wv);
					});
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
}

#[cfg(test)]
mod test_super {
	use super::*;

	#[test]
	fn test_fetch() {
		let crates: Vec<String> = vec!["cargo-crate".to_string(), "sshq".to_string()];
		let opts = InfoOpts { json: true };

		let res = Info::new().fetch(crates, &opts);
		assert!(res.is_ok());
		assert_eq!(2, res.unwrap().len());
	}
}
