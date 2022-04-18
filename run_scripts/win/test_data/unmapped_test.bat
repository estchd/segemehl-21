echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\unmapped"
mkdir ".\test_data\unmapped\bam"
mkdir ".\test_data\unmapped\sam"
mkdir ".\test_data\unmapped\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode unmapped --output .\test_data\unmapped\output

move .\test_data\unmapped\output_unmapped_0.bam .\test_data\unmapped\bam\output_unmapped_0.bam
move .\test_data\unmapped\output_unmapped_0.sam .\test_data\unmapped\sam\output_unmapped_0.sam
move .\test_data\unmapped\output_unmapped_1.bam .\test_data\unmapped\bam\output_unmapped_1.bam
move .\test_data\unmapped\output_unmapped_1.sam .\test_data\unmapped\sam\output_unmapped_1.sam
move .\test_data\unmapped\output_unmapped_2.bam .\test_data\unmapped\bam\output_unmapped_2.bam
move .\test_data\unmapped\output_unmapped_2.sam .\test_data\unmapped\sam\output_unmapped_2.sam
move .\test_data\unmapped\output_unmapped_3.bam .\test_data\unmapped\bam\output_unmapped_3.bam
move .\test_data\unmapped\output_unmapped_3.sam .\test_data\unmapped\sam\output_unmapped_3.sam
move .\test_data\unmapped\output_unmapped_4.bam .\test_data\unmapped\bam\output_unmapped_4.bam
move .\test_data\unmapped\output_unmapped_4.sam .\test_data\unmapped\sam\output_unmapped_4.sam
move .\test_data\unmapped\output_unmapped_5.bam .\test_data\unmapped\bam\output_unmapped_5.bam
move .\test_data\unmapped\output_unmapped_5.sam .\test_data\unmapped\sam\output_unmapped_5.sam
move .\test_data\unmapped\output_unmapped_6.bam .\test_data\unmapped\bam\output_unmapped_6.bam
move .\test_data\unmapped\output_unmapped_6.sam .\test_data\unmapped\sam\output_unmapped_6.sam
move .\test_data\unmapped\output_unmapped_7.bam .\test_data\unmapped\bam\output_unmapped_7.bam
move .\test_data\unmapped\output_unmapped_7.sam .\test_data\unmapped\sam\output_unmapped_7.sam
move .\test_data\unmapped\output_unmapped_8.bam .\test_data\unmapped\bam\output_unmapped_8.bam
move .\test_data\unmapped\output_unmapped_8.sam .\test_data\unmapped\sam\output_unmapped_8.sam
move .\test_data\unmapped\output_unmapped_9.bam .\test_data\unmapped\bam\output_unmapped_9.bam
move .\test_data\unmapped\output_unmapped_9.sam .\test_data\unmapped\sam\output_unmapped_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_0.bam --output .\test_data\unmapped\stat\output_unmapped_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_1.bam --output .\test_data\unmapped\stat\output_unmapped_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_2.bam --output .\test_data\unmapped\stat\output_unmapped_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_3.bam --output .\test_data\unmapped\stat\output_unmapped_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_4.bam --output .\test_data\unmapped\stat\output_unmapped_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_5.bam --output .\test_data\unmapped\stat\output_unmapped_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_6.bam --output .\test_data\unmapped\stat\output_unmapped_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_7.bam --output .\test_data\unmapped\stat\output_unmapped_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_8.bam --output .\test_data\unmapped\stat\output_unmapped_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\unmapped\bam\output_unmapped_9.bam --output .\test_data\unmapped\stat\output_unmapped_9.stat

cd .\run_scripts\win\test_data