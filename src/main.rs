use std::io;

mod init_1_2_hello;
mod init_1_3_formatted_print;
mod init_2_0_guessing_game;
mod init_3_1_variable_and_mutability;
mod init_3_2_data_types;
mod init_3_3_functions;
mod init_3_4_comments;
mod init_3_5_control_flow;

mod init_4_1_ownership;
mod init_4_3_the_slice_type;
fn main() {
    let mut program_code = String::new();

    println!("Welcome to the Rust programming language!");
    println!("");
    println!("Please enter your choice:");
    println!("1. Guessing game");

    io::stdin()
        .read_line(&mut program_code)
        .expect("Failed to read line");

    match program_code.trim() {
        "1" => init_2_0_guessing_game::main(),
        _ => println!("Invalid input"),
    }
}
