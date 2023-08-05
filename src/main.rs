use std::{fs, io};
use std::process::exit;

fn main() {
    let source_code = fs::read("C:\\Users\\metal\\Desktop\\untitled\\src\\brainCode.b")
        .expect("that shit broken dawg");//String::new();

    let mut tape:[i32;1000] = [0;1000];
    let mut stack:Vec<usize> = Vec::new();
    let mut pointer:usize = 0;

    let mut code_index = 0;
    while code_index < source_code.len(){
        match source_code[code_index]{
            62 => { //>
                pointer+=1;
                code_index+=1;
            }
            60 => { // <
                pointer-=1;
                code_index+=1;
            }
            43 => { //+
                tape[pointer]+=1;
                code_index+=1;
            }
            45 => { // -
                tape[pointer]-=1;
                code_index+=1;
            }
            91 => { // [
                if tape[pointer] == 0 {
                    let mut count = 1;
                    while count != 0 {
                        code_index+=1;
                        if source_code[code_index] == 91 {
                            count += 1;
                        } else if source_code[code_index] == 93 {
                            count -= 1;
                        }
                    }
                } else {
                    stack.push(code_index+1);
                    code_index+=1;
                }
            }
            93 => { // ]
                if tape[pointer] != 0 {
                    code_index = stack.pop().unwrap();
                    stack.push(code_index);
                } else {
                    code_index+=1;
                    stack.pop();
                }
            }
            46 => {  // .
                print!("{}",i32_to_char(tape[pointer]));
                code_index+=1;
            }
            44 => { // ,
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("you fucked up");
                tape[pointer] = user_input.as_bytes()[0] as i32;
                code_index+=1;
            }
            32 | 10 | 13 | 9 => { // whitespace ' ' | '\n' | '\r' | '\t'
                code_index+=1;
            }
            47 => {
                code_index+=1;
                if source_code[code_index]  == 47 {
                    while code_index < source_code.len() && source_code[code_index]!= 10{
                        code_index+=1;
                    }
                } else if source_code[code_index] == 42{ // *
                    code_index+=1;
                    while code_index < source_code.len() && source_code[code_index] != 42{
                        code_index+=1;
                    }
                    code_index+=1;
                    if code_index < source_code.len() && source_code[code_index] == 47{
                        code_index+=1;
                    } else {
                        println!("\nunclosed multiline comment");
                        exit(-1);
                    }
                } else {
                    println!("\ninvalid character '{}' at position {}", source_code[code_index] as char, code_index);
                    exit(-1);
                }
                code_index+=1;
            }
            _ => {
                println!("\ninvalid character '{}' at position {}",source_code[code_index] as char, code_index);
                exit(-1)
            }
        }
    }
}

fn i32_to_char(number: i32) -> char{
    if number >= 0 && number < 128 {
        let number:u8 = number as u8;
        number as char
    } else {
        println!("value out of range for convertion");
        exit(-1);
    }
}

