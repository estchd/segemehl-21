echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\mapq_per_ref"
mkdir ".\test_data\mapq_per_ref\bam"
mkdir ".\test_data\mapq_per_ref\sam"
mkdir ".\test_data\mapq_per_ref\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode mapq_per_ref --output .\test_data\mapq_per_ref\output

move .\test_data\mapq_per_ref\output_mapq_per_ref_0.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_0.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_0.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_0.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_1.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_1.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_1.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_1.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_2.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_2.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_2.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_2.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_3.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_3.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_3.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_3.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_4.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_4.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_4.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_4.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_5.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_5.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_5.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_5.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_6.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_6.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_6.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_6.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_7.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_7.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_7.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_7.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_8.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_8.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_8.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_8.sam
move .\test_data\mapq_per_ref\output_mapq_per_ref_9.bam .\test_data\mapq_per_ref\bam\output_mapq_per_ref_9.bam
move .\test_data\mapq_per_ref\output_mapq_per_ref_9.sam .\test_data\mapq_per_ref\sam\output_mapq_per_ref_9.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_0.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_0.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_1.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_1.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_2.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_2.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_3.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_3.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_4.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_4.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_5.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_5.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_6.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_6.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_7.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_7.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_8.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_8.stat
cargo run --release --bin segemehl_21_backend -- --input .\test_data\mapq_per_ref\bam\output_mapq_per_ref_9.bam --output .\test_data\mapq_per_ref\stat\output_mapq_per_ref_9.stat

cd .\run_scripts\win\test_data