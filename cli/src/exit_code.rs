#[derive(Debug)]
pub enum ExitCode {
	OK,
	GeneralError,
	NoResultFound,
}

impl From<ExitCode> for i32 {
	fn from(err: ExitCode) -> Self {
		match err {
			ExitCode::OK => 0,
			ExitCode::GeneralError => 1,
			ExitCode::NoResultFound => 2,
		}
	}
}
