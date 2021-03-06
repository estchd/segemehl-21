use clap::{Command, Arg};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TestGenerationMode {
	Reference,
	Default,
	Bin,
	ReferenceReadLength,
	CIGAR,
	MapQ,
	MapQPerRef,
	ReadLenRef,
	ReadLenSeq,
	Unmapped,
	RefLen,
	Split,
	Mixed,
	SplitGap,
	SplitCount,
	SplitTlen,
}

#[derive(Debug, Clone)]
pub struct CommandLineParameters {
	pub output_path: String,
	pub record_count: usize,
	pub mode: TestGenerationMode
}

impl CommandLineParameters {
	pub fn read() -> Self {
		let matches = Command::new("Segemehl21 Test Data Generator")
			.version("0.1")
			.author("Erik S.")
			.about("Generates BAM files to test Segemehl21")
			.arg(
				Arg::new("output_path")
					.long("output")
					.value_name("OUTPUT_BAM_PATH")
					.help("Path to the Output BAM File")
					.takes_value(true)
					.required(false)
			)
			.arg(
				Arg::new("record_count")
					.index(1)
					.value_name("RECORD_COUNT")
					.help("How many records the output file should contain")
					.takes_value(true)
					.required(false)
					.validator(number_validator)
			)
			.arg(
				Arg::new("mode")
					.long("mode")
					.value_name("MODE")
					.help("Test generation Mode")
					.takes_value(true)
					.required(false)
			)
			.get_matches();

		let output_path = matches.value_of("output_path").map(|item| String::from(item))
			.unwrap_or("output".to_string());
		let record_count = matches.value_of("record_count").map(|item| item.parse().unwrap())
			.unwrap_or(1000);
		let mode = matches.value_of("mode").map(|item|
			match item {
				"ref" => TestGenerationMode::Reference,
				"bin" => TestGenerationMode::Bin,
				"ref_read_len" => TestGenerationMode::ReferenceReadLength,
				"cigar" => TestGenerationMode::CIGAR,
				"mapq" => TestGenerationMode::MapQ,
				"mapq_per_ref" => TestGenerationMode::MapQPerRef,
				"read_len_ref" => TestGenerationMode::ReadLenRef,
				"read_len_seq" => TestGenerationMode::ReadLenSeq,
				"unmapped" => TestGenerationMode::Unmapped,
				"ref_len" => TestGenerationMode::RefLen,
				"split" => TestGenerationMode::Split,
				"mixed" => TestGenerationMode::Mixed,
				"split_gap" => TestGenerationMode::SplitGap,
				"split_count" => TestGenerationMode::SplitCount,
				"split_tlen" => TestGenerationMode::SplitTlen,
				_ => TestGenerationMode::Default
			}
		).unwrap_or(TestGenerationMode::Default);


		CommandLineParameters {
			output_path,
			record_count,
			mode
		}
	}
}

fn number_validator(value: &str) -> Result<(),String> {
	value.trim().parse::<usize>()
		.map(|_| ())
		.map_err(|err| format!("{}", err))
}