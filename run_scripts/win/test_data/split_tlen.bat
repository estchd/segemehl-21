echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\split_tlen"
mkdir ".\test_data\split_tlen\bam"
mkdir ".\test_data\split_tlen\sam"
mkdir ".\test_data\split_tlen\stat"

cargo build --release --bin segemehl_21_test_generation

.\target\release\segemehl_21_test_generation.exe --mode split_tlen --output .\test_data\split_tlen\output

move .\test_data\split_tlen\output_split_tlen_0.bam .\test_data\split_tlen\bam\output_split_tlen_0.bam
move .\test_data\split_tlen\output_split_tlen_0.sam .\test_data\split_tlen\sam\output_split_tlen_0.sam
move .\test_data\split_tlen\output_split_tlen_1.bam .\test_data\split_tlen\bam\output_split_tlen_1.bam
move .\test_data\split_tlen\output_split_tlen_1.sam .\test_data\split_tlen\sam\output_split_tlen_1.sam
move .\test_data\split_tlen\output_split_tlen_2.bam .\test_data\split_tlen\bam\output_split_tlen_2.bam
move .\test_data\split_tlen\output_split_tlen_2.sam .\test_data\split_tlen\sam\output_split_tlen_2.sam
move .\test_data\split_tlen\output_split_tlen_3.bam .\test_data\split_tlen\bam\output_split_tlen_3.bam
move .\test_data\split_tlen\output_split_tlen_3.sam .\test_data\split_tlen\sam\output_split_tlen_3.sam
move .\test_data\split_tlen\output_split_tlen_4.bam .\test_data\split_tlen\bam\output_split_tlen_4.bam
move .\test_data\split_tlen\output_split_tlen_4.sam .\test_data\split_tlen\sam\output_split_tlen_4.sam
move .\test_data\split_tlen\output_split_tlen_5.bam .\test_data\split_tlen\bam\output_split_tlen_5.bam
move .\test_data\split_tlen\output_split_tlen_5.sam .\test_data\split_tlen\sam\output_split_tlen_5.sam
move .\test_data\split_tlen\output_split_tlen_6.bam .\test_data\split_tlen\bam\output_split_tlen_6.bam
move .\test_data\split_tlen\output_split_tlen_6.sam .\test_data\split_tlen\sam\output_split_tlen_6.sam
move .\test_data\split_tlen\output_split_tlen_7.bam .\test_data\split_tlen\bam\output_split_tlen_7.bam
move .\test_data\split_tlen\output_split_tlen_7.sam .\test_data\split_tlen\sam\output_split_tlen_7.sam
move .\test_data\split_tlen\output_split_tlen_8.bam .\test_data\split_tlen\bam\output_split_tlen_8.bam
move .\test_data\split_tlen\output_split_tlen_8.sam .\test_data\split_tlen\sam\output_split_tlen_8.sam
move .\test_data\split_tlen\output_split_tlen_9.bam .\test_data\split_tlen\bam\output_split_tlen_9.bam
move .\test_data\split_tlen\output_split_tlen_9.sam .\test_data\split_tlen\sam\output_split_tlen_9.sam

cargo build --release --bin segemehl_21_backend

.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_0.bam --output .\test_data\split_tlen\stat\output_split_tlen_0.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_1.bam --output .\test_data\split_tlen\stat\output_split_tlen_1.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_2.bam --output .\test_data\split_tlen\stat\output_split_tlen_2.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_3.bam --output .\test_data\split_tlen\stat\output_split_tlen_3.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_4.bam --output .\test_data\split_tlen\stat\output_split_tlen_4.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_5.bam --output .\test_data\split_tlen\stat\output_split_tlen_5.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_6.bam --output .\test_data\split_tlen\stat\output_split_tlen_6.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_7.bam --output .\test_data\split_tlen\stat\output_split_tlen_7.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_8.bam --output .\test_data\split_tlen\stat\output_split_tlen_8.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\split_tlen\bam\output_split_tlen_9.bam --output .\test_data\split_tlen\stat\output_split_tlen_9.stat

cd .\run_scripts\win\test_data