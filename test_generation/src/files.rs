use bam::{BamWriter, Header, Record, RecordWriter, SamWriter};
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

pub struct Files {
	sam: SamWriter<BufWriter<File>>,
	bam: BamWriter<File>
}

impl Files {
	pub fn from_path<S: AsRef<Path>, B: AsRef<Path>>(sam_path: S, bam_path: B, header: Header) -> Self {
		let sam = SamWriter::from_path(sam_path, header.clone()).expect("Couldn't create output SAM file");
		let bam = BamWriter::from_path(bam_path, header.clone()).expect("Couldn't create output BAM file");

		Self {
			sam,
			bam
		}
	}

	pub fn write(&mut self, record: &Record) {
		self.sam.write(record).unwrap();
		self.bam.write(record).unwrap();
	}

	pub fn flush(&mut self) {
		self.sam.flush().expect("Couldn't write SAM file");
		self.bam.flush().expect("Couldn't write BAM file");
	}
}
