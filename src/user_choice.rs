use colored::*;
use console::style;
use dialoguer::Select;

pub async fn get_user_choice() -> usize {
    println!("{}", "\nChoose option with key arrow and hit Enter.".bold());

    let selections = &[
        style("1. List all currency symbols").green().to_string(),
        style("2. Convert currency").blue().to_string(),
        style("3. Exit").red().to_string(),
    ];

    Select::new()
        .items(&selections[..])
        .default(0)
        .interact()
        .expect("Failed to get user choice")
}
