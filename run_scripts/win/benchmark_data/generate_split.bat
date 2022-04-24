cd ..\..\..\

mkdir ".\benchmark_data\"
mkdir ".\benchmark_data\split_read\"
mkdir ".\benchmark_data\split_read\bam\"
mkdir ".\benchmark_data\split_read\sam\"
mkdir ".\benchmark_data\split_read\stat\"
mkdir ".\benchmark_data\split_read\plot\"

cargo run --release --bin segemehl_21_test_generation -- 1000 --mode split --output .\benchmark_data\split_read\output
cargo run --release --bin segemehl_21_test_generation -- 10000 --mode split --output .\benchmark_data\split_read\output
cargo run --release --bin segemehl_21_test_generation -- 100000 --mode split --output .\benchmark_data\split_read\output
cargo run --release --bin segemehl_21_test_generation -- 1000000 --mode split --output .\benchmark_data\split_read\output
cargo run --release --bin segemehl_21_test_generation -- 10000000 --mode split --output .\benchmark_data\split_read\output
cargo run --release --bin segemehl_21_test_generation -- 100000000 --mode split --output .\benchmark_data\split_read\output

move .\benchmark_data\split_read\output_split_1000.bam .\benchmark_data\split_read\bam\output_split_1000.bam
move .\benchmark_data\split_read\output_split_1000.sam .\benchmark_data\split_read\sam\output_split_1000.sam
move .\benchmark_data\split_read\output_split_10000.bam .\benchmark_data\split_read\bam\output_split_10000.bam
move .\benchmark_data\split_read\output_split_10000.sam .\benchmark_data\split_read\sam\output_split_10000.sam
move .\benchmark_data\split_read\output_split_100000.bam .\benchmark_data\split_read\bam\output_split_100000.bam
move .\benchmark_data\split_read\output_split_100000.sam .\benchmark_data\split_read\sam\output_split_100000.sam
move .\benchmark_data\split_read\output_split_1000000.bam .\benchmark_data\split_read\bam\output_split_1000000.bam
move .\benchmark_data\split_read\output_split_1000000.sam .\benchmark_data\split_read\sam\output_split_1000000.sam
move .\benchmark_data\split_read\output_split_10000000.bam .\benchmark_data\split_read\bam\output_split_10000000.bam
move .\benchmark_data\split_read\output_split_10000000.sam .\benchmark_data\split_read\sam\output_split_10000000.sam
move .\benchmark_data\split_read\output_split_100000000.bam .\benchmark_data\split_read\bam\output_split_100000000.bam
move .\benchmark_data\split_read\output_split_100000000.sam .\benchmark_data\split_read\sam\output_split_100000000.sam

cd .\run_scripts\win\benchmark_data\