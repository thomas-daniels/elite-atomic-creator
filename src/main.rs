mod api;
mod date;
mod tournament;

use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let personal_api_token_untrimmed = if args.len() >= 2 {
        if let Ok(contents) = fs::read_to_string(&args[1]) {
            contents
        } else {
            eprintln!("Unable to read file {}", args[1]);
            return;
        }
    } else {
        println!("(Note: if you don't want to enter your API token every time, save it to a file and pass the filepath as first argument to the program.");
        println!("API token:");
        let mut buffer = String::new();
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            buffer
        } else {
            eprintln!("Unable to read token from stdin");
            return;
        }
    };
    let personal_api_token = personal_api_token_untrimmed.trim();

    match api::create_tournament(
        &api::new_http_client(),
        &tournament::elite_atomic_at(&date::datetime_of_next_tournament_after(&date::now())),
        personal_api_token,
    ) {
        Ok(id) => println!("Tournament created! https://lichess.org/tournament/{}", id),
        Err(e) => eprintln!("Error while creating tournament: {}", e),
    };
}
