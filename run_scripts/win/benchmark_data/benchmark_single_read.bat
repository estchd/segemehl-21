cd ../../../

cargo run --release --bin segemehl_21_test_generation -- 1000 --output ./benchmark_data/single_read/output_1000
cargo run --release --bin segemehl_21_test_generation -- 10000 --output ./benchmark_data/single_read/output_10000
cargo run --release --bin segemehl_21_test_generation -- 100000 --output ./benchmark_data/single_read/output_100000
cargo run --release --bin segemehl_21_test_generation -- 1000000 --output ./benchmark_data/single_read/output_1000000
cargo run --release --bin segemehl_21_test_generation -- 10000000 --output ./benchmark_data/single_read/output_10000000
cargo run --release --bin segemehl_21_test_generation -- 100000000 --output ./benchmark_data/single_read/output_100000000

mv ./benchmark_data/single_read/output_1000.bam ./benchmark_data/single_read/bam/output_1000.bam
mv ./benchmark_data/single_read/output_1000.sam ./benchmark_data/single_read/sam/output_1000.sam
mv ./benchmark_data/single_read/output_10000.bam ./benchmark_data/single_read/bam/output_10000.bam
mv ./benchmark_data/single_read/output_10000.sam ./benchmark_data/single_read/sam/output_10000.sam
mv ./benchmark_data/single_read/output_100000.bam ./benchmark_data/single_read/bam/output_100000.bam
mv ./benchmark_data/single_read/output_100000.sam ./benchmark_data/single_read/sam/output_100000.sam
mv ./benchmark_data/single_read/output_1000000.bam ./benchmark_data/single_read/bam/output_1000000.bam
mv ./benchmark_data/single_read/output_1000000.sam ./benchmark_data/single_read/sam/output_1000000.sam
mv ./benchmark_data/single_read/output_10000000.bam ./benchmark_data/single_read/bam/output_10000000.bam
mv ./benchmark_data/single_read/output_10000000.sam ./benchmark_data/single_read/sam/output_10000000.sam
mv ./benchmark_data/single_read/output_100000000.bam ./benchmark_data/single_read/bam/output_100000000.bam
mv ./benchmark_data/single_read/output_100000000.sam ./benchmark_data/single_read/sam/output_100000000.sam

cargo build --release --bin segemehl_21_backend

cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/single_read/plot/benchmark_1000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/single_read/bam/output_1000.bam --output ./benchmark_data/single_read/stat/output_1000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/single_read/plot/benchmark_10000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/single_read/bam/output_10000.bam --output ./benchmark_data/single_read/stat/output_10000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/single_read/plot/benchmark_100000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/single_read/bam/output_100000.bam --output ./benchmark_data/single_read/stat/output_100000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/single_read/plot/benchmark_1000000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/single_read/bam/output_1000000.bam --output ./benchmark_data/single_read/stat/output_1000000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/single_read/plot/benchmark_10000000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/single_read/bam/output_10000000.bam --output ./benchmark_data/single_read/stat/output_10000000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/single_read/plot/benchmark_100000000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/single_read/bam/output_100000000.bam --output ./benchmark_data/single_read/stat/output_100000000.stat