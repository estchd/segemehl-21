echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\reference_read_length"
mkdir ".\test_data\reference_read_length\bam"
mkdir ".\test_data\reference_read_length\sam"
mkdir ".\test_data\reference_read_length\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode ref_read_len --output .\test_data\reference_read_length\output

move .\test_data\reference_read_length\output_reference_read_length_0.bam .\test_data\reference_read_length\bam\output_reference_read_length_0.bam
move .\test_data\reference_read_length\output_reference_read_length_0.sam .\test_data\reference_read_length\sam\output_reference_read_length_0.sam
move .\test_data\reference_read_length\output_reference_read_length_1.bam .\test_data\reference_read_length\bam\output_reference_read_length_1.bam
move .\test_data\reference_read_length\output_reference_read_length_1.sam .\test_data\reference_read_length\sam\output_reference_read_length_1.sam
move .\test_data\reference_read_length\output_reference_read_length_2.bam .\test_data\reference_read_length\bam\output_reference_read_length_2.bam
move .\test_data\reference_read_length\output_reference_read_length_2.sam .\test_data\reference_read_length\sam\output_reference_read_length_2.sam
move .\test_data\reference_read_length\output_reference_read_length_3.bam .\test_data\reference_read_length\bam\output_reference_read_length_3.bam
move .\test_data\reference_read_length\output_reference_read_length_3.sam .\test_data\reference_read_length\sam\output_reference_read_length_3.sam
move .\test_data\reference_read_length\output_reference_read_length_4.bam .\test_data\reference_read_length\bam\output_reference_read_length_4.bam
move .\test_data\reference_read_length\output_reference_read_length_4.sam .\test_data\reference_read_length\sam\output_reference_read_length_4.sam
move .\test_data\reference_read_length\output_reference_read_length_5.bam .\test_data\reference_read_length\bam\output_reference_read_length_5.bam
move .\test_data\reference_read_length\output_reference_read_length_5.sam .\test_data\reference_read_length\sam\output_reference_read_length_5.sam
move .\test_data\reference_read_length\output_reference_read_length_6.bam .\test_data\reference_read_length\bam\output_reference_read_length_6.bam
move .\test_data\reference_read_length\output_reference_read_length_6.sam .\test_data\reference_read_length\sam\output_reference_read_length_6.sam
move .\test_data\reference_read_length\output_reference_read_length_7.bam .\test_data\reference_read_length\bam\output_reference_read_length_7.bam
move .\test_data\reference_read_length\output_reference_read_length_7.sam .\test_data\reference_read_length\sam\output_reference_read_length_7.sam
move .\test_data\reference_read_length\output_reference_read_length_8.bam .\test_data\reference_read_length\bam\output_reference_read_length_8.bam
move .\test_data\reference_read_length\output_reference_read_length_8.sam .\test_data\reference_read_length\sam\output_reference_read_length_8.sam
move .\test_data\reference_read_length\output_reference_read_length_9.bam .\test_data\reference_read_length\bam\output_reference_read_length_9.bam
move .\test_data\reference_read_length\output_reference_read_length_9.sam .\test_data\reference_read_length\sam\output_reference_read_length_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_0.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_1.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_2.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_3.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_4.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_5.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_6.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_7.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_8.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference_read_length\bam\output_reference_read_length_9.bam --output .\test_data\reference_read_length\stat\output_reference_read_length_9.stat

cd .\run_scripts\win\test_data