cd ..\..\..\

cargo build --release --bin segemehl_21_backend

cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/mixed_read/plot/benchmark_mixed_1000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/mixed_read/bam/output_mixed_1000.bam --output ./benchmark_data/mixed_read/stat/output_mixed_1000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/mixed_read/plot/benchmark_mixed_10000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/mixed_read/bam/output_mixed_10000.bam --output ./benchmark_data/mixed_read/stat/output_mixed_10000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/mixed_read/plot/benchmark_mixed_100000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/mixed_read/bam/output_mixed_100000.bam --output ./benchmark_data/mixed_read/stat/output_mixed_100000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/mixed_read/plot/benchmark_mixed_1000000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/mixed_read/bam/output_mixed_1000000.bam --output ./benchmark_data/mixed_read/stat/output_mixed_1000000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/mixed_read/plot/benchmark_mixed_10000000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/mixed_read/bam/output_mixed_10000000.bam --output ./benchmark_data/mixed_read/stat/output_mixed_10000000.stat
cmdbench --iterations 10 --print-averages --save-plot=./benchmark_data/mixed_read/plot/benchmark_mixed_100000000.png --save-plot-size 30 10 ./target/release/segemehl_21_backend.exe --input ./benchmark_data/mixed_read/bam/output_mixed_100000000.bam --output ./benchmark_data/mixed_read/stat/output_mixed_100000000.stat

cd .\run_scripts\win\benchmark_data\