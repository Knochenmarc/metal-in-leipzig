use std::borrow::Borrow;

use html_escape::decode_html_entities;

use crate::site::parse_linked_data_events;
use crate::tools::date::parse_iso_datetime;
use crate::{Event, Location, Site, HTTP};

pub(crate) struct Soltmann<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Soltmann<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "sm",
                name: "Soltmann",
                website: "https://www.soltmann-bar.de/",
            },
        }
    }
}

impl Site for Soltmann<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &HTTP) -> Vec<Event> {
        let mut result = Vec::new();

        let html = http.get("https://www.soltmann-bar.de/veranstaltungen/liste/");

        let events = parse_linked_data_events(html.as_str());
        for data_event in events {
            let evt = Event::new(
                decode_html_entities(data_event["name"].as_str().unwrap()).replace("@Soltmann", ""),
                parse_iso_datetime(data_event["startDate"].as_str().unwrap()),
                self.location.borrow(),
                data_event["url"].as_str().unwrap().to_string(),
                Some(data_event["image"].as_str().unwrap().to_string()),
            );

            result.push(evt);
        }

        result
    }
}
