use std::hash::{BuildHasher, Hash, Hasher};

use chrono::{DateTime, FixedOffset};
use twox_hash::RandomXxh3HashBuilder64;

pub struct Event<'l> {
    pub name: String,
    pub date: DateTime<FixedOffset>,
    pub location: &'l Location,
    pub url: String,
    pub image: Image,
}

impl<'a> Event<'a> {
    pub fn new(
        name: String,
        date: DateTime<FixedOffset>,
        location: &'a Location,
        url: String,
        image: String,
    ) -> Self {
        Self {
            name,
            date,
            location,
            url,
            image: Image::new(image),
        }
    }
}

pub struct Location {
    pub slug: String,
    pub name: String,
    pub website: String,
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
        let mut hasher = RandomXxh3HashBuilder64::default().build_hasher();
        remote_url.hash(&mut hasher);
        let hash = hasher.finish().to_string();

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
