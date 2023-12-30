use chrono::NaiveDateTime;
use twox_hash::xxh3::hash64;

pub struct Event<'l> {
    pub name: String,
    pub date: NaiveDateTime,
    pub location: &'l Location,
    pub url: String,
    pub image: Option<Image>,
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

        Self {
            name,
            date,
            location,
            url,
            image,
        }
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

pub struct Image {
    pub remote_url: String,
    pub public_avif_url: String,
    pub public_jpg_url: String,
    pub hash: String,
    width: u32,
    height: u32,
    ratio: f32,
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
            ratio: 0.0,
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.ratio = (width / height) as f32;
    }
}
