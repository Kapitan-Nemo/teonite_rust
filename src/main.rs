use colored::*;
use dotenv::dotenv;
use std::env;
use std::path::Path;

mod currency_codes;
use currency_codes::fetch_symbols;

mod exchange_rates;
use exchange_rates::fetch_exchange_rate;

mod user_choice;
use user_choice::get_user_choice;

mod error_handling;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        r#"                                                               
        88                      
        ,d                                          ""    ,d                
        88                                                88                
      MM88MMM  ,adPPYba,   ,adPPYba,   8b,dPPYba,   88  MM88MMM  ,adPPYba,  
        88    a8P_____88  a8"     "8a  88P'   `"8a  88    88    a8P_____88  
        88    8PP"""""""  8b       d8  88       88  88    88    8PP"""""""  
        88,   "8b,   ,aa  "8a,   ,a8"  88       88  88    88,   "8b,   ,aa  
        "Y888  `"Ybbd8"'   `"YbbdP"'   88       88  88    "Y888  `"Ybbd8"'
        "#
    );
    println!("{}", "💰 Welcome to the currency converter! 💰");

    // Check if .env file exists
    if !Path::new(".env").exists() {
        println!("{}", "Failed to find .env file".red());
        return Ok(());
    }

    // Load .env file
    dotenv().ok();

    // Check if API_KEY is set
    let api_key = env::var("API_KEY");
    if api_key.is_err() {
        println!(
            "{}",
            "API_KEY is not set. Please set it in the .env file.".red()
        );
        return Ok(());
    }
    let api_key = api_key.unwrap();

    loop {
        let choice = get_user_choice().await;

        match choice {
            0 => {
                let symbols = fetch_symbols(&api_key).await?;
                for (code, name) in symbols.supported_codes {
                    println!("{}: {}", code.green(), name.blue());
                }
            }
            1 => {
                let mut base = String::new();
                let mut target = String::new();
                let mut amount_str = String::new();

                println!(
                    "\nEnter the {} code (the currency you have):",
                    "base currency".blue()
                );
                println!("For example, {}, etc.", "USD, EUR, GBP, JPY".green());
                std::io::stdin().read_line(&mut base)?;

                println!(
                    "\nEnter the {} code (the currency you want to convert to):",
                    "target currency".blue()
                );
                println!("For example, {}, etc.", "USD, EUR, GBP, JPY".green());
                std::io::stdin().read_line(&mut target)?;

                let amount: f64;
                loop {
                    println!("\nEnter the {} to be converted:", "amount".blue());
                    amount_str.clear();
                    std::io::stdin().read_line(&mut amount_str)?;
                    match amount_str.trim().parse() {
                        Ok(num) => {
                            amount = num;
                            break;
                        }
                        Err(_) => {
                            println!(
                                "{}",
                                "\nYou must enter a number. (e.g. 20 or 20.20) Please try again."
                                    .red()
                            );
                            continue;
                        }
                    }
                }
                loop {
                    match fetch_exchange_rate(&base.trim(), &target.trim(), &api_key).await {
                        Ok(rate) => {
                            let converted_amount: f64 = rate * amount;
                            let output = format!(
                                "{} {} is {} {} at an exchange rate of {}",
                                amount.to_string().green(),
                                base.trim(),
                                format!("{:.2}", converted_amount).blue(),
                                target.trim(),
                                rate.to_string().red()
                            );

                            let line = "-".repeat(output.len());

                            println!("{}", line);
                            println!("{}", output);
                            println!("{}", line);
                            break;
                        }
                        Err(error) => {
                            println!("{}", format!("An error occurred: {}", error).red());
                            break;
                        }
                    }
                }
            }
            2 => {
                println!("{}", "\nHalt and Catch Fire. Goodbye!".green());
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
