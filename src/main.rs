use std::process;
use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let filename = arg_collector();
    let filecontents = file_reader(filename);
    let tokens = convert(filecontents);
    let cppcode = translate(tokens);
    fs::write("out.cpp", cppcode).expect("Unable to write!");
}


fn arg_collector() -> String{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("NO FILE SPECIFIED!");
        process::exit(1);
    }
    let arg = args.remove(1);
    return arg;
}

fn file_reader(filename: String) -> String{
    if Path::new(&filename).is_file() {
        let raw = fs::read_to_string(filename).expect("Failed to read File!").replace("\r","");
        return raw;
    } else {
        eprintln!("Specified File was not found!");
        process::exit(1);
    }
}

fn convert(rawcode: String) -> Vec<usize> {
    let split = rawcode.split("\n").collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut l;
    for token in split {
        l = token.len();
        if l != 0 {
            tokens.push(l);
        }
    }
    return tokens;
}

fn translate(tokens: Vec<usize>) -> String{
    let mut outcode = vec!("#include <stdio.h>".to_string(),"#include <vector>".to_string(),"using namespace std;".to_string(),"#define CALL_VARIALBLE(name) goto name;".to_string(), "int main() {".to_string(), "vector<int> stack;".to_string(), "int A;".to_string(), "int B;".to_string());
    let mut linecount = 1;
    let mut prevop: usize = 0;
    let mut lastelement: String;
    for token in tokens {
        if prevop == 14 {
            lastelement = outcode.last().expect("This is bad!. Last element of code does not exist! This should be impossible!").to_string();
            outcode.push(format!("{}{};",lastelement, token));
            outcode.swap_remove(outcode.len()-2);
            prevop = 0;
        } else if prevop == 25{
            lastelement = outcode.last().expect("This is bad!. Last element of code does not exist! This should be impossible!").to_string();
            outcode.push(format!("{}{});",lastelement, token));
            outcode.swap_remove(outcode.len()-2);
            prevop = 0;
        } else {
            match token {
                9 => { // Inp operation. Pushes the ascii value of the first byte of stdin to the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("stack.push_back(getchar());".to_string());
                    linecount += 1;
                    prevop = token;
                }
                10 => { // Add operation. Pops the top two values from the stack and pushes their sum onto the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("B = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("stack.push_back(A + B);".to_string());
                    linecount += 1;
                    prevop = token;
                }
                11 => { // Sub operation. Pops value A from the stack and then value B. Pushes value B - A onto the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("B = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("stack.push_back(B - A);".to_string());
                    linecount += 1;
                    prevop = token;
                }
                12 => { // Dup operation. Duplicates the top value of the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("stack.push_back(stack.back());".to_string());
                    linecount += 1;
                    prevop = token;
                }
                13 => { // Cond operation. Pops the top value from the stack. If it is 0, skip the next instruction. If the instruction to be skipped is gotou or push, skip that instructions argument as well.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("if (A == 0) {".to_string());
                    linecount += 1;
                    prevop = token;
                }
                14 => { // Gotou operation. Sets the program counter to the value of the line under the instruction, skipping that line.
                    outcode.push("goto ".to_string());
                    linecount += 1;
                    prevop = token;
                }
                15 => { // Outn operation. Pops the top of the stack and outputs it as a number.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("printf(\"%d\", A);".to_string());
                    linecount += 1;
                    prevop = token;
                }
                16 => { // Outa operation. Pops the top of the stack and outputs its ascii value.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("putchar(A);".to_string());
                    linecount += 1;
                    prevop = token;
                }
                17 => { // Rol operation. Rotates the stack to the left: [ 7 6 5 ] -> [ 6 5 7 ]
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("rotate(stack.begin(), stack.begin()+1, stack.end());".to_string());
                    linecount += 1;
                    prevop = token;
                }
                18 => { // Swap operation. Swaps the top two values of the stack: [ 7 6 5 ] -> [ 6 7 5 ]
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("B = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("stack.push_back(B);".to_string());
                    outcode.push("stack.push_back(A);".to_string());
                    linecount += 1;
                    prevop = token;
                }
                20 => { // Mul operation. Pops the top two values from the stack and pushes their product onto the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("B = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("stack.push_back(A*B);".to_string());
                    linecount += 1;
                    prevop = token;
                }
                21 => { // Div operation. Pops value A from the stack and then value B. Pushes value B / A onto the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("B = stack.back();".to_string());
                    outcode.push("stack.pop_back();".to_string());
                    outcode.push("stack.push_back(B / A);".to_string()); // This is how length handeles division (round towards zero) asked the developer.
                    linecount += 1;
                    prevop = token;
                }
                23 => { // Pop operation. Pops the top value of the stack.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("stack.pop_back();".to_string());
                    linecount += 1;
                    prevop = token;
                }
                24 => { // Gotos operation. Pops the top of the stack and sets the program counter to it (indexed starting at 1).
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("A = stack.back();".to_string());
                    outcode.push("stack.pop_back()".to_string());
                    outcode.push("CALL_VARIALBLE(std::format(\"label{}\", A));".to_string());
                    linecount += 1;
                    prevop = token;
                }
                25 => { // Push operation. Pushes the value of the line under this instruction to the stack, skipping that line.
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("stack.push_back(".to_string());
                    linecount += 1;
                    prevop = token;
                }
                27 => { // Ror operation. Rotates the stack to the right: [ 7 6 5 ] -> [ 5 7 6 ]
                    outcode.push(format!("label{}:", linecount));
                    outcode.push("rotate(stack.begin(), stack.begin()-1, stack.end());".to_string());
                    linecount += 1;
                    prevop = token;
                }
                _ => {}
            }
    }
    }

    outcode.push("return 0;".to_string());
    outcode.push("}".to_string());
    //println!("{:?}", outcode);
    return outcode.join("\n");
}