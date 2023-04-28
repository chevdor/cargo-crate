use anyhow::*;
use cargo_toml::Manifest;
use std::{
	fs,
	path::{Path, PathBuf},
};

/// This helper function takes a path as input. It can be a folder
/// or a toml file (Usually named Cargo.toml). If returns the name
/// of the crate for a single package or the list of packages for workspaces.
pub fn get_crates(path: &Path) -> Result<Vec<Manifest>> {
	let path = path.canonicalize()?;

	if !path.exists() {
		bail!("Path is expected to be valid and exsist");
	}

	// we want `path` to be a folder and `cargo_toml_path` to be a file
	let path = if path.is_file() { path.parent().unwrap().into() } else { path };
	let cargo_toml_path = if path.is_dir() { path.join("Cargo.toml") } else { path.clone() };

	let crate_file = fs::read(cargo_toml_path);
	if crate_file.is_err() {
		bail!("Impossible to find a valid Cargo.toml at {}", path.display());
	}

	let crate_info = cargo_toml::Manifest::from_slice(&crate_file.unwrap());
	if crate_info.is_err() {
		bail!("Impossible to load a valid Cargo.toml at {}", path.display());
	}

	let crate_info = crate_info.unwrap();

	let size = if let Some(workspace) = &crate_info.workspace {
		workspace.members.len()
	} else if crate_info.package.is_some() {
		1
	} else {
		bail!("No package found")
	};

	let mut result: Vec<Manifest> = Vec::with_capacity(size);

	if crate_info.package.is_some() {
		result.push(crate_info)
	} else if let Some(workspace) = crate_info.workspace {
		let mut crates: Vec<Manifest> = workspace
			.members
			.iter()
			.flat_map(|local_path| {
				let crate_path = path.join(PathBuf::from(local_path));
				get_crates(&crate_path).unwrap()
			})
			.collect();
		result.append(&mut crates)
	} else {
		bail!("Workspace has no members")
	}

	Ok(result)
}

#[cfg(test)]
mod test_utils {
	use super::*;

	#[test]
	fn test_crate_from_folder() {
		let crates = get_crates(&PathBuf::from(".")).unwrap();
		// println!("crates = {:?}", crates);
		assert_eq!(1, crates.len());
	}

	#[test]
	fn test_crate_from_file() {
		let crates = get_crates(&PathBuf::from("./Cargo.toml")).unwrap();
		// println!("crates = {:?}", crates);
		assert_eq!(1, crates.len());
	}

	#[test]
	fn test_workspace_from_folder() {
		let crates = get_crates(&PathBuf::from("..")).unwrap();
		// println!("crates = {:?}", crates);
		assert_eq!(2, crates.len());
	}

	#[test]
	fn test_workspace_from_file() {
		let crates = get_crates(&PathBuf::from("../Cargo.toml")).unwrap();
		// println!("crates = {:?}", crates);
		assert_eq!(2, crates.len());
	}
}
