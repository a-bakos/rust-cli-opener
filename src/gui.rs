pub enum GUI {
    Start,
    Item2,
}

pub fn render(item: GUI) {
    match item {
        GUI::Start => println!("> START > "),
        GUI::Item2 => println!("something else"),
    }
}
