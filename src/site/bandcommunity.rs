use std::borrow::Borrow;

use crate::event::{Event, EventStatus, EventType, Location};
use crate::site::{parse_linked_data_events, Site};
use crate::tools::date::{parse_iso_datetime, parse_short_date};
use crate::tools::Http;
use html_escape::decode_html_entities;
use regex::Regex;

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
        let response = http.get("https://bandcommunity-leipzig.org/veranstaltungen/");

        if response.as_ref().is_err() && response.as_ref().unwrap_err().is_connect() {
            // Maybe something wrong with TLS?
            return result;
        }

        let subdates_reg: Regex = Regex::new("(?si)📆 (\\d+.\\d+.) (.*?)<").unwrap();

        let html = response.unwrap();
        let mut last_name = String::new();

        for data_event in parse_linked_data_events(html.as_str()) {
            let description = data_event["description"].as_str().unwrap();
            let name = data_event["name"].as_str().unwrap();
            if name != "Open Jam Session"
                && !name.contains("Hip Hop")
                && !description.contains("Rapper")
                && !description.contains("HipHop")
                && !last_name.eq_ignore_ascii_case(name)
            {
                last_name = name.to_string();
                if description.contains("Termine:") && description.contains("📆 ") {
                    for captures in subdates_reg.captures_iter(description) {
                        let name = captures.get(2).unwrap().as_str();
                        let mut evt = Event::new(
                            decode_html_entities(name).to_string(),
                            parse_short_date(captures.get(1).unwrap().as_str()),
                            self.location.borrow(),
                            data_event["url"].as_str().unwrap().to_string(),
                            Some(data_event["image"].as_str().unwrap().to_string()),
                        );
                        evt.evt_type = EventType::Concert;
                        result.push(evt);
                    }
                } else {
                    let start_date =
                        parse_iso_datetime(data_event["startDate"].as_str().unwrap()).unwrap();
                    let end_date =
                        Some(parse_iso_datetime(data_event["endDate"].as_str().unwrap()).unwrap());

                    let mut evt = Event::new(
                        decode_html_entities(name).to_string(),
                        start_date,
                        self.location.borrow(),
                        data_event["url"].as_str().unwrap().to_string(),
                        Some(data_event["image"].as_str().unwrap().to_string()),
                    );

                    let lower_name = name.to_lowercase();
                    evt.evt_type =
                        if lower_name.contains("festival") || lower_name.contains("festevil") {
                            EventType::Festival
                        } else {
                            EventType::Concert
                        };

                    evt.end_date = end_date;
                    evt.status =
                        EventStatus::from_schema(data_event["eventStatus"].as_str().unwrap());

                    result.push(evt);
                }
            }
        }

        result
    }
}
