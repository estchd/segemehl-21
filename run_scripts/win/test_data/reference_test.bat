echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\reference"
mkdir ".\test_data\reference\bam"
mkdir ".\test_data\reference\sam"
mkdir ".\test_data\reference\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode ref --output .\test_data\reference\output

move .\test_data\reference\output_reference_0.bam .\test_data\reference\bam\output_reference_0.bam
move .\test_data\reference\output_reference_0.sam .\test_data\reference\sam\output_reference_0.sam
move .\test_data\reference\output_reference_1.bam .\test_data\reference\bam\output_reference_1.bam
move .\test_data\reference\output_reference_1.sam .\test_data\reference\sam\output_reference_1.sam
move .\test_data\reference\output_reference_2.bam .\test_data\reference\bam\output_reference_2.bam
move .\test_data\reference\output_reference_2.sam .\test_data\reference\sam\output_reference_2.sam
move .\test_data\reference\output_reference_3.bam .\test_data\reference\bam\output_reference_3.bam
move .\test_data\reference\output_reference_3.sam .\test_data\reference\sam\output_reference_3.sam
move .\test_data\reference\output_reference_4.bam .\test_data\reference\bam\output_reference_4.bam
move .\test_data\reference\output_reference_4.sam .\test_data\reference\sam\output_reference_4.sam
move .\test_data\reference\output_reference_5.bam .\test_data\reference\bam\output_reference_5.bam
move .\test_data\reference\output_reference_5.sam .\test_data\reference\sam\output_reference_5.sam
move .\test_data\reference\output_reference_6.bam .\test_data\reference\bam\output_reference_6.bam
move .\test_data\reference\output_reference_6.sam .\test_data\reference\sam\output_reference_6.sam
move .\test_data\reference\output_reference_7.bam .\test_data\reference\bam\output_reference_7.bam
move .\test_data\reference\output_reference_7.sam .\test_data\reference\sam\output_reference_7.sam
move .\test_data\reference\output_reference_8.bam .\test_data\reference\bam\output_reference_8.bam
move .\test_data\reference\output_reference_8.sam .\test_data\reference\sam\output_reference_8.sam
move .\test_data\reference\output_reference_9.bam .\test_data\reference\bam\output_reference_9.bam
move .\test_data\reference\output_reference_9.sam .\test_data\reference\sam\output_reference_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_0.bam --output .\test_data\reference\stat\output_reference_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_1.bam --output .\test_data\reference\stat\output_reference_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_2.bam --output .\test_data\reference\stat\output_reference_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_3.bam --output .\test_data\reference\stat\output_reference_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_4.bam --output .\test_data\reference\stat\output_reference_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_5.bam --output .\test_data\reference\stat\output_reference_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_6.bam --output .\test_data\reference\stat\output_reference_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_7.bam --output .\test_data\reference\stat\output_reference_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_8.bam --output .\test_data\reference\stat\output_reference_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\reference\bam\output_reference_9.bam --output .\test_data\reference\stat\output_reference_9.stat

cd .\run_scripts\win\test_data