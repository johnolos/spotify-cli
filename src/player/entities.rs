use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename="type")]
    pub type_of_device: String,
    pub volume_percent: Option<i8>,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = if self.is_active { "yes" } else { "no" };
        let restricted = if self.is_restricted { "yes" } else { "no" };

        writeln!(f, "\
         id:\t\t{:?}\n\
         active:\t\t{}\n\
         restricted:\t{}\n\
         name:\t\t{}\n\
         type:\t\t{}\n\
         volume:\t{:?}%\n",
        self.id,
        active,
        restricted,
        self.name,
        self.type_of_device,
        self.volume_percent)
    }
}

#[derive(Debug, Deserialize)]
pub struct DevicesResponse {
    pub devices: Vec<Device>
}

impl fmt::Display for DevicesResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for device in &self.devices {
            write!(f, "{}\n\n", device)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct ExternalID {
    pub key: String,
    pub value: String
}
#[derive(Debug, Deserialize)]
pub struct ExternalUrl {
    pub key: String,
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct TrackLink {
    pub external_urls: Vec<ExternalUrl>,
    pub href: String,
    pub id: String,
    #[serde(rename="type")]
    pub type_of_track: String,
    pub uri: String
}
#[derive(Debug, Deserialize)]
pub struct Restriction {
    pub reason: String
}
#[derive(Debug, Deserialize)]
pub struct Image {
    height: i16,
    url: String,
    width: i16
}

#[derive(Debug, Deserialize)]
pub struct AlbumSimplified {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub external_urls: Vec<ExternalUrl>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<Restriction>,
    #[serde(rename="type")]
    pub type_of_album: String,
    pub uri: String
}

#[derive(Debug, Deserialize)]
pub struct ArtistSimplified {
    external_urls: Vec<ExternalUrl>,
    href: String,
    id: String,
    name: String,
    #[serde(rename="type")]
    type_of_artist: String,
    uri: String
}

#[derive(Debug, Deserialize)]
pub struct TrackFull {
    pub album: AlbumSimplified,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: i8,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_ids: Vec<ExternalID>,
    pub external_urls: Vec<ExternalUrl>,
    pub href: String,
    pub id: String,
    pub is_playable: bool,
    pub linked_from: TrackLink,
    pub restrictions: Option<Restriction>,
    pub name: String,
    pub popularity: i8,
    pub preview_url: String,
    pub track_number: i8,
    #[serde(rename="type")]
    pub type_of_track: String,
    pub uri: String
}

#[derive(Debug, Deserialize)]
pub struct TrackSimplified {
    pub devices: Vec<Device>
}

#[derive(Debug, Deserialize)]
pub struct PlayingContext {
    pub device: Device,
    pub repeat_state: String,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: i64,
    pub progress_ms: i32,
    pub item: Option<TrackFull>
}

#[derive(Debug, Deserialize)]
pub struct Context {
    #[serde(rename="type")]
    pub type_of_context: String,
    pub href: String,
    pub external_urls: Vec<ExternalUrl>,
    pub uri: String
}