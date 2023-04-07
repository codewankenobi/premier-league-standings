#[macro_use]
extern crate prettytable;

use crate::table::table::*;

pub mod table;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_path("/Users/budison/developments/rust/premier-league-standings/.env").unwrap();
    let api_token = std::env::var("API_TOKEN").expect("API_TOKEN environment variable not found");

    let response: LeagueTable = reqwest::Client::new()
        .get("http://api.football-data.org/v4/competitions/PL/standings")
        .header("X-Auth-Token", api_token)
        .send()
        .await?
        .json()
        .await?;

    println!("{}", response);

    Ok(())
}
