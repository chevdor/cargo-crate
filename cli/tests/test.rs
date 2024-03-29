#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod help {
		use assert_cmd::Command;

		#[test]
		fn it_shows_help() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod info {
		use assert_cmd::Command;
		#[test]
		fn it_returns_info() {
			const CRATES: [&str; 4] = ["cargo-crate", "glob", "globset", "subwasm"];
			CRATES.iter().for_each(|c| {
				println!("Checking infos for crate: {c}");
				let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
				let assert = cmd.args(["info", c]).assert();
				assert.success().code(0);
			})
		}
	}

	#[cfg(test)]
	mod open {
		use assert_cmd::Command;
		#[test]
		fn it_gets_a_runtime() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

			let assert = cmd.args(["open", "cargo-crate"]).assert();
			assert.success().code(0);
		}
	}
}
