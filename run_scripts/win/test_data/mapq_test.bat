echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\mapq"
mkdir ".\test_data\mapq\bam"
mkdir ".\test_data\mapq\sam"
mkdir ".\test_data\mapq\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode mapq --output .\test_data\mapq\output

move .\test_data\mapq\output_mapq_0.bam .\test_data\mapq\bam\output_mapq_0.bam
move .\test_data\mapq\output_mapq_0.sam .\test_data\mapq\sam\output_mapq_0.sam
move .\test_data\mapq\output_mapq_1.bam .\test_data\mapq\bam\output_mapq_1.bam
move .\test_data\mapq\output_mapq_1.sam .\test_data\mapq\sam\output_mapq_1.sam
move .\test_data\mapq\output_mapq_2.bam .\test_data\mapq\bam\output_mapq_2.bam
move .\test_data\mapq\output_mapq_2.sam .\test_data\mapq\sam\output_mapq_2.sam
move .\test_data\mapq\output_mapq_3.bam .\test_data\mapq\bam\output_mapq_3.bam
move .\test_data\mapq\output_mapq_3.sam .\test_data\mapq\sam\output_mapq_3.sam
move .\test_data\mapq\output_mapq_4.bam .\test_data\mapq\bam\output_mapq_4.bam
move .\test_data\mapq\output_mapq_4.sam .\test_data\mapq\sam\output_mapq_4.sam
move .\test_data\mapq\output_mapq_5.bam .\test_data\mapq\bam\output_mapq_5.bam
move .\test_data\mapq\output_mapq_5.sam .\test_data\mapq\sam\output_mapq_5.sam
move .\test_data\mapq\output_mapq_6.bam .\test_data\mapq\bam\output_mapq_6.bam
move .\test_data\mapq\output_mapq_6.sam .\test_data\mapq\sam\output_mapq_6.sam
move .\test_data\mapq\output_mapq_7.bam .\test_data\mapq\bam\output_mapq_7.bam
move .\test_data\mapq\output_mapq_7.sam .\test_data\mapq\sam\output_mapq_7.sam
move .\test_data\mapq\output_mapq_8.bam .\test_data\mapq\bam\output_mapq_8.bam
move .\test_data\mapq\output_mapq_8.sam .\test_data\mapq\sam\output_mapq_8.sam
move .\test_data\mapq\output_mapq_9.bam .\test_data\mapq\bam\output_mapq_9.bam
move .\test_data\mapq\output_mapq_9.sam .\test_data\mapq\sam\output_mapq_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_0.bam --output .\test_data\mapq\stat\output_mapq_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_1.bam --output .\test_data\mapq\stat\output_mapq_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_2.bam --output .\test_data\mapq\stat\output_mapq_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_3.bam --output .\test_data\mapq\stat\output_mapq_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_4.bam --output .\test_data\mapq\stat\output_mapq_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_5.bam --output .\test_data\mapq\stat\output_mapq_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_6.bam --output .\test_data\mapq\stat\output_mapq_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_7.bam --output .\test_data\mapq\stat\output_mapq_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_8.bam --output .\test_data\mapq\stat\output_mapq_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq\bam\output_mapq_9.bam --output .\test_data\mapq\stat\output_mapq_9.stat

cd .\run_scripts\win\test_data