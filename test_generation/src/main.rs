use std::fs::File;
use std::process::Command;
use bam::{Header, SamWriter};
use crate::command_line::CommandLineParameters;

mod command_line;
mod options;

fn main() {
	let params = CommandLineParameters::read();
	let sam = format!("{}.sam",params.output_path);
	let bam = format!("{}.bam",params.output_path);

	let sam_header = Header::new();

	let mut sam_writer = SamWriter::from_path(&sam, sam_header).expect("Couldn't create output SAM file");

	sam_writer.flush().expect("Couldn't write SAM file");

	// Convert from sam to bam

	let bam_file = File::create(&bam).expect("Couldn't create output BAM file");

	let mut convert_cmd = Command::new("samtools view");
	convert_cmd
		.arg("-S")
		.arg("-b")
		.arg(sam)
		.stdout(bam_file);

	println!("{:?}", convert_cmd);

	let convert = convert_cmd
		.status()
		.expect("Couldn't Convert sam to bam file");

	if !convert.success() {
		panic!("Couldn't Convert sam to bam file");
	}
}