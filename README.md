# cmd-wrapper
> a command wrapper to modify command arguments list.

## Motivation
When we are compiling a program,
in some case, we need to add or remove some compiling flags to make it works.  
For example, [hipcc](https://github.com/ROCm-Developer-Tools/HIP/tree/develop/bin) compiler
does not accept the two same static libraries (e.g. `hipcc ./libx.a ./libx.a -o a.out` and we use hip 3.9.1),
while this is common for [CMake cycle dependency](https://cmake.org/cmake/help/v3.2/command/target_link_libraries.html).
Then we need to write a compiler wrapper script to remove the duplicated static library in command line arguments list.
The initial idea of **cmd-wrapper** was for this purpose.
