use crate::tournament;
use reqwest;
use serde_json;

/// Defining the HttpClient type ensures that all
/// HTTP logic can be isolated in the api module:
/// should I ever decide to switch from reqwest to
/// another crate, then I only have to touch the api module.
pub type HttpClient = reqwest::Client;

/// Creates a new instance of the HttpClient type.
///
/// While it's a single function call, the purpose is
/// the same as for the definition of HttpClient, i.e.
/// isolating the HTTP/reqwest logic in one module.
pub fn new_http_client() -> HttpClient {
    HttpClient::new()
}

/// Sends the API request to create the tournament.
/// If successful, returns the ID of the newly created tournament.
/// If unsuccessful, returns an error.
pub fn create_tournament(
    http_client: &HttpClient,
    tour: &tournament::Info,
    api_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut resp = http_client
        .post("https://lichess.org/api/tournament")
        .form(&[
            ("name", tour.name),
            ("clockTime", &tour.clock_time.to_string()),
            ("clockIncrement", &tour.clock_increment.to_string()),
            ("minutes", &tour.minutes.to_string()),
            ("startDate", &tour.start_date.to_string()),
            ("variant", tour.variant),
            ("rated", &tour.rated.to_string()),
            ("berserkable", &tour.berserkable.to_string()),
            ("conditions.minRating.rating", &tour.min_rating.to_string()),
        ])
        .bearer_auth(api_token)
        .send()?;
    if resp.status().is_success() {
        let json: serde_json::Value = resp.json()?;
        match json.get("id") {
            Some(serde_json::Value::String(id)) => Ok(id.to_string()),
            _ => Err("Could not retrieve ID after creating tournament.".into()),
        }
    } else {
        Err(format!(
            "Unsuccessful API request:\n{}\n{}",
            resp.status(),
            resp.text()?
        )
        .into())
    }
}
