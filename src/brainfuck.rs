use std::fs;
use std::process::Command;

// Accepts utf-8 string with only brainfuck characters, ignores everything else
// Returns true if it compiles, false if it doesn't
// As for io errors, just crash, because this isn't a user facing project

pub fn compile(filename: &str) -> bool {
    let code: String = fs::read_to_string(filename).unwrap();
    
    let mut output = String::new();
    output += "global _start\n";
    output += "section .bss\n";
    output += "memory: resb 65536\n"; // Declare memory space
    output += "section .text\n";
    output += "_start:\n";
    output += "xor rax, rax\n"; //Set initial position to 0

    let mut open: u32 = 0;
    let mut close: u32 = 0;
    let mut list: Vec<u32> = Vec::new();

    for c in code.chars() {
        match c {
            '<' => output += "sub ax, 1\n",
            '>' => output += "add ax, 1\n",
            '+' => output += "add byte[memory + rax], 1\n",
            '-' => output += "sub byte[memory + rax], 1\n",
            '[' => {
                open += 1;
                list.push(open);
                output += &format!("open_{}:\n", open.to_string());
                output += "cmp byte[memory + rax], 0\n";
                output += &format!("jz close_{}\n", open.to_string());
            },
            ']' => {
                close += 1;
                if close > open {
                    return false;
                }
                let jump = list.pop().expect("Bracket mismatch error, should never happen");
                output += &format!("jmp open_{}\n", jump.to_string());
                output += &format!("close_{}:\n", close.to_string()); 
            },
            _ => (),
        }
    }

    // Every bracket must have a match
    if open != close {
        return false;
    }

    // Exit boilerplate
    output += "movzx rdi, byte [memory + rax]\n";
    output += "mov rax, 60\n";
    output += "syscall\n";

    let name = &filename[..filename.len()-2];
    let newname = String::from(name) + "asm";
    let objname = String::from(name) + "o";
    fs::write(&newname, output).unwrap();

    Command::new("nasm").arg("-felf64").arg(&newname).status().expect("Nasm failed to assemble");
    Command::new("ld").arg(&objname).status().expect("Ld failed to link");
    return true;
}
