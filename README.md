# cmd-wrapper
> a command wrapper to modify command line arguments list.

## Motivation
When we are compiling a program,
in some case, we need to add or remove some compiling flags to make it works.  
For example, [hipcc](https://github.com/ROCm-Developer-Tools/HIP/tree/develop/bin) compiler
does not accept the two same static libraries (e.g. `hipcc ./libx.a ./libx.a -o a.out` under hip 3.9.1 in our case),
while this is common for [CMake cycle dependency](https://cmake.org/cmake/help/v3.2/command/target_link_libraries.html).
Then we need to write a compiler wrapper script to remove the duplicated static library in command line arguments list.
The initial idea of **cmd-wrapper** was for this purpose.

## Build
```bash
git clone https://github.com/genshen/cmd-wrapper.git
cd cmd-wrapper
cargo build
```

## Documentation
### set main command
Set the main command, we can use env `WRAPPED_CMD`.
e.g.
```bash
WRAPPED_CMD=clang cmd-wrapper --version
# => clang --version
```

### remove duplicated arguments
By using env `WRAPPED_REMOVE_DUP_ARGS`, we can remove the duplicated arguments and only keep the first one.
```bash
WRAPPED_CMD=ls WRAPPED_REMOVE_DUP_ARGS=-a cmd-wrapper -a -a -l
# => ls -a -l
```
In above example, the origin command is `ls -a -a -l`, which has two `-a` arguments in it.
We can remove the duplicated `-a` argument.

We can also remove two or more groups of duplicated arguments by using colon in env `WRAPPED_REMOVE_DUP_ARGS`:
```bash
WRAPPED_CMD=cat WRAPPED_REMOVE_DUP_ARGS=a.txt:b.txt \
    cmd-wrapper a.txt b.txt c.txt b.txt a.txt
# => cat a.txt b.txt c.txt
```

### remove arguments
todo:

### prepend arguments
By specifying ing env `WRAPPED_PREPEND_ARGS`, we can prepend some common line arguments into the origin arguments list.
Two or more prepending arguments are separated by colon (`:).
```bash
WRAPPED_CMD=clang WRAPPED_PREPEND_ARGS=-g:-O2 cmd-wrapper main.c -o main
# => clang -g -O2 main.c -o main
```

**Prepending condition**:
we can also use env `WRAPPED_PREPEND_IF` to control the condition of adding arguments specified by `WRAPPED_PREPEND_ARGS`.  
The detailed rules are listed as following:
- If `WRAPPED_PREPEND_ARGS` is specified, it must be a string of regex expression.
  The regex is matched with the origin arguments list. 
  If at least one argument matches the regex, the prepending condition will be set to true 
  and cmd-wrapper will prepend arguments specified by env `WRAPPED_PREPEND_ARGS` to the origin arguments list.
- If env `WRAPPED_PREPEND_IF` is empty, the prepending condition would always be set to true.
  (always prepending arguments in `WRAPPED_PREPEND_ARGS` into the origin arguments list.)
e.g. 

```bash
## the origin arguments list matches the regex in ${WRAPPED_PREPEND_IF}.
WRAPPED_CMD=clang++ WRAPPED_PREPEND_ARGS=-g:-O2 \
  WRAPPED_PREPEND_IF="(\w+)\.cc\$" cmd-wrapper main.cc -o main
# => clang++ -g -O2 main.cc -o main
```
```bash
## the origin arguments list does not match the regex in ${WRAPPED_PREPEND_IF}.
WRAPPED_CMD=clang++ WRAPPED_PREPEND_ARGS=-g:-O2 \
  WRAPPED_PREPEND_IF="(\w+)\.cc\$" cmd-wrapper main.cpp -o main
# =>  clang++ main.cpp -o main
```

### debug mode
use `--debug` to print the full arguments list generated by cmd-wrapper.
```bash
WRAPPED_CMD=clang++ WRAPPED_PREPEND_ARGS=-g:-O2 WRAPPED_PREPEND_IF="(\w+)\.cc\$" cmd-wrapper --debug main.cc -o main
full arguments: ["-g", "-O2", "main.cc", "-o", "main"]
```
