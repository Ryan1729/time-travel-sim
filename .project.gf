[gdb]
path=./rust-gdb

[commands]
Compile time-travel-sim=shell cargo b --bin time-travel-sim --profile debugging
Run time-travel-sim=file target/debugging/time-travel-sim;run&