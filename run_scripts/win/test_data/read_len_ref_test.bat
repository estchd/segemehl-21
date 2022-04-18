echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\read_len_ref"
mkdir ".\test_data\read_len_ref\bam"
mkdir ".\test_data\read_len_ref\sam"
mkdir ".\test_data\read_len_ref\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode read_len_ref --output .\test_data\read_len_ref\output

move .\test_data\read_len_ref\output_read_len_ref_0.bam .\test_data\read_len_ref\bam\output_read_len_ref_0.bam
move .\test_data\read_len_ref\output_read_len_ref_0.sam .\test_data\read_len_ref\sam\output_read_len_ref_0.sam
move .\test_data\read_len_ref\output_read_len_ref_1.bam .\test_data\read_len_ref\bam\output_read_len_ref_1.bam
move .\test_data\read_len_ref\output_read_len_ref_1.sam .\test_data\read_len_ref\sam\output_read_len_ref_1.sam
move .\test_data\read_len_ref\output_read_len_ref_2.bam .\test_data\read_len_ref\bam\output_read_len_ref_2.bam
move .\test_data\read_len_ref\output_read_len_ref_2.sam .\test_data\read_len_ref\sam\output_read_len_ref_2.sam
move .\test_data\read_len_ref\output_read_len_ref_3.bam .\test_data\read_len_ref\bam\output_read_len_ref_3.bam
move .\test_data\read_len_ref\output_read_len_ref_3.sam .\test_data\read_len_ref\sam\output_read_len_ref_3.sam
move .\test_data\read_len_ref\output_read_len_ref_4.bam .\test_data\read_len_ref\bam\output_read_len_ref_4.bam
move .\test_data\read_len_ref\output_read_len_ref_4.sam .\test_data\read_len_ref\sam\output_read_len_ref_4.sam
move .\test_data\read_len_ref\output_read_len_ref_5.bam .\test_data\read_len_ref\bam\output_read_len_ref_5.bam
move .\test_data\read_len_ref\output_read_len_ref_5.sam .\test_data\read_len_ref\sam\output_read_len_ref_5.sam
move .\test_data\read_len_ref\output_read_len_ref_6.bam .\test_data\read_len_ref\bam\output_read_len_ref_6.bam
move .\test_data\read_len_ref\output_read_len_ref_6.sam .\test_data\read_len_ref\sam\output_read_len_ref_6.sam
move .\test_data\read_len_ref\output_read_len_ref_7.bam .\test_data\read_len_ref\bam\output_read_len_ref_7.bam
move .\test_data\read_len_ref\output_read_len_ref_7.sam .\test_data\read_len_ref\sam\output_read_len_ref_7.sam
move .\test_data\read_len_ref\output_read_len_ref_8.bam .\test_data\read_len_ref\bam\output_read_len_ref_8.bam
move .\test_data\read_len_ref\output_read_len_ref_8.sam .\test_data\read_len_ref\sam\output_read_len_ref_8.sam
move .\test_data\read_len_ref\output_read_len_ref_9.bam .\test_data\read_len_ref\bam\output_read_len_ref_9.bam
move .\test_data\read_len_ref\output_read_len_ref_9.sam .\test_data\read_len_ref\sam\output_read_len_ref_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_0.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_1.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_2.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_3.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_4.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_5.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_6.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_7.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_8.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_ref\bam\output_read_len_ref_9.bam --output .\test_data\read_len_ref\stat\output_read_len_ref_9.stat

cd .\run_scripts\win\test_data