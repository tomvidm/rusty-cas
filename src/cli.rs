use std::io;
use std::io::{Write};

struct CLI_Input {
    string: String
}

pub fn run() {
    loop {
        let input = get_input();
        if input.string.trim() == "quit" {
            break;
        }
    }
}

fn get_input() -> CLI_Input {
    let mut input = String::new();
    print!(">> ");
    io::stdout().flush();
    io::stdin().read_line(&mut input);
    return CLI_Input{ string: input }
}