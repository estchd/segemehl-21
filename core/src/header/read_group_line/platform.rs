use std::convert::TryFrom;

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
	type Error = ();

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
			_ => Err(())
		}
	}
}