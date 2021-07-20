use clap::{App, Arg};

pub struct CommandLineParameters {
    pub bam_path: String,
    pub bai_path: Option<String>,
    pub output_path: String
}

impl CommandLineParameters {
    pub fn read() -> Self {
        let matches = App::new("Segemehl-21 Backend")
            .version("0.1")
            .author("Erik S.")
            .about("Generates Segemehl-21 Statistics Data from Bam Files")
            .arg(
                Arg::with_name("bam_path")
                    .short("i")
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
                    .short("o")
                    .long("output")
                    .value_name("OUTPUT_PATH")
                    .help("Name of the output File that will be generated")
                    .takes_value(true)
                    .required(false)

            )
            .get_matches();

        let bam_path = matches.value_of("bam_path").map(|item| String::from(item)).unwrap();
        let bai_path = matches.value_of("bai_path").map(|item| String::from(item));
        let output_path = matches.value_of("output_path").map(|item| String::from(item))
            .unwrap_or("statistics.json".to_string());

        CommandLineParameters {
            bam_path,
            bai_path,
            output_path
        }
    }
}