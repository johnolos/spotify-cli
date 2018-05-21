use core::entities::{Token, Credentials};
use hyper::header::{Accept, Authorization, Basic, Bearer, ContentType, Headers, UserAgent};
use reqwest::{Client, Response, Error};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use url::Url;
use std::process;
use std::io;
use std::mem;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use open;
use regex::Regex;


pub struct Authorize {
    token: Arc<RwLock<Option<Token>>>,
}

impl Authorize {
    pub fn new(token: Arc<RwLock<Option<Token>>>) -> Authorize {
        Authorize { token }
    }

    pub fn set_token(self, new_token: Token) {
        mem::replace(&mut self.token.write().unwrap().unwrap(), new_token);
    }

    pub fn get_authorization(client_id: &String) -> Result<String, Error> {
        let mut url = Url::parse("https://accounts.spotify.com/authorize").unwrap();

        let uuid = Uuid::new_v4().to_string();

        url.query_pairs_mut()
            .append_pair("client_id", client_id)
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
            Ok(_) => println!("Check your browser!"),
            Err(_) => {
                println!("Open the following url in your browser of choice: {}", url.to_string());
            }
        };

        println!("Once you're authenticated - copy url from the redirect and paste it here");

        print!(">>> ");

        let mut redirect_line = String::new();

        match io::stdin().read_line(&mut redirect_line) {
            Ok(_n) => {
                println!("{}", redirect_line)
            },
            Err(e) => {
                println!("error: {}", e);
                process::exit(0x0001)
            }
        }

        let re = Regex::new(r"\??((?P<query>\w*)=)(?P<value>[^&\n]*)?").unwrap();
        let mut query_params = HashMap::new();


        for caps in re.captures_iter(&redirect_line) {
            query_params.insert(caps["query"].to_string(), caps["value"].to_string());
        }

        let code = query_params.get("code").unwrap();
        let state = query_params.get("state").unwrap();

        println!("code={}", code);
        println!("state={}", state);

        if !state.eq(&uuid.to_string()) {
            eprintln!("Error: The received state did not match.");
        }

        Ok(code.to_string())
    }

    pub fn update_token(self, credentials: &Credentials) -> Result<(), Error> {

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

        let request = client
            .post(&url.to_string())
            .body(format!("\
             grant_type=authorization_code&\
             code={}&\
             redirect_uri=http://127.0.0.1", credentials.code))
            .send();

        let new_token = request?.json::<Token>()?;

        self.set_token(new_token);

        println!("Updated token");
        Ok(())
    }

    fn construct_headers(self, credentials: &Credentials) -> Result<Headers, Error> {
        self.update_token(&credentials);

        let token = self.token.read().unwrap().unwrap().access_token;


        let mut headers = Headers::new();

        headers.set(UserAgent::new("spot-cli/0.1.0"));
        headers.set(ContentType::json());
        headers.set(Authorization(Bearer { token }));

        return Ok(headers);
    }

    pub fn get_request(
        self,
        url: Url,
        credentials: &Credentials,
        params: Option<HashMap<&str, String>>,
    ) -> Result<Response, Error> {

        let headers = self.construct_headers(credentials)?;

        let client = Client::builder().default_headers(headers).build()?;

        if let Some(params) = params {
            return client.get(url).query(&params).send();
        }

        return client.get(url).send();
    }

    pub fn post_request(
        self,
        url: Url,
        credentials: &Credentials,
        object: impl Serialize,
    ) -> Result<Response, Error> {

        let headers = self.construct_headers(credentials)?;

        let client = Client::builder().default_headers(headers).build()?;

        return client
            .post(url)
            .body(serde_json::to_string(&object).unwrap())
            .send();
    }
}
