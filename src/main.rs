use std::{fs, io};
use std::process::exit;

fn main() {
    let source_code = fs::read("C:\\Users\\metal\\Desktop\\untitled\\src\\brainCode.b")
        .expect("that shit broken dawg");//String::new();

    let mut tape:[i32;1000] = [0;1000];

    let mut pointer:usize = 0;

    let mut code_index = 0;
    while code_index < source_code.len(){
        match source_code[code_index] as char {
            '>' => {
                pointer+=1;
                code_index+=1;
            }
            '<' => {
                pointer-=1;
                code_index+=1;
            }
            '+' => {
                tape[pointer]+=1;
                code_index+=1;
            }
            '-' => {
                tape[pointer]-=1;
                code_index+=1;
            }
            '[' => {
                if tape[pointer] == 0 {
                    let mut count = 1;
                    while count != 0 {
                        code_index+=1;
                        if source_code[code_index] as char == '[' {
                            count += 1;
                        } else if source_code[code_index] as char == ']' {
                            count -= 1;
                        }
                    }
                } else {
                    code_index+=1;
                }
            }
            ']' => {
                if tape[pointer] != 0 {
                    let mut count = 1;
                    while count != 0 {
                        code_index-=1;
                        if source_code[code_index] as char == ']' {
                            count += 1;
                        } else if source_code[code_index] as char == '[' {
                            count -= 1;
                        }
                    }
                } else {
                    code_index+=1;
                }
            }
            '.' => {
                print!("{}",i32_to_char(tape[pointer]));
                code_index+=1;
            }
            ',' => {
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("you fucked up");
                tape[pointer] = user_input.as_bytes()[0] as i32;
                code_index+=1;
            }
            ' ' | '\n' | '\r' | '\t' => {
                code_index+=1;
            }
            '/' => {
                code_index+=1;
                if source_code[code_index] as char == '/' {
                    while code_index < source_code.len() && source_code[code_index] as char != '\n'{
                        code_index+=1;
                    }
                } else if source_code[code_index] as char == '*'{
                    code_index+=1;
                    while code_index < source_code.len() && source_code[code_index] as char != '*'{
                        code_index+=1;
                    }
                    code_index+=1;
                    if code_index < source_code.len() && source_code[code_index] as char == '/'{
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

