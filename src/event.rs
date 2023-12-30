use chrono::{Datelike, NaiveDateTime, Utc, Weekday};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use twox_hash::xxh3::hash64;

pub struct Event<'l> {
    pub name: String,
    pub date: NaiveDateTime,
    pub location: &'l Location,
    pub url: String,
    pub image: Option<Image>,
    pub bands: Vec<BandInfo>,
}

impl<'a> Event<'a> {
    pub fn new(
        name: String,
        date: NaiveDateTime,
        location: &'a Location,
        url: String,
        image_url: Option<String>,
    ) -> Self {
        let image = match image_url {
            None => None,
            Some(str) => Some(Image::new(str)),
        };

        let bands: Vec<BandInfo> = Vec::new();

        Self {
            name,
            date,
            location,
            url,
            image,
            bands,
        }
    }

    pub fn set_image(&mut self, image_url: String) {
        self.image = Some(Image::new(image_url));
    }

    pub fn add_band(&mut self, name: String) {
        self.bands.push(BandInfo {
            name,
            genre: None,
            website: None,
            metallum_link: None,
            spirit_link: None,
        });
    }
}

impl<'a> Serialize for Event<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Event", 6)?;
        s.serialize_field("name", self.name.replace("\"", "&quot;").as_str())?;
        s.serialize_field("name_html", html_escape::encode_safe(&self.name).as_ref())?;

        let weekday = match self.date.weekday() {
            Weekday::Mon => "Mo.",
            Weekday::Tue => "Di.",
            Weekday::Wed => "Mi.",
            Weekday::Thu => "Do.",
            Weekday::Fri => "Fr.",
            Weekday::Sat => "Sa.",
            Weekday::Sun => "So.",
        };
        let date: String;
        if self.date.year() == Utc::now().year() {
            date = self.date.format(" %d.%m.").to_string();
        } else {
            date = self.date.format(" %d.%m. %Y").to_string();
        }
        s.serialize_field("date", &(weekday.to_owned() + &date))?;
        s.serialize_field("date_slug", &self.date.format("%Y%m%d").to_string())?;

        s.serialize_field("location", &self.location)?;
        s.serialize_field("url", &self.url)?;
        s.serialize_field("image", &self.image)?;
        s.end()
    }
}

pub struct Location {
    pub slug: String,
    pub name: String,
    pub website: String,
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Self {
            slug: self.slug.clone(),
            name: self.name.clone(),
            website: self.website.clone(),
        }
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Location", 3)?;
        s.serialize_field("slug", &self.slug)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("website", &self.website)?;
        s.end()
    }
}

pub struct Image {
    pub remote_url: String,
    pub public_avif_url: String,
    pub public_jpg_url: String,
    pub hash: String,
    width: u32,
    height: u32,
}

impl Image {
    fn new(remote_url: String) -> Self {
        let hash = hash64(remote_url.as_bytes()).to_string();

        Self {
            remote_url,
            public_avif_url: String::from("flyer/".to_owned() + &hash + ".avif"),
            public_jpg_url: String::from("flyer/".to_owned() + &hash + ".jpg"),
            hash,
            width: 0,
            height: 0,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let width: i32 = 290;
        let height = 290.0 * (self.height as f32) / (self.width as f32);
        let height = height as i32;
        let mut s = serializer.serialize_struct("Image", 4)?;
        s.serialize_field("public_avif_url", &self.public_avif_url)?;
        s.serialize_field("public_jpg_url", &self.public_jpg_url)?;
        s.serialize_field("width", &width)?;
        s.serialize_field("height", &height)?;
        s.end()
    }
}

pub struct BandInfo {
    pub name: String,
    pub genre: Option<String>,
    pub website: Option<String>,
    pub metallum_link: Option<String>,
    pub spirit_link: Option<String>,
}
