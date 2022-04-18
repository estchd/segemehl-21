call ./benchmark_single_read.bat > ../../../benchmark_data/single_read/benchmark_stdout.txt
cd ./run_scripts/win/benchmark_data
call ./benchmark_split_read.bat > ../../../benchmark_data/split_read/benchmark_stdout.txt
cd ./run_scripts/win/benchmark_data
call ./benchmark_mixed_read.bat > ../../../benchmark_data/mixed_read/benchmark_stdout.txt
