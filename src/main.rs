mod brainfuck;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut source: Option<&str> = None;
    let mut output: Option<&str> = None;

    for mut x in 1..args.len() {
        match args[x].as_str() {
            "--help" => help(),
            "-h" => help(),
            "-o" => {
                x = x + 1; // Go to next argument
                if x == args.len() {
                    println!("Need to input a name to use -o");
                }
                output = Some(&args[x]);
            },
            _ => source = Some(&args[x]),
        }
    }

    match source {
        None => {
            println!("Must input a file to compile");
            return;
        },
        Some(path) => {
            brainfuck::compile(path, output);
        },

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
    println!("All other arguments will become the file name.");
    println!("If you pass multiple files, the last one will be compiled.");

    process::exit(0);
}
