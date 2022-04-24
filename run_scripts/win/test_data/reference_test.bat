echo off

cd ..\..\..\

mkdir ".\test_data"
mkdir ".\test_data\reference"
mkdir ".\test_data\reference\bam"
mkdir ".\test_data\reference\sam"
mkdir ".\test_data\reference\stat"

cargo build --release --bin segemehl_21_test_generation

.\target\release\segemehl_21_test_generation.exe --mode ref --output .\test_data\reference\output

move .\test_data\reference\output_reference_0.bam .\test_data\reference\bam\output_reference_0.bam
move .\test_data\reference\output_reference_0.sam .\test_data\reference\sam\output_reference_0.sam
move .\test_data\reference\output_reference_1.bam .\test_data\reference\bam\output_reference_1.bam
move .\test_data\reference\output_reference_1.sam .\test_data\reference\sam\output_reference_1.sam
move .\test_data\reference\output_reference_2.bam .\test_data\reference\bam\output_reference_2.bam
move .\test_data\reference\output_reference_2.sam .\test_data\reference\sam\output_reference_2.sam
move .\test_data\reference\output_reference_3.bam .\test_data\reference\bam\output_reference_3.bam
move .\test_data\reference\output_reference_3.sam .\test_data\reference\sam\output_reference_3.sam
move .\test_data\reference\output_reference_4.bam .\test_data\reference\bam\output_reference_4.bam
move .\test_data\reference\output_reference_4.sam .\test_data\reference\sam\output_reference_4.sam
move .\test_data\reference\output_reference_5.bam .\test_data\reference\bam\output_reference_5.bam
move .\test_data\reference\output_reference_5.sam .\test_data\reference\sam\output_reference_5.sam
move .\test_data\reference\output_reference_6.bam .\test_data\reference\bam\output_reference_6.bam
move .\test_data\reference\output_reference_6.sam .\test_data\reference\sam\output_reference_6.sam
move .\test_data\reference\output_reference_7.bam .\test_data\reference\bam\output_reference_7.bam
move .\test_data\reference\output_reference_7.sam .\test_data\reference\sam\output_reference_7.sam
move .\test_data\reference\output_reference_8.bam .\test_data\reference\bam\output_reference_8.bam
move .\test_data\reference\output_reference_8.sam .\test_data\reference\sam\output_reference_8.sam
move .\test_data\reference\output_reference_9.bam .\test_data\reference\bam\output_reference_9.bam
move .\test_data\reference\output_reference_9.sam .\test_data\reference\sam\output_reference_9.sam

cargo build --release --bin segemehl_21_backend

.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_0.bam --output .\test_data\reference\stat\output_reference_0.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_1.bam --output .\test_data\reference\stat\output_reference_1.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_2.bam --output .\test_data\reference\stat\output_reference_2.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_3.bam --output .\test_data\reference\stat\output_reference_3.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_4.bam --output .\test_data\reference\stat\output_reference_4.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_5.bam --output .\test_data\reference\stat\output_reference_5.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_6.bam --output .\test_data\reference\stat\output_reference_6.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_7.bam --output .\test_data\reference\stat\output_reference_7.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_8.bam --output .\test_data\reference\stat\output_reference_8.stat
.\target\release\segemehl_21_backend.exe --input .\test_data\reference\bam\output_reference_9.bam --output .\test_data\reference\stat\output_reference_9.stat

cd .\run_scripts\win\test_data