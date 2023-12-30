use std::borrow::Borrow;

use html_escape::decode_html_entities;

use crate::event::{Event, Location};
use crate::site::{parse_linked_data_events, Site};
use crate::tools::date::parse_iso_datetime;
use crate::tools::HTTP;

pub struct Darkflower {
    location: Location,
}

impl Darkflower {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "df".to_string(),
                name: "Darkflower".to_string(),
                website: "https://darkflower.club/".to_string(),
            },
        }
    }
}

impl Site for Darkflower {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();
        let json = http.get_json("https://darkflower.club/wp-json/wp/v2/pages/932");
        let html = json["content"]["rendered"].as_str().unwrap();
        for data_event in parse_linked_data_events(html) {
            let name = data_event["name"].as_str().unwrap();
            result.push(Event::new(
                decode_html_entities(name).to_string(),
                parse_iso_datetime(data_event["startDate"].as_str().unwrap()),
                self.location.borrow(),
                data_event["url"].as_str().unwrap().to_string(),
                Option::Some(data_event["image"].as_str().unwrap().to_string()),
            ));
        }

        result
    }
}
