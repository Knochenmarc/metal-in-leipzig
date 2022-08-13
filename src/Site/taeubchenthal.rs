use std::borrow::Borrow;

use regex::Regex;

use crate::site::eventim::Eventim;
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands};
use crate::tools::date::parse_iso_datetime;
use crate::{Event, Location, Site, HTTP};

const URL: &str = "https://www.taeubchenthal.com/";

pub(crate) struct Taeubchenthal {
    location: Location,
}

impl Taeubchenthal {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "tt".to_string(),
                name: "Täubchenthal".to_string(),
                website: URL.to_string(),
            },
        }
    }
}

impl Site for Taeubchenthal {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();

        let html = http.get(&*(URL.to_string() + "programm"));
        let reg = Regex::new("(?is)<div class=\"event event--list.*?<img src=\"(?P<img>.*?)\".*?<h2><a href=\"(?P<url>programm/.*?)\".*?>(?P<name>.*?)</a></h2>.*?<time datetime=\"(?P<date>.*?)\">").unwrap();

        let eventim = Eventim::new("taeubchenthal-leipzig-18055", http.borrow());
        let has_metal_band = HasMetalBands {};

        for capture in reg.captures_iter(html.as_str()) {
            let date = capture.name("date").unwrap().as_str();
            if date.len() < 20 {
                continue;
            }
            let name = capture.name("name").unwrap().as_str();

            let mut evt = Event::new(
                name.to_string(),
                parse_iso_datetime(date),
                self.location.borrow(),
                URL.to_owned() + &capture.name("url").unwrap().as_str().to_string(),
                Some(URL.to_owned() + &capture.name("img").unwrap().as_str().to_string()),
            );

            if name != "CHECKMATE" {
                evt.add_band(name.replace(" - Openair", ""));
            }

            for band in evt.bands.iter_mut() {
                spirit_of_metal::find_band(band, http.borrow());
                metallum::find_band(band, http.borrow());
            }

            if eventim.is_it_metal(evt.borrow()) || has_metal_band.is_it_metal(evt.borrow()) {
                result.push(evt);
            }
        }

        result
    }
}
