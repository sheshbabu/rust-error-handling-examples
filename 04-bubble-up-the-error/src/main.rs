use std::collections::HashMap;

fn main() {
    match get_current_date() {
        Ok(date) => println!("We've time travelled to {}!!", date),
        Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
    }
}

fn get_current_date() -> Result<String, reqwest::Error> {
    // Try changing the url to "https://postman-echo.com/time/objectzzzz"
    let url = "https://postman-echo.com/time/object";
    let result = reqwest::blocking::get(url);

    // Update this to use `?` operator
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let body = response.json::<HashMap<String, i32>>();

    // Update this to use `?` operator
    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };

    let date = json["years"].to_string();

    Ok(date)
}
