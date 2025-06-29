fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CODE 1 : Recoverable Errors with Result

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // // another way to handle the error
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == std::io::ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // CODE 2 : Shortcuts for Panic on Error: unwrap and expect

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // CODE 3 : A Shortcut for Propagating Errors: the ? Operator

    fn read_username_from_file_v2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // improved

    fn read_username_from_file_v3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;    

        Ok(username)
    }

    // this is also same but it misses on opportunity to explain error handling

    fn read_username_from_file_v4() -> Result<String, io::Error> {
        std::fs::read_to_string("hello.txt")
    }

    // CODE 4 : Where The ? Operator Can Be Used

    fn read_username_from_file_v5() -> Result<String, io::Error> {
        let mut greeting_file = File::open("hello.txt")?;
        let mut buf = String::new();
        greeting_file.read_to_string(&mut buf)?;
        return Ok(buf);
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let greeting_file = File::open("hello.txt")?;

    Ok(())

    

}
