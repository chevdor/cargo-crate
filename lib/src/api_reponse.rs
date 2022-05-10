use crates_io_api::CrateResponse;
use crates_io_api::User;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse {
	pub krate: CrateResponse,
	pub owners: Vec<User>,
}
