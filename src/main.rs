mod brainfuck;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut source: Option<&str> = None;
    let mut output: Option<&str> = None;
    let mut assemble: bool = true;
    let mut x = 1;

    while x < args.len() {
        match args[x].as_str() {
            "--help" => help(),
            "-h" => help(),
            "-S" => assemble = false,
            "-o" => {
                x += 1; // Go to next argument
                if x == args.len() {
                    println!("Need to input a name to use -o");
                    return;
                }
                output = Some(&args[x]);
            },
            _ => source = Some(&args[x]),
        }
        x += 1;
    }

    match source {
        None => println!("Must input a file to compile"),
        Some(path) => brainfuck::compile(path, output, assemble),
    }
}

fn help() {
    println!("Compiler for brainfuck");
    println!("Usage");
    println!("bf [Options] [File]");
    println!("bf [File] [Options]");
    println!("Options:");
    println!("-h, --help\tSee all options");
    println!("-o\t\tWhat name to give to the output");
    println!("-S\t\tOutput compiled code, don't assemble");
    println!("All other arguments will become the file name.");
    println!("If you pass multiple files, the last one will be compiled.");

    process::exit(0);
}
