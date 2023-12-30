use std::borrow::Borrow;

use chrono::DateTime;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::{Filter, Site};
use crate::tools::HTTP;

pub struct Anker {
    location: Location,
}

impl Anker {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "ank".to_string(),
                name: "der ANKER".to_string(),
                website: "https://anker-leipzig.de".to_string(),
            },
        }
    }
}

impl Site for Anker {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();

        let eventim = Eventim::new("der-anker-leipzig-7330".to_string(), http.borrow());

        let api = http.get_json(
            "https://anker-leipzig.de/wp-json/wp/v2/event_listing?per_page=100"
                .to_string()
                .as_str(),
        );

        for item in api.as_array().unwrap() {
            let image_url = match item["_links"]["wp:featuredmedia"][0]["href"].as_str() {
                None => None,
                Some(url) => Some(http.get_json(url)["source_url"].to_string()),
            };

            let evt = Event::new(
                item["title"]["rendered"].to_string(), //TODO: decode html
                DateTime::parse_from_rfc3339(
                    (item["date"].as_str().unwrap().to_owned() + "Z").as_str(),
                )
                .unwrap(),
                self.location.borrow(),
                item["link"].to_string(),
                image_url,
            );

            if eventim.is_it_metal(evt.borrow()) {
                result.push(evt);
            }
        }

        return result;
    }
}
