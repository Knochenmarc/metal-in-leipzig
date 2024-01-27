use std::borrow::Borrow;

use regex::Regex;

use crate::site::eventim::Eventim;
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands};
use crate::tools::date::parse_iso_datetime;
use crate::{Event, Http, Location, Site};

const URL: &str = "https://www.taeubchenthal.com/";

pub(crate) struct Taeubchenthal<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Taeubchenthal<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "tt",
                name: "Täubchenthal",
                website: URL,
            },
        }
    }
}

impl Site for Taeubchenthal<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let html = http.get(&(URL.to_string() + "programm")).unwrap();
        let reg = Regex::new("(?is)<div class=\"event event--list.*?<img src=\"(?P<img>.*?)\".*?<h2><a href=\"(?P<url>programm/.*?)\".*?>(?P<name>.*?)</a></h2>.*?<time datetime=\"(?P<date>.*?)\">").unwrap();

        let eventim = Eventim::new("taeubchenthal-leipzig-18055", http);
        let has_metal_band = HasMetalBands {};

        for capture in reg.captures_iter(html.as_str()) {
            let date = capture.name("date").unwrap().as_str();
            if date.len() < 20 {
                continue;
            }
            let name = capture.name("name").unwrap().as_str();

            let mut evt = Event::new(
                name.to_string(),
                parse_iso_datetime(date).unwrap(),
                self.location.borrow(),
                URL.to_owned() + capture.name("url").unwrap().as_str(),
                Some(URL.to_owned() + capture.name("img").unwrap().as_str()),
            );

            if name != "MESH" && name != "Luna" && name != "Schiller" {
                evt.add_band(name.replace(" - Openair", ""));
            }

            for band in evt.bands.iter_mut() {
                spirit_of_metal::find_band(band, http);
                metallum::find_band(band, http);
            }

            if eventim.is_it_metal(evt.borrow()) || has_metal_band.is_it_metal(evt.borrow()) {
                result.push(evt);
            }
        }

        result
    }
}
