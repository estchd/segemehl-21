cd ..\..\..\

mkdir ".\benchmark_data\"
mkdir ".\benchmark_data\mixed_read\"
mkdir ".\benchmark_data\mixed_read\bam\"
mkdir ".\benchmark_data\mixed_read\sam\"
mkdir ".\benchmark_data\mixed_read\stat\"
mkdir ".\benchmark_data\mixed_read\plot\"

cargo run --release --bin segemehl_21_test_generation -- 1000 --mode mixed --output .\benchmark_data\mixed_read\output
cargo run --release --bin segemehl_21_test_generation -- 10000 --mode mixed --output .\benchmark_data\mixed_read\output
cargo run --release --bin segemehl_21_test_generation -- 100000 --mode mixed --output .\benchmark_data\mixed_read\output
cargo run --release --bin segemehl_21_test_generation -- 1000000 --mode mixed --output .\benchmark_data\mixed_read\output
cargo run --release --bin segemehl_21_test_generation -- 10000000 --mode mixed --output .\benchmark_data\mixed_read\output
cargo run --release --bin segemehl_21_test_generation -- 100000000 --mode mixed --output .\benchmark_data\mixed_read\output

move .\benchmark_data\mixed_read\output_mixed_1000.bam .\benchmark_data\mixed_read\bam\output_mixed_1000.bam
move .\benchmark_data\mixed_read\output_mixed_1000.sam .\benchmark_data\mixed_read\sam\output_mixed_1000.sam
move .\benchmark_data\mixed_read\output_mixed_10000.bam .\benchmark_data\mixed_read\bam\output_mixed_10000.bam
move .\benchmark_data\mixed_read\output_mixed_10000.sam .\benchmark_data\mixed_read\sam\output_mixed_10000.sam
move .\benchmark_data\mixed_read\output_mixed_100000.bam .\benchmark_data\mixed_read\bam\output_mixed_100000.bam
move .\benchmark_data\mixed_read\output_mixed_100000.sam .\benchmark_data\mixed_read\sam\output_mixed_100000.sam
move .\benchmark_data\mixed_read\output_mixed_1000000.bam .\benchmark_data\mixed_read\bam\output_mixed_1000000.bam
move .\benchmark_data\mixed_read\output_mixed_1000000.sam .\benchmark_data\mixed_read\sam\output_mixed_1000000.sam
move .\benchmark_data\mixed_read\output_mixed_10000000.bam .\benchmark_data\mixed_read\bam\output_mixed_10000000.bam
move .\benchmark_data\mixed_read\output_mixed_10000000.sam .\benchmark_data\mixed_read\sam\output_mixed_10000000.sam
move .\benchmark_data\mixed_read\output_mixed_100000000.bam .\benchmark_data\mixed_read\bam\output_mixed_100000000.bam
move .\benchmark_data\mixed_read\output_mixed_100000000.sam .\benchmark_data\mixed_read\sam\output_mixed_100000000.sam

cd .\run_scripts\win\benchmark_data\