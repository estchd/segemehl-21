echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\split_gap"
mkdir ".\test_data\split_gap\bam"
mkdir ".\test_data\split_gap\sam"
mkdir ".\test_data\split_gap\stat"

cargo build --release --bin segemehl_21_test_generation

.\target\release\segemehl_21_test_generation.exe --mode split_gap --output .\test_data\split_gap\output

move .\test_data\split_gap\output_split_gap_0.bam .\test_data\split_gap\bam\output_split_gap_0.bam
move .\test_data\split_gap\output_split_gap_0.sam .\test_data\split_gap\sam\output_split_gap_0.sam
move .\test_data\split_gap\output_split_gap_1.bam .\test_data\split_gap\bam\output_split_gap_1.bam
move .\test_data\split_gap\output_split_gap_1.sam .\test_data\split_gap\sam\output_split_gap_1.sam
move .\test_data\split_gap\output_split_gap_2.bam .\test_data\split_gap\bam\output_split_gap_2.bam
move .\test_data\split_gap\output_split_gap_2.sam .\test_data\split_gap\sam\output_split_gap_2.sam
move .\test_data\split_gap\output_split_gap_3.bam .\test_data\split_gap\bam\output_split_gap_3.bam
move .\test_data\split_gap\output_split_gap_3.sam .\test_data\split_gap\sam\output_split_gap_3.sam
move .\test_data\split_gap\output_split_gap_4.bam .\test_data\split_gap\bam\output_split_gap_4.bam
move .\test_data\split_gap\output_split_gap_4.sam .\test_data\split_gap\sam\output_split_gap_4.sam
move .\test_data\split_gap\output_split_gap_5.bam .\test_data\split_gap\bam\output_split_gap_5.bam
move .\test_data\split_gap\output_split_gap_5.sam .\test_data\split_gap\sam\output_split_gap_5.sam
move .\test_data\split_gap\output_split_gap_6.bam .\test_data\split_gap\bam\output_split_gap_6.bam
move .\test_data\split_gap\output_split_gap_6.sam .\test_data\split_gap\sam\output_split_gap_6.sam
move .\test_data\split_gap\output_split_gap_7.bam .\test_data\split_gap\bam\output_split_gap_7.bam
move .\test_data\split_gap\output_split_gap_7.sam .\test_data\split_gap\sam\output_split_gap_7.sam
move .\test_data\split_gap\output_split_gap_8.bam .\test_data\split_gap\bam\output_split_gap_8.bam
move .\test_data\split_gap\output_split_gap_8.sam .\test_data\split_gap\sam\output_split_gap_8.sam
move .\test_data\split_gap\output_split_gap_9.bam .\test_data\split_gap\bam\output_split_gap_9.bam
move .\test_data\split_gap\output_split_gap_9.sam .\test_data\split_gap\sam\output_split_gap_9.sam

cargo build --release --bin segemehl_21_backend

.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_0.bam --output .\test_data\split_gap\stat\output_split_gap_0.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_1.bam --output .\test_data\split_gap\stat\output_split_gap_1.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_2.bam --output .\test_data\split_gap\stat\output_split_gap_2.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_3.bam --output .\test_data\split_gap\stat\output_split_gap_3.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_4.bam --output .\test_data\split_gap\stat\output_split_gap_4.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_5.bam --output .\test_data\split_gap\stat\output_split_gap_5.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_6.bam --output .\test_data\split_gap\stat\output_split_gap_6.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_7.bam --output .\test_data\split_gap\stat\output_split_gap_7.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_8.bam --output .\test_data\split_gap\stat\output_split_gap_8.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_gap\bam\output_split_gap_9.bam --output .\test_data\split_gap\stat\output_split_gap_9.stat

cd .\run_scripts\win\test_data