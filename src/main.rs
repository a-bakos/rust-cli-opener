fn main() {
    if open::that("https://members.buzzgym.co.uk/site/capacity").is_ok() {
        println!("opened");
    }
}
