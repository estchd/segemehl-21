echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\read_len_seq"
mkdir ".\test_data\read_len_seq\bam"
mkdir ".\test_data\read_len_seq\sam"
mkdir ".\test_data\read_len_seq\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode read_len_seq --output .\test_data\read_len_seq\output

move .\test_data\read_len_seq\output_read_len_seq_0.bam .\test_data\read_len_seq\bam\output_read_len_seq_0.bam
move .\test_data\read_len_seq\output_read_len_seq_0.sam .\test_data\read_len_seq\sam\output_read_len_seq_0.sam
move .\test_data\read_len_seq\output_read_len_seq_1.bam .\test_data\read_len_seq\bam\output_read_len_seq_1.bam
move .\test_data\read_len_seq\output_read_len_seq_1.sam .\test_data\read_len_seq\sam\output_read_len_seq_1.sam
move .\test_data\read_len_seq\output_read_len_seq_2.bam .\test_data\read_len_seq\bam\output_read_len_seq_2.bam
move .\test_data\read_len_seq\output_read_len_seq_2.sam .\test_data\read_len_seq\sam\output_read_len_seq_2.sam
move .\test_data\read_len_seq\output_read_len_seq_3.bam .\test_data\read_len_seq\bam\output_read_len_seq_3.bam
move .\test_data\read_len_seq\output_read_len_seq_3.sam .\test_data\read_len_seq\sam\output_read_len_seq_3.sam
move .\test_data\read_len_seq\output_read_len_seq_4.bam .\test_data\read_len_seq\bam\output_read_len_seq_4.bam
move .\test_data\read_len_seq\output_read_len_seq_4.sam .\test_data\read_len_seq\sam\output_read_len_seq_4.sam
move .\test_data\read_len_seq\output_read_len_seq_5.bam .\test_data\read_len_seq\bam\output_read_len_seq_5.bam
move .\test_data\read_len_seq\output_read_len_seq_5.sam .\test_data\read_len_seq\sam\output_read_len_seq_5.sam
move .\test_data\read_len_seq\output_read_len_seq_6.bam .\test_data\read_len_seq\bam\output_read_len_seq_6.bam
move .\test_data\read_len_seq\output_read_len_seq_6.sam .\test_data\read_len_seq\sam\output_read_len_seq_6.sam
move .\test_data\read_len_seq\output_read_len_seq_7.bam .\test_data\read_len_seq\bam\output_read_len_seq_7.bam
move .\test_data\read_len_seq\output_read_len_seq_7.sam .\test_data\read_len_seq\sam\output_read_len_seq_7.sam
move .\test_data\read_len_seq\output_read_len_seq_8.bam .\test_data\read_len_seq\bam\output_read_len_seq_8.bam
move .\test_data\read_len_seq\output_read_len_seq_8.sam .\test_data\read_len_seq\sam\output_read_len_seq_8.sam
move .\test_data\read_len_seq\output_read_len_seq_9.bam .\test_data\read_len_seq\bam\output_read_len_seq_9.bam
move .\test_data\read_len_seq\output_read_len_seq_9.sam .\test_data\read_len_seq\sam\output_read_len_seq_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_0.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_1.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_2.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_3.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_4.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_5.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_6.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_7.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_8.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\read_len_seq\bam\output_read_len_seq_9.bam --output .\test_data\read_len_seq\stat\output_read_len_seq_9.stat

cd .\run_scripts\win\test_data