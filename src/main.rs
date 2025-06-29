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

mod init_5_1_struct_data_type;
mod init_5_2_a_program_using_structs;
mod init_5_3_method_syntax;

mod init_6_1_enum;
mod init_6_2_match_control_flow;
mod init_6_3_let_if_control_flow;

mod init_8_1_common_collections_vectors;
mod init_8_2_storing_string_in_utf_8;
mod init_8_3_hash_maps;
mod init_8_4_exercise;


mod init_9_1_panicing;
mod init_9_2_recoverable_errors;
mod init_9_3_to_panic_or_not_to_panic;

mod init_10_0_generics_intro;



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
