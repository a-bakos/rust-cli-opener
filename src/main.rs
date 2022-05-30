use std::collections::HashMap;
use std::io;

fn main() {
    // Resource library - this will eventually be in JSON
    let mut lib: HashMap<String, String> = HashMap::new();
    lib.insert(
        "buzz".to_string(),
        "https://members.buzzgym.co.uk/site/capacity".to_string(),
    );
    lib.insert("cmd".to_string(), "c:/Windows/System32/cmd.exe".to_string());
    lib.insert("note".to_string(), "notepad.exe".to_string());

    //println!("{:?}", lib.get("buzz").is_some());
    //println!("{:?}", lib.get("cmd").is_some());
    //println!("{:?}", lib.get("note").is_some());

    loop {
        println!(">> START");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to user input");

        // Validate user input
        let user_input: String = match user_input.trim().to_lowercase().parse() {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        // Exit condition
        if user_input == "exit" || user_input == "/e" {
            break;
        }

        // Look user input up in resource library and open if in there
        if lib.get(&user_input).is_some() {
            let open_this: &str = lib.get(&user_input).unwrap();
            if open::that(open_this).is_ok() {
                println!("OPENED");
                continue;
            }
        } else {
            println!("NOPE!");
            continue;
        }
    }
}
