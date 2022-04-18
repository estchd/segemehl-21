echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\cigar"
mkdir ".\test_data\cigar\bam"
mkdir ".\test_data\cigar\sam"
mkdir ".\test_data\cigar\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode cigar --output .\test_data\cigar\output

move .\test_data\cigar\output_cigar_0.bam .\test_data\cigar\bam\output_cigar_0.bam
move .\test_data\cigar\output_cigar_0.sam .\test_data\cigar\sam\output_cigar_0.sam
move .\test_data\cigar\output_cigar_1.bam .\test_data\cigar\bam\output_cigar_1.bam
move .\test_data\cigar\output_cigar_1.sam .\test_data\cigar\sam\output_cigar_1.sam
move .\test_data\cigar\output_cigar_2.bam .\test_data\cigar\bam\output_cigar_2.bam
move .\test_data\cigar\output_cigar_2.sam .\test_data\cigar\sam\output_cigar_2.sam
move .\test_data\cigar\output_cigar_3.bam .\test_data\cigar\bam\output_cigar_3.bam
move .\test_data\cigar\output_cigar_3.sam .\test_data\cigar\sam\output_cigar_3.sam
move .\test_data\cigar\output_cigar_4.bam .\test_data\cigar\bam\output_cigar_4.bam
move .\test_data\cigar\output_cigar_4.sam .\test_data\cigar\sam\output_cigar_4.sam
move .\test_data\cigar\output_cigar_5.bam .\test_data\cigar\bam\output_cigar_5.bam
move .\test_data\cigar\output_cigar_5.sam .\test_data\cigar\sam\output_cigar_5.sam
move .\test_data\cigar\output_cigar_6.bam .\test_data\cigar\bam\output_cigar_6.bam
move .\test_data\cigar\output_cigar_6.sam .\test_data\cigar\sam\output_cigar_6.sam
move .\test_data\cigar\output_cigar_7.bam .\test_data\cigar\bam\output_cigar_7.bam
move .\test_data\cigar\output_cigar_7.sam .\test_data\cigar\sam\output_cigar_7.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_0.bam --output .\test_data\cigar\stat\output_cigar_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_1.bam --output .\test_data\cigar\stat\output_cigar_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_2.bam --output .\test_data\cigar\stat\output_cigar_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_3.bam --output .\test_data\cigar\stat\output_cigar_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_4.bam --output .\test_data\cigar\stat\output_cigar_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_5.bam --output .\test_data\cigar\stat\output_cigar_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_6.bam --output .\test_data\cigar\stat\output_cigar_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\cigar\bam\output_cigar_7.bam --output .\test_data\cigar\stat\output_cigar_7.stat

cd .\run_scripts\win\test_data