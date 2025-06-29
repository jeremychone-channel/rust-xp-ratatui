use derive_more::From;
use tokio::task::JoinError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
	#[from]
	Custom(String),

	// -- Externals
	#[from]
	TokioJoin(JoinError),
	#[from]
	Io(std::io::Error), // as example
}

// region:    --- Custom

impl Error {
	pub fn custom(val: impl std::fmt::Display) -> Self {
		Self::Custom(val.to_string())
	}
}

impl From<&str> for Error {
	fn from(val: &str) -> Self {
		Self::Custom(val.to_string())
	}
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
