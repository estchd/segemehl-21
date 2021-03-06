use std::num::{NonZeroU32};
use clap::{App, Arg};

#[derive(Debug, Clone)]
pub struct CommandLineParameters {
    pub bam_path: String,
    pub bai_path: Option<String>,
    pub output_path: String,
    pub expected_record_count: Option<usize>,
    pub bin_size: Option<NonZeroU32>,
    pub info_dump: bool
}

impl CommandLineParameters {
    pub fn read() -> Self {
        let matches = App::new("Segemehl-21 Backend")
            .version("0.1")
            .author("Erik S.")
            .about("Generates Segemehl-21 Statistics Data from Bam Files")
            .arg(
                Arg::with_name("bam_path")
                    .long("input")
                    .value_name("BAM_PATH")
                    .help("Path to the Input Bam File")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                Arg::with_name("bai_path")
                    .long("index")
                    .value_name("BAI_PATH")
                    .help("Path to the Index for the Bai Index File")
                    .takes_value(true)
                    .required(false)
            )
            .arg(
                Arg::with_name("output_path")
                    .long("output")
                    .value_name("OUTPUT_PATH")
                    .help("Name of the output File that will be generated")
                    .takes_value(true)
                    .required(false)

            )
            .arg(
                Arg::with_name("expected_record_count")
                    .long("record_count")
                    .value_name("EXPECTED_RECORD_COUNT")
                    .help("How many records the bam file is expected to contain")
                    .takes_value(true)
                    .required(false)
                    .validator(number_validator)
            )
            .arg(
                Arg::with_name("bin_size")
                    .long("bin_size")
                    .value_name("BIN_SIZE")
                    .help("The bam size that should be used")
                    .takes_value(true)
                    .required(false)
                    .validator(non_zero_number_validator)
            )
            .arg(
                Arg::with_name("info_dump")
                    .long("info_dump")
                    .value_name("INFO_DUMP")
                    .help("Dump Numeric Infos for each Reference into the Command Line")
                    .takes_value(false)
                    .required(false)
            )
            .get_matches();

        let bam_path = matches.value_of("bam_path").map(|item| String::from(item))
            .unwrap_or("input.bam".to_string());
        let bai_path = matches.value_of("bai_path").map(|item| String::from(item));
        let output_path = matches.value_of("output_path").map(|item| String::from(item))
            .unwrap_or("output.stat".to_string());
        let expected_record_count = matches.value_of("expected_record_count")
            .map(|item| item.trim().parse::<usize>().unwrap());
        let bin_size = matches.value_of("bin_size")
                .map(|item| item.parse::<u32>().unwrap())
                .map(|item| NonZeroU32::new(item).unwrap());

        let info_dump = matches.is_present("info_dump");

        CommandLineParameters {
            bam_path,
            bai_path,
            output_path,
            expected_record_count,
            bin_size,
            info_dump
        }
    }
}

fn number_validator(value: String) -> Result<(),String> {
    value.trim().parse::<usize>()
        .map(|_| ())
        .map_err(|err| format!("{}", err))
}

fn non_zero_number_validator(value: String) -> Result<(), String> {
    let number = value.trim().parse::<u32>()
        .map_err(|err| format!("{}", err))?;

    NonZeroU32::new(number)
        .map(|_| ())
        .ok_or("Number was Zero".to_string())
}