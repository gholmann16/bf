use std::fs;
use std::process;

// Accepts utf-8 string with only brainfuck characters, ignores everything else
pub fn compile(path: &str, file: Option<&str>) {
    let code: String = fs::read_to_string(path).unwrap();
    
    let mut output = String::new();
    output += "global _start\n";
    output += "section .bss\n";
    output += "memory: resb 65536\n"; // Declare memory space
    output += "section .text\n";
    output += "_start:\n";
    output += "xor rbx, rbx\n"; //Set initial position to 0

    let mut open: usize = 0;
    let mut close: usize = 0;
    let mut list: Vec<usize> = Vec::new();

    for c in code.chars() {
        match c {
            '<' => output += "sub bx, 1\n",
            '>' => output += "add bx, 1\n",
            '+' => output += "add byte[memory + rbx], 1\n",
            '-' => output += "sub byte[memory + rbx], 1\n",
            '[' => {
                open += 1;
                list.push(open);

                output += &format!("open_{}:\n", open.to_string());
                output += "cmp byte[memory + rbx], 0\n";
                output += &format!("jz close_{}\n", open.to_string());
            },
            ']' => {
                close += 1;
                let Some(jump) = list.pop() else { // If list is empty, we have run out of opening brackets to jmp to
                    println!("Bracket mismatch error. Closing bracket ']' found with no matching opening bracket '['");
                    process::exit(1);
                };

                output += &format!("jmp open_{}\n", jump.to_string()); // Always go back, because if it's 0 it will jump right back to close
                output += &format!("close_{}:\n", close.to_string());
            },
            '.' => {
                output += "mov rax, 1\n"; // Write syscall
                output += "mov rdi, 1\n"; // fd = stdout
                output += "mov rdx, 1\n"; // size = 1
                output += "mov rsi, memory\n";
                output += "add rsi, rbx\n";
                output += "syscall\n";
            },
            ',' => {
                output += "mov rax, 0\n"; // Read syscall
                output += "mov rdi, 0\n"; // fd = stdin
                output += "mov rdx, 1\n"; // size = 1
                output += "mov rsi, memory\n";
                output += "add rsi, rbx\n";
                output += "syscall\n";
            },
            _ => (),
        }
    }

    // Every bracket must have a match
    if open != close {
        println!("Not enough closing brackets ']'. Must have one per opening bracket '['.");
        process::exit(2);
    }

    // Exit boilerplate
    output += "movzx rdi, byte [memory + rbx]\n";
    output += "mov rax, 60\n";
    output += "syscall\n";

    let name = &path[..path.len()-2];

    let assembly = String::from(name) + "s";
    let object = String::from(name) + "o";

    fs::write(&assembly, output).unwrap();

    process::Command::new("nasm").arg("-felf64").arg(&assembly).status().expect("Nasm failed to assemble");
    fs::remove_file(&assembly).expect("Failed to remove assembly file");

    process::Command::new("ld").arg(&object).status().expect("Ld failed to link");
    fs::remove_file(&object).expect("Failed to remove object file");

    match file {
        None => (),
        Some(o) => fs::rename(&path, o).expect("Failed to rename output file"),
    }
}
