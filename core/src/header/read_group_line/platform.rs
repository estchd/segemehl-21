use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlatformError {
	#[error("the provided platform is not a valid platform")]
	InvalidPlatform
}

pub enum Platform {
	CAPILLARY,
	DNBSEQ,
	HELICOS,
	ILLUMINA,
	IONTORRENT,
	LS454,
	ONT,
	PACBIO,
	SOLID
}

impl TryFrom<&str> for Platform {
	type Error = PlatformError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"CAPILLARY" => Ok(Self::CAPILLARY),
			"DNBSEQ" => Ok(Self::DNBSEQ),
			"HELICOS" => Ok(Self::HELICOS),
			"ILLUMINA" => Ok(Self::ILLUMINA),
			"IONTORRENT" => Ok(Self::IONTORRENT),
			"LS454" => Ok(Self::LS454),
			"ONT" => Ok(Self::ONT),
			"PACBIO" => Ok(Self::PACBIO),
			"SOLID" => Ok(Self::SOLID),
			_ => Err(PlatformError::InvalidPlatform)
		}
	}
}