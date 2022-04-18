echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\bin"
mkdir ".\test_data\bin\bam"
mkdir ".\test_data\bin\sam"
mkdir ".\test_data\bin\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode bin --output .\test_data\bin\output

move .\test_data\bin\output_bin_0.bam .\test_data\bin\bam\output_bin_0.bam
move .\test_data\bin\output_bin_0.sam .\test_data\bin\sam\output_bin_0.sam
move .\test_data\bin\output_bin_1.bam .\test_data\bin\bam\output_bin_1.bam
move .\test_data\bin\output_bin_1.sam .\test_data\bin\sam\output_bin_1.sam
move .\test_data\bin\output_bin_2.bam .\test_data\bin\bam\output_bin_2.bam
move .\test_data\bin\output_bin_2.sam .\test_data\bin\sam\output_bin_2.sam
move .\test_data\bin\output_bin_3.bam .\test_data\bin\bam\output_bin_3.bam
move .\test_data\bin\output_bin_3.sam .\test_data\bin\sam\output_bin_3.sam
move .\test_data\bin\output_bin_4.bam .\test_data\bin\bam\output_bin_4.bam
move .\test_data\bin\output_bin_4.sam .\test_data\bin\sam\output_bin_4.sam
move .\test_data\bin\output_bin_5.bam .\test_data\bin\bam\output_bin_5.bam
move .\test_data\bin\output_bin_5.sam .\test_data\bin\sam\output_bin_5.sam
move .\test_data\bin\output_bin_6.bam .\test_data\bin\bam\output_bin_6.bam
move .\test_data\bin\output_bin_6.sam .\test_data\bin\sam\output_bin_6.sam
move .\test_data\bin\output_bin_7.bam .\test_data\bin\bam\output_bin_7.bam
move .\test_data\bin\output_bin_7.sam .\test_data\bin\sam\output_bin_7.sam
move .\test_data\bin\output_bin_8.bam .\test_data\bin\bam\output_bin_8.bam
move .\test_data\bin\output_bin_8.sam .\test_data\bin\sam\output_bin_8.sam
move .\test_data\bin\output_bin_9.bam .\test_data\bin\bam\output_bin_9.bam
move .\test_data\bin\output_bin_9.sam .\test_data\bin\sam\output_bin_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_0.bam --output .\test_data\bin\stat\output_bin_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_1.bam --output .\test_data\bin\stat\output_bin_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_2.bam --output .\test_data\bin\stat\output_bin_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_3.bam --output .\test_data\bin\stat\output_bin_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_4.bam --output .\test_data\bin\stat\output_bin_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_5.bam --output .\test_data\bin\stat\output_bin_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_6.bam --output .\test_data\bin\stat\output_bin_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_7.bam --output .\test_data\bin\stat\output_bin_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_8.bam --output .\test_data\bin\stat\output_bin_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\bin\bam\output_bin_9.bam --output .\test_data\bin\stat\output_bin_9.stat

cd .\run_scripts\win\test_data