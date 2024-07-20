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
		let size = Byte::from_u64(c.crate_size.unwrap_or_default());
		let adjusted_size = format!("{:.2}", size.get_appropriate_unit(UnitType::Decimal));

		// fmt.write_fmt(format_args!("created {}", c.created_at))?;
		// fmt.write_fmt(format_args!("updated {}\n", c.updated_at))?;
		let publisher = match &c.published_by {
			Some(user) => match user.name.as_ref() {
				Some(name) => name,
				_ => "n/a",
			},
			_ => "n/a",
		};
		fmt.write_fmt(format_args!(
			"  v{version:<9}\t{time:<16}\t{size:<-10}\t{publisher:<20}\t{downloads:<10}\t{yanked:>8}",
			version = c.num,
			size = adjusted_size,
			yanked = if c.yanked { "⚠️ YANKED" } else { "" },
			publisher = publisher,
			time = HumanTime::from(c.updated_at),
			downloads = c.downloads,
		))?;
		// fmt.write_fmt(format_args!("  published {} by {}", HumanTime::from(c.updated_at), publisher))?;
		// fmt.write_fmt(format_args!(", downloads: {}", c.downloads))?;

		// fmt.write_fmt(format_args!("{:?}", self.version))?;
		Ok(())
	}
}

impl<'a> From<&'a Version> for WrappedVersion<'a> {
	fn from(version: &'a Version) -> Self {
		WrappedVersion { version }
	}
}
