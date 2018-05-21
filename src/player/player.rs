use core::entities::{Credentials};
use core::common::Authorize;
use player::entities::DevicesResponse;
use reqwest::{Response, Error};
use url::Url;

pub struct PlayerAPI<'a> {
    authorize: &'a Authorize,
}

impl<'a> PlayerAPI<'a> {
    pub fn new(authorize: &'a Authorize) -> PlayerAPI<'a> {
        PlayerAPI { authorize }
    }

    pub fn get_devices(&self, credentials: &Credentials) -> Result<DevicesResponse, Error> {
        let url = Url::parse("https://api.spotify.com/v1/me/player/devices").unwrap();

        let mut response: Response = self.authorize.get_request(url, credentials, None)?;

        println!("{:?}", response.text().ok());

        return response.json::<DevicesResponse>();
    }
}
