#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate clap;
extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate termion;
extern crate time;
extern crate url;
extern crate open;
extern crate uuid;

use player::player_api::PlayerAPI;
use core::error::CliError;
use core::entities::Credentials;
use core::common::Authorize;
use std::env;
use std::process;
use clap::App;


pub mod cli;
pub mod player;
pub mod core;


fn main() -> Result<(), CliError> {
    let app: App = cli::build_cli();

    let matches = app.get_matches();

    let color = true;

    let client_id: String = match env::var("SPOTIFY_CLIENT_ID") {
        Ok(client_id) => client_id,
        Err(_) => {
            return Err(CliError::new("env SPOTIFY_CLIENT_ID was missing{}", color));
        }
    };

    let secret: String = match env::var("SPOTIFY_CLIENT_SECRET") {
        Ok(secret) => secret,
        Err(_) => {
            return Err(CliError::new("env SPOTIFY_CLIENT_SECRET was missing", color));
        }
    };

    let code: String = match env::var("SPOTIFY_CODE") {
        Ok(secret) => secret,
        Err(_) => {
            let code = Authorize::get_authorization(client_id);
            return process::exit(0x0001)
        }
    };



    let credentials = Credentials::new(client_id, secret, code);


    if let Some(matches) = matches.subcommand_matches("player") {

        let devices = match PlayerAPI::get_devices(credentials) {
            Ok(devices) => devices,
            Err(e) =>
                return Err(CliError::new(&format!("{}", e), color))
        };

        println!("{:}", devices);
    }

    Ok(())
}
