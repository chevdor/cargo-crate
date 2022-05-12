use byte_unit::*;
use chrono_humanize::HumanTime;
use crates_io_api::Version;
use std::fmt::Display;

pub struct WrappedVersion<'a> {
	pub version: &'a Version,
}

impl<'a> Display for WrappedVersion<'a> {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		let c = self.version;
		let size = Byte::from_bytes(c.crate_size.unwrap_or_default() as u128);
		let adjusted_size = size.get_appropriate_unit(false);

		fmt.write_fmt(format_args!("- v{} - {} {}\n", c.num, adjusted_size, if c.yanked { "⚠️ YANKED" } else { "" }))?;
		// fmt.write_fmt(format_args!("created {}", c.created_at))?;
		// fmt.write_fmt(format_args!("updated {}\n", c.updated_at))?;
		let publisher = match &c.published_by {
			Some(user) => match user.name.as_ref() {
				Some(name) => name,
				_ => "n/a",
			},
			_ => "n/a",
		};
		fmt.write_fmt(format_args!("  published {} by {}", HumanTime::from(c.updated_at), publisher))?;
		fmt.write_fmt(format_args!(", downloads: {}", c.downloads))?;

		// fmt.write_fmt(format_args!("{:?}", self.version))?;
		Ok(())
	}
}

impl<'a> From<&'a Version> for WrappedVersion<'a> {
	fn from(version: &'a Version) -> Self {
		WrappedVersion { version }
	}
}
