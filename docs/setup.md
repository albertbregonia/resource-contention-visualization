# Setup 
1. Create a virutal environment (optional)
```bash
python -m venv <virtual environment directory name>
source <path to virtual environment interpreter>
```
2. Install Python dependencies
```bash
# i used pip, you may use others so long as they have matplotlib
pip install -r requirements.txt
```
1. Build the executable for your target platform with:
```bash
cargo build --release
```
You may also use the feature flag `log_contention` to see the maximum channel queue length printed to `stderr` 
```bash
cargo build --release --features log_contention
```
2. Copy the executable to the root of the repository:
```bash
cp ./target/release/benchmark.exe .
```
3. Setup complete. See [Usage](../README.md#usage) section for further instructon.