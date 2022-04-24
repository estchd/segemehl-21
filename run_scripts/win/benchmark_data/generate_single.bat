cd ..\..\..\

mkdir ".\benchmark_data\"
mkdir ".\benchmark_data\single_read\"
mkdir ".\benchmark_data\single_read\bam\"
mkdir ".\benchmark_data\single_read\sam\"
mkdir ".\benchmark_data\single_read\stat\"
mkdir ".\benchmark_data\single_read\plot\"

cargo run --release --bin segemehl_21_test_generation -- 1000 --mode single --output .\benchmark_data\single_read\output_1000
cargo run --release --bin segemehl_21_test_generation -- 10000 --mode single --output .\benchmark_data\single_read\output_10000
cargo run --release --bin segemehl_21_test_generation -- 100000 --mode single --output .\benchmark_data\single_read\output_100000
cargo run --release --bin segemehl_21_test_generation -- 1000000 --mode single --output .\benchmark_data\single_read\output_1000000
cargo run --release --bin segemehl_21_test_generation -- 10000000 --mode single --output .\benchmark_data\single_read\output_10000000
cargo run --release --bin segemehl_21_test_generation -- 100000000 --mode single --output .\benchmark_data\single_read\output_100000000

move .\benchmark_data\single_read\output_1000.bam .\benchmark_data\single_read\bam\output_1000.bam
move .\benchmark_data\single_read\output_1000.sam .\benchmark_data\single_read\sam\output_1000.sam
move .\benchmark_data\single_read\output_10000.bam .\benchmark_data\single_read\bam\output_10000.bam
move .\benchmark_data\single_read\output_10000.sam .\benchmark_data\single_read\sam\output_10000.sam
move .\benchmark_data\single_read\output_100000.bam .\benchmark_data\single_read\bam\output_100000.bam
move .\benchmark_data\single_read\output_100000.sam .\benchmark_data\single_read\sam\output_100000.sam
move .\benchmark_data\single_read\output_1000000.bam .\benchmark_data\single_read\bam\output_1000000.bam
move .\benchmark_data\single_read\output_1000000.sam .\benchmark_data\single_read\sam\output_1000000.sam
move .\benchmark_data\single_read\output_10000000.bam .\benchmark_data\single_read\bam\output_10000000.bam
move .\benchmark_data\single_read\output_10000000.sam .\benchmark_data\single_read\sam\output_10000000.sam
move .\benchmark_data\single_read\output_100000000.bam .\benchmark_data\single_read\bam\output_100000000.bam
move .\benchmark_data\single_read\output_100000000.sam .\benchmark_data\single_read\sam\output_100000000.sam

cd .\run_scripts\win\benchmark_data\