pub enum GUI {
    ProgramStart,
    LoopStart,
    OpenSuccess,
    OpenFail,
    NotFound,
}

pub fn render(item: GUI) {
    match item {
        GUI::ProgramStart => println!("> START > "),
        GUI::LoopStart => println!("What do you want to open?"),
        GUI::OpenSuccess => println!("Opened successfully!\n"),
        GUI::OpenFail => println!("Opening failed!\n"),
        GUI::NotFound => println!("Resource not found!\n"),
    }
}
