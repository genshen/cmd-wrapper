## case1: use `WRAPPED_FILE`
```bash
WRAPPED_FILE=./wrapper.example.toml ./target/debug/cmd-wrapper --debug -O3 -O3  main.cc -O3  -o main
> clang++ ["-g", "-c", "-O3", "main.cc", "-o", "main"]
```

## case 2
```bash
./target/debug/cmd-wrapper --debug -O3 -O3  main.cc -O3  -o main
> clang++ ["-O3", "-O3", "main.cc", "-O3", "-o", "main"] # nothing changes
```

## case 3: use default wrapper file
```bash
ln -s ./wrapper.example.toml ./wrapper.toml
./target/debug/cmd-wrapper --debug -O3 -O3  main.cc -O3  -o main
> clang++ ["-g", "-c", "-O3", "main.cc", "-o", "main"]
rm ./wrapper.toml
```

## case4: env overwrite
```bash
WRAPPED_FILE=./wrapper.example.toml \
WRAPPED_CMD=g++ \
WRAPPED_PREPEND_IF="(\w+)\.cpp\$" \
WRAPPED_REMOVE_DUP_ARGS="-lcxx" \
WRAPPED_PREPEND_ARGS=-I./:-O0 \
  ./target/debug/cmd-wrapper --debug -lcxx -lcxx main.cpp -lcxx  -o main
> g++ ["-I./", "-O0", "-lcxx", "main.cpp", "-o", "main"]
```
