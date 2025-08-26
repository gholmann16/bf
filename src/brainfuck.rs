use std::fs;

// Accepts utf-8 string with only brainfuck characters, ignores everything else
// Returns true if it compiles, false if it doesn't
// As for io errors, just crash, because this isn't a user facing project
pub fn compile(filename: &str) -> bool {
    let code: String = fs::read_to_string(filename).unwrap();
    let newname = &filename[..filename.len()-3];
    println!("{}", newname);
    // let mut output = File::create()
    // for c in code.chars() {
        
    // }
    return true;
}

