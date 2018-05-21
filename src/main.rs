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
extern crate regex;

use player::player::PlayerAPI;
use core::error::CliError;
use core::entities::Credentials;
use core::common::Authorize;
use std::env;
use std::sync::{Arc, RwLock};
use std::process;
use clap::App;


pub mod cli;
pub mod player;
pub mod core;


fn main() -> Result<(), CliError> {
    let app: App = cli::build_cli();

    let matches = app.get_matches();

    let color = true;

    let ref client_id: String = match env::var("SPOTIFY_CLIENT_ID") {
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
            match Authorize::get_authorization(client_id) {
                Ok(code) => code,
                Err(_) => return Err(CliError::new("code was neither in envs nor able to retrieve it", color))
            }
        }
    };

    let ref credentials = Credentials::new(client_id.to_string(), secret, code);

    let ref authorize = Authorize::new(Arc::new(RwLock::new(None)));

    let ref player_api = PlayerAPI::new(authorize);


    if let Some(matches) = matches.subcommand_matches("player") {

        let devices = match player_api.get_devices(credentials) {
            Ok(devices) => devices,
            Err(e) =>
                return Err(CliError::new(&format!("{}", e), color))
        };

        println!("{:}", devices);
    }

    Ok(())
}