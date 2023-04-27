use crate::types::CrateName;
use anyhow::{bail, Result};
use lib_cargo_crate::get_crates;
use std::{fmt::Display, path::PathBuf};

/// When specifying one or more crates, the user
/// has the option to provide a crate name or a path
/// on the local filesystem. In that case, the name
/// will be fetched from the Cargo.toml.
#[derive(Debug, Clone)]
pub enum CrateInput {
	Name(CrateName),
	Path(PathBuf),
}

impl CrateInput {
	pub fn names(&self) -> Result<Vec<CrateName>> {
		let names: Vec<CrateName> = match self {
			// This case is straight forward, we got the name!
			CrateInput::Name(n) => vec![n.into()],

			// This is less trivial as the path maybe a folder, containing a Cargo.toml
			// that itself is a workspace.
			CrateInput::Path(p) => {
				let candidates = get_crates(p)?;
				let names: Vec<CrateName> = candidates
					.iter()
					.map(|candidate| {
						if let Some(pkg) = candidate.package.clone() {
							pkg.name
						} else {
							unreachable!("get_crates returned one or more workspaces !")
						}
					})
					.collect();

				names
			}
		};

		if names.is_empty() {
			bail!("Could not find any valid name")
		} else {
			Ok(names)
		}
	}
}

impl Display for CrateInput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			CrateInput::Name(name) => f.write_fmt(format_args!("Name: {name}")),
			CrateInput::Path(path) => f.write_fmt(format_args!("Path: {}", path.display())),
		}
	}
}

impl From<&str> for CrateInput {
	fn from(value: &str) -> Self {
		let path_maybe = PathBuf::from(value);
		if path_maybe.exists() {
			CrateInput::Path(path_maybe)
		} else {
			CrateInput::Name(value.into())
		}
	}
}

#[cfg(test)]
mod test_crate_input {
	use super::*;

	#[test]
	fn test_name() {
		const VALUE: &str = "IamDefinitelyNotAfile";
		let input = CrateInput::from(VALUE);
		let names = input.names().unwrap();
		assert_eq!(vec![VALUE.to_string()], names);
	}

	#[test]
	fn test_folder_package() {
		const VALUE: &str = ".";
		let input = CrateInput::from(VALUE);
		let names = input.names().unwrap();
		assert_eq!(vec!["cargo-crate".to_string()], names);
	}

	#[test]
	fn test_folder_workspace() {
		const VALUE: &str = "..";
		let input = CrateInput::from(VALUE);
		let names = input.names().unwrap();
		assert_eq!(vec!["cargo-crate".to_string(), "lib-cargo-crate".to_string()], names);
	}

	#[test]
	fn test_file() {
		const VALUE: &str = "./Cargo.toml";
		let input = CrateInput::from(VALUE);
		let names = input.names().unwrap();
		assert_eq!(vec!["cargo-crate".to_string()], names);
	}
}
