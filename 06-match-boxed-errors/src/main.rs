use chrono::NaiveDate;
use std::collections::HashMap;

fn main() {
    match get_current_date() {
        Ok(date) => println!("We've time travelled to {}!!", date),
        Err(e) => {
            eprintln!("Oh noes, we don't know which era we're in! :(");
            if let Some(err) = e.downcast_ref::<reqwest::Error>() {
                eprintln!("Request Error: {}", err)
            } else if let Some(err) = e.downcast_ref::<chrono::format::ParseError>() {
                eprintln!("Parse Error: {}", err)
            }
        }
    }
}

fn get_current_date() -> Result<String, Box<dyn std::error::Error>> {
    // Try changing the url to "https://postman-echo.com/time/objectzzzz"
    let url = "https://postman-echo.com/time/object";
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;

    // Try changing the format to "{}-{}-{}z"
    let formatted_date = format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);
    let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")?;
    let date = parsed_date.format("%Y %B %d").to_string();

    Ok(date)
}
