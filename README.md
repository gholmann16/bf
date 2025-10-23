# Brainfuck Compiler

Simple brainfuck compiler. Inputs brainfuck text and outputs an x86_64 elf formatted binary. Requires nasm and ld to be installed. Written fully in rust with no extra dependencies.

Remember when using input that the terminal stores the newline character and will only input a 0 if you provide it with EOF or a similar symbol

Uses similar command line options as gcc
