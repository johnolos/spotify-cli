use core::entities::{Token, Credentials, AuthorizationCode};
use hyper::header::{Accept, Authorization, Basic, Bearer, ContentType, Headers, UserAgent};
use reqwest::{Client, Response, Error};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use url::Url;
use std::process;
use uuid::Uuid;
use open;

pub trait Authorize {}

impl Authorize {
    pub fn get_authorization(client_id: String) -> Result<AuthorizationCode, Error> {
        let mut url = Url::parse("https://accounts.spotify.com/authorize").unwrap();

        let uuid = Uuid::new_v4().to_string();

        url.query_pairs_mut()
            .append_pair("client_id", &client_id)
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", "http://127.0.0.1")
            .append_pair("state", &uuid)
            .append_pair("scope", "\
             user-modify-playback-state \
             user-read-currently-playing \
             user-read-playback-state \
             user-library-modify \
             user-library-read \
             playlist-read-collaborative \
             playlist-read-private \
             playlist-modify-private \
             playlist-modify-public \
             user-read-recently-played \
             user-top-read")
            .append_pair("show_dialog", "false");

        match open::that(url.to_string()) {
            Ok(_) => println!("Check your browser:"),
            Err(_) => {
                println!("Open the following url in your browser of choice: {}", url.to_string());
            }
        };

        return Ok(AuthorizationCode::new("code".to_string()));
    }


    pub fn get_token(credentials: Credentials) -> Result<Response, Error> {

        let url = Url::parse("https://accounts.spotify.com/api/token").unwrap();

        let mut headers = Headers::new();
        headers.set(UserAgent::new("spot-cli/0.1.0"));
        headers.set(Accept::json());
        headers.set(ContentType::form_url_encoded());
        headers.set(Authorization(Basic {
            username: credentials.client_id.to_string(),
            password: Some(credentials.secret.to_string()),
        }));

        let client = Client::builder().default_headers(headers).build()?;

        println!("{}", "I got here");

        return client.post(&url.to_string()).body(format!("grant_type=authorization_code&code={}&redirect_uri=http://127.0.0.1", credentials.code)).send();
    }

    fn construct_headers(token: String) -> Headers {
        let mut headers = Headers::new();

        headers.set(UserAgent::new("spot-cli/0.1.0"));
        headers.set(ContentType::json());
        headers.set(Authorization(Bearer { token }));

        return headers;
    }

    pub fn get_request(
        url: Url,
        credentials: Credentials,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Response, Error> {

//        let response = Authorize::get_token(credentials)?;
//
//        println!("{:?}", response.text());
//
//        return process::exit(0x0001);

        let token: Token = Authorize::get_token(credentials)?.json::<Token>()?;

//        println!("{:?}", token);

        let headers = Authorize::construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build()?;

        if let Some(params) = params {
            return client.get(url).query(&params).send();
        }

        return client.get(url).send();
    }

    pub fn post_request(
        url: Url,
        credentials: Credentials,
        object: impl Serialize,
    ) -> Result<Response, Error> {

        let token: Token = Authorize::get_token(credentials)?.json::<Token>()?;

        let headers = Authorize::construct_headers(token.access_token);

        let client = Client::builder().default_headers(headers).build()?;

        return client
            .post(url)
            .body(serde_json::to_string(&object).unwrap())
            .send();
    }
}
