use std::io;
use std::io::{Write};
use symengine::{Engine};

pub fn run() {
    show_welcome();
    let mut engine = Engine::new();
    loop {
        let input = get_input();
        match input.trim() {
            "quit" => {
                show_farewell(); 
                break;
            },
            "help" => show_help(),
            _ => engine.interpret(&input)
        }
    }
}

fn parse_input(string: &String) {
    println!("I have no idea how to parse this stuff");
}

fn show_welcome() {
    println!("===================================");
    println!("Welcome to Rusty CAS");
    println!("Enjoy your time with this");
    println!("worthless garbage software!!!");
    println!("  help - show some helpful commands");
    println!("  quit - quit the program lol");
    println!("===================================");
}

fn show_help() {
    println!("   This program does nothing of value yet.");
}

fn show_farewell() {
    println!("   FAke software! Sad!");
}

fn get_input() -> String {
    let mut input = String::new();
    print!(">> ");
    io::stdout().flush();
    io::stdin().read_line(&mut input);
    return input
}