echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\ref_len"
mkdir ".\test_data\ref_len\bam"
mkdir ".\test_data\ref_len\sam"
mkdir ".\test_data\ref_len\stat"

cargo run --release --bin segemehl_21_test_generation -- --mode ref_len --output .\test_data\ref_len\output

move .\test_data\ref_len\output_ref_len.bam .\test_data\ref_len\bam\output_ref_len.bam
move .\test_data\ref_len\output_ref_len.sam .\test_data\ref_len\sam\output_ref_len.sam

cargo run --release --bin segemehl_21_backend -- --input .\test_data\ref_len\bam\output_ref_len.bam --output .\test_data\ref_len\stat\output_ref_len.stat

cd .\run_scripts\win\test_data