use std::borrow::Borrow;

use html_escape::decode_html_entities;

use crate::event::{Event, EventStatus, EventType, Location};
use crate::site::{parse_linked_data_events, Site};
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;

pub struct Bandcommunity<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Bandcommunity<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "bc",
                name: "Bandhaus Leipzig",
                website: "https://bandcommunity-leipzig.org/",
            },
        }
    }
}

impl Site for Bandcommunity<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();
        let html = http
            .get("https://bandcommunity-leipzig.org/veranstaltungen/")
            .unwrap();

        for data_event in parse_linked_data_events(html.as_str()) {
            let description = data_event["description"].as_str().unwrap();
            if !description.contains("Rapper") && !description.contains("HipHop") {
                let name = data_event["name"].as_str().unwrap();
                let mut evt = Event::new(
                    decode_html_entities(name).to_string(),
                    parse_iso_datetime(data_event["startDate"].as_str().unwrap()).unwrap(),
                    self.location.borrow(),
                    data_event["url"].as_str().unwrap().to_string(),
                    Some(data_event["image"].as_str().unwrap().to_string()),
                );

                let lower_name = name.to_lowercase();
                evt.evt_type = if lower_name.contains("festival") || lower_name.contains("festevil")
                {
                    EventType::Festival
                } else {
                    EventType::Concert
                };

                evt.end_date =
                    Some(parse_iso_datetime(data_event["endDate"].as_str().unwrap()).unwrap());
                evt.status = EventStatus::from_schema(data_event["eventStatus"].as_str().unwrap());

                result.push(evt);
            }
        }

        result
    }
}