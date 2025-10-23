use std::fs;
use std::process;

// Accepts utf-8 string with only brainfuck characters, ignores everything else
pub fn compile(path: &str, file: Option<&str>, assemble: bool) {
    let code: String = fs::read_to_string(path).unwrap();

    let mut output = String::new();
    output += "global _start\n";
    output += "section .bss\n";
    output += "m: resb 65536\n"; // Declare memory space
    output += "section .text\n";
    output += "_start:\n";
    output += "mov rbx, 0\n"; // Set mem pointer to memory

    let mut open: usize = 100; // skip o16, o32, o64 etc
    let mut list: Vec<usize> = Vec::new();

    for c in code.chars() {
        match c {
            '<' => output += "dec bx\n",
            '>' => output += "inc bx\n",
            '+' => output += "inc byte[m+rbx]\n",
            '-' => output += "dec byte[m+rbx]\n",
            '[' => {
                open += 1;
                list.push(open);
                output += &format!("jmp c{}\n", open.to_string());
                output += &format!("o{}:\n", open.to_string());
            },
            ']' => {
                let Some(jump) = list.pop() else { // If list is empty, we have run out of opening brackets to jmp to
                    println!("Bracket mismatch error. Closing bracket ']' found with no matching opening bracket '['");
                    process::exit(1);
                };

                output += &format!("c{}:\n", jump.to_string());
                output += "cmp byte[m+rbx], 0\n";
                output += &format!("jnz o{}\n", jump.to_string());

            },
            '.' => {
                output += "mov rax, 1\n"; // Write syscall
                output += "mov rdi, 1\n"; // fd = stdout
                output += "mov rdx, 1\n"; // size = 1
                output += "mov rsi, m\n"; // memory pointer
                output += "add rsi, rbx\n"; // memory offset
                output += "syscall\n";
            },
            ',' => {
                output += "mov rax, 0\n"; // Read syscall
                output += "mov rdi, 0\n"; // fd = stdin
                output += "mov rdx, 1\n"; // size = 1
                output += "mov rsi, m\n"; // memory pointer
                output += "add rsi, rbx\n"; // memory offset
                output += "syscall\n";
            },
            _ => (),
        }
    }

    // Every bracket must have a match
    if list.len() != 0 {
        println!("Not enough closing brackets ']'. Must have one per opening bracket '['.");
        process::exit(2);
    }

    // Exit boilerplate
    output += "mov rdi, 0\n";
    output += "mov rax, 60\n";
    output += "syscall\n";

    let name = &path[..path.len()-2];
    let mut to_be_renamed = String::from(name) + "s";
    fs::write(&to_be_renamed, output).unwrap();

    if assemble {
        let object = String::from(name) + "o";
        process::Command::new("nasm").arg("-felf64").arg(&to_be_renamed).status().expect("Nasm failed to assemble");
        fs::remove_file(&to_be_renamed).expect("Failed to remove assembly file");

        process::Command::new("ld").arg(&object).status().expect("Ld failed to link");
        fs::remove_file(&object).expect("Failed to remove object file");

        to_be_renamed = "a.out".to_string();
    }

    match file {
        None => (),
        Some(out) => fs::rename(&to_be_renamed, out).expect("Failed to rename output file"),
    }
}
