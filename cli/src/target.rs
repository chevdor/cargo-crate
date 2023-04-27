use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum Target {
	CratesIo,
	Repository,
	Homepage,
	Documentation,
}

impl Display for Target {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Target::CratesIo => f.write_str("crates.io"),
			Target::Repository => f.write_str("repository"),
			Target::Homepage => f.write_str("homepage"),
			Target::Documentation => f.write_str("documentation"),
		}
	}
}
