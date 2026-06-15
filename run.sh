# go through all the configs in the ./configs directory and run them all
# python spits out result in the current working directory
for filename in $(ls configs); do
    CONFIG_PATH=$(pwd)/configs/$filename ./target/release/benchmark.exe | python stats.py ${filename%.*}
done