/*
Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data
Here are some exercises you should now be equipped to solve:

1.
Given a list of integers, use a vector and return the median (when sorted,
the value in the middle position) and mode (the value that occurs most often;
a hash map will be helpful here) of the list.

2.
Convert strings to pig latin
The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay
Words that start with a vowel have hay added to the end instead (apple becomes apple-hay)
Keep in mind the details about UTF-8 encoding!

3.
Using a hash map and vectors, create a text interface to allow a user to add
employee names to a department in a company; for example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
department or all people in the company by department, sorted alphabetically.
The standard library API documentation describes methods that vectors,
strings, and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail, so it’s a perfect time to discuss error handling
We’ll do that next!
*/

use std::{collections::HashMap, io};

fn main() {
    // 1. Given a list of integers, use a vector and return the median (when sorted,
    //    the value in the middle position) and mode (the value that occurs most often;
    //    a hash map will be helpful here) of the list.

    let mut list = vec![
        1, 2, 3, 2, 4, 1, 1, 1, 1, 1, 3, 4, 6, 8, 9, 1, 1, 2, 4, 5, 2, 4, 6, 8, 9, 6,
    ];

    list.sort();

    println!("sorted list: {list:?}");

    let median: f64;
    let mid = list.len() / 2;

    if list.len() % 2 == 0 {
        // Even length: average of the two middle elements
        median = (list[mid - 1] as f64 + list[mid] as f64) / 2.0;
    } else {
        // Odd length: middle element
        median = list[mid] as f64;
    }

    println!("Median is: {}", median);

    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for &num in &list {
        // Iterate over references to avoid moving `numbers_for_mode`
        *frequencies.entry(num).or_insert(0) += 1;
    }

    // Find the mode(s)
    let mut max_frequency = 0;
    let mut modes: Vec<i32> = Vec::new();

    for (&number, &count) in &frequencies {
        // Iterate over references in the hashmap
        if count > max_frequency {
            max_frequency = count;
            modes.clear(); // New max frequency found, clear previous modes
            modes.push(number);
        } else if count == max_frequency {
            modes.push(number); // Another number with the same max frequency
        }
    }

    if modes.len() == 1 {
        println!("Mode is: {}", modes[0]);
    } else {
        println!("Modes are: {:?}", modes);
    }

    // 2.
    // Convert strings to pig latin
    // The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay
    // Words that start with a vowel have hay added to the end instead (apple becomes apple-hay)
    // Keep in mind the details about UTF-8 encoding!

    const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";
    const VOWELS: &str = "aeiouy";

    fn get_first_letter(word: &String) -> char {
        for c in word.chars() {
            return c;
        }
        ' '
    }

    fn convert_word_to_piglatin(word: &String) -> String {
        let mut piglatin_word = String::new();
        let mut first_letter = get_first_letter(word);

        if (CONSONANTS.contains(first_letter)) {
            piglatin_word = format!(
                "{}-{}ay",
                word.chars().skip(1).collect::<String>(),
                first_letter
            );
        } else {
            piglatin_word = format!("{}-hay", word);
        }

        piglatin_word
    }

    let normal_sentence = String::from("hello world apple first I am a sentence").to_lowercase();

    let piglatin_sentence = normal_sentence
        .split_whitespace()
        .map(|word| convert_word_to_piglatin(&String::from(word)))
        .collect::<Vec<String>>()
        .join(" ");

    println!("Normal sentence: {}", normal_sentence);
    println!("Piglatin sentence: {}", piglatin_sentence);

    // 3.
    // Using a hash map and vectors, create a text interface to allow a user to add
    // employee names to a department in a company; for example, “Add Sally to Engineering”
    // or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
    // department or all people in the company by department, sorted alphabetically.
    // The standard library API documentation describes methods that vectors,
    // strings, and hash maps have that will be helpful for these exercises!

    let mut company_departments = HashMap::new();

    company_departments.insert(
        String::from("Engineering"),
        vec![String::from("Sally"), String::from("Amir")],
    );
    company_departments.insert(
        String::from("Sales"),
        vec![String::from("John"), String::from("Jane")],
    );

    println!("Company departments: {:?}", company_departments);

    loop {
        let mut user_input = String::new();

        println!("Enter a command: (Add <name> to <department> or List <department> or List all)");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();

        let command = user_input.split_whitespace().collect::<Vec<&str>>();

        println!("You entered: {:?}\n", command);

        let cmd = if let Some(command) = command.get(0) {
            *command
        } else {
            println!("Invalid command");
            continue;
        };

        match cmd {
            "Add" => {
                let name = if let Some(name) = command.get(1) {
                    *name
                } else {
                    println!("Invalid command");
                    continue;
                };

                let department = if let Some(department) = command.get(3) {
                    *department
                } else {
                    println!("Invalid command");
                    continue;
                };

                company_departments
                    .entry(department.to_string())
                    .or_insert(vec![])
                    .push(name.to_string());
            }
            "List" => {
                let department = if let Some(department) = command.get(1) {
                    *department
                } else {
                    println!("Invalid command");
                    continue;
                };

                match department {
                    "all" => {
                        for (department, people) in &company_departments {
                            println!("People in {}: {:?}", department, people);
                        }
                    }
                    department => {
                        println!(
                            "People in {}: {:?}",
                            department,
                            company_departments.get(&department.to_string())
                        );
                    }
                }
            }
            "Quit" => {
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
    }

    // Gemini's rust solution
    // {
    //     // 1. Define an Enum to represent the different commands
    //     //    This makes our command structure type-safe and explicit.
    //     #[derive(Debug)]
    //     enum Command {
    //         Add {
    //             name: String,
    //             department: String,
    //         },
    //         List {
    //             department: String, // Can be "all" or a specific department name
    //         },
    //         Quit,
    //     }
    //     // 2. Define an Enum for potential parsing errors
    //     //    This provides more specific feedback than just "Invalid command".
    //     #[derive(Debug)]
    //     enum ParseError {
    //         MissingCommand,
    //         UnknownCommand,
    //         MissingArgument(&'static str), // Indicates which argument is missing
    //         InvalidFormat(&'static str),   // For malformed commands like "Add name department"
    //     }
    //     // Implement a way to display the ParseError
    //     impl std::fmt::Display for ParseError {
    //         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //             match self {
    //                 ParseError::MissingCommand => write!(f, "Error: No command entered."),
    //                 ParseError::UnknownCommand => write!(
    //                     f,
    //                     "Error: Unknown command. Expected 'Add', 'List', or 'Quit'."
    //                 ),
    //                 ParseError::MissingArgument(arg) => {
    //                     write!(f, "Error: Missing required argument: {}", arg)
    //                 }
    //                 ParseError::InvalidFormat(msg) => {
    //                     write!(f, "Error: Invalid command format. {}", msg)
    //                 }
    //             }
    //         }
    //     }
    //     // 3. A dedicated function to parse the user's input string into a Command enum.
    //     //    This function returns a Result, allowing us to gracefully handle parsing failures.
    //     fn parse_command(input: &str) -> Result<Command, ParseError> {
    //         let mut parts = input.split_whitespace();
    //         // Get the first part, which should be the command verb
    //         let cmd_verb = parts.next().ok_or(ParseError::MissingCommand)?;
    //         match cmd_verb {
    //             "Add" => {
    //                 let name = parts.next().ok_or(ParseError::MissingArgument("name"))?;
    //                 let to_keyword = parts
    //                     .next()
    //                     .ok_or(ParseError::MissingArgument("'to' keyword"))?;
    //                 let department = parts
    //                     .next()
    //                     .ok_or(ParseError::MissingArgument("department"))?;
    //                 if to_keyword != "to" {
    //                     return Err(ParseError::InvalidFormat(
    //                         "Expected 'to' keyword after name. Format: Add <name> to <department>",
    //                     ));
    //                 }
    //                 Ok(Command::Add {
    //                     // .to_string() converts &str to owned String, needed for storing in HashMap
    //                     name: name.to_string(),
    //                     department: department.to_string(),
    //                 })
    //             }
    //             "List" => {
    //                 let department = parts
    //                     .next()
    //                     .ok_or(ParseError::MissingArgument("department or 'all'"))?;
    //                 Ok(Command::List {
    //                     department: department.to_string(),
    //                 })
    //             }
    //             "Quit" => Ok(Command::Quit),
    //             _ => Err(ParseError::UnknownCommand),
    //         }
    //     }
    //     let mut company_departments: HashMap<String, Vec<String>> = HashMap::new();
    //     println!("Company Department Manager");
    //     println!("Commands: ");
    //     println!("  Add <name> to <department>");
    //     println!("  List <department> (e.g., 'List Sales' or 'List all')");
    //     println!("  Quit");
    //     println!("----------------------------------");
    //     loop {
    //         let mut user_input = String::new();
    //         print!("\nEnter a command: "); // Using print! for no newline
    //         io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout"); // Ensure prompt is displayed
    //         io::stdin()
    //             .read_line(&mut user_input)
    //             .expect("Failed to read line");
    //         let user_input = user_input.trim();
    //         // 4. Match on the Result from parsing the command
    //         match parse_command(user_input) {
    //             Ok(command) => {
    //                 // 5. Match on the successfully parsed Command enum
    //                 match command {
    //                     Command::Add { name, department } => {
    //                         // Use references directly in println! before moving the owned Strings.
    //                         // `&name` and `&department` create temporary borrows for printing.
    //                         println!("Added {} to {}.", &name, &department);
    //                         company_departments
    //                             .entry(department) // `department` (the owned String) is moved here
    //                             .or_insert_with(Vec::new) // Use or_insert_with for lazy initialization
    //                             .push(name); // `name` (the owned String) is moved here
    //                     }
    //                     Command::List { department } => {
    //                         if department == "all" {
    //                             if company_departments.is_empty() {
    //                                 println!("No departments or people added yet.");
    //                             } else {
    //                                 for (dept_name, people) in &company_departments {
    //                                     println!("People in {}: {:?}", dept_name, people);
    //                                 }
    //                             }
    //                         } else {
    //                             match company_departments.get(&department) {
    //                                 Some(people) => {
    //                                     println!("People in {}: {:?}", department, people);
    //                                 }
    //                                 None => {
    //                                     println!("Department '{}' not found.", department);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     Command::Quit => {
    //                         println!("Exiting Company Department Manager. Goodbye!");
    //                         break;
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 // Handle parsing errors
    //                 println!("{}", e);
    //                 // Optionally print usage hint
    //                 match e {
    //                     ParseError::UnknownCommand => {
    //                         println!("Please use 'Add', 'List', or 'Quit'.")
    //                     }
    //                     ParseError::InvalidFormat(_) => {
    //                         println!("Hint: 'Add <name> to <department>' or 'List <department>'.")
    //                     }
    //                     _ => {} // No extra hint for other errors
    //                 }
    //             }
    //         }
    //     }
    // }


}
