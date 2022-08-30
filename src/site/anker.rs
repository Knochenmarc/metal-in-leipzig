use std::borrow::Borrow;

use chrono::NaiveDateTime;
use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, EventStatus, EventType, Location};
use crate::site::eventim::Eventim;
use crate::site::{parse_linked_data_events, Filter, Site};
use crate::tools::date::parse_german_date;
use crate::tools::Http;

pub struct Anker<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Anker<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "ank",
                name: "der ANKER",
                website: "https://anker-leipzig.de",
            },
        }
    }
}

impl Site for Anker<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let eventim = Eventim::new("der-anker-leipzig-7330", http.borrow());

        let html = http.get("https://anker-leipzig.de/va/veranstaltungen/");
        let reg: Regex = Regex::new(
            "(?si)wpem-single-event-widget.*?<a href=\"(?P<url>.*?)\".*?<h3 class=\"wpem-heading-text\" title=\"(?P<title>.*?)\">.*?wpem-event-date-time-text\">.*?,\\s(?P<date>.*?)<",
        )
        .unwrap();

        for captures in reg.captures_iter(html.as_str()) {
            let name = captures.name("title").unwrap().as_str();
            let date = captures.name("date").unwrap().as_str();
            let url = captures.name("url").unwrap().as_str();

            let mut evt = Event::new(
                decode_html_entities(name).to_string(),
                parse_german_date(date).and_hms(0, 0, 0),
                self.location.borrow(),
                url.to_string(),
                None,
            );

            if eventim.is_it_metal(evt.borrow()) {
                let sub_html = http.get(url);
                let data_events = parse_linked_data_events(sub_html.as_str());
                if !data_events.is_empty() {
                    let data_event = data_events.first().unwrap();

                    data_event["image"]
                        .as_array()
                        .unwrap()
                        .first()
                        .map(|i| i.as_str().unwrap().to_string())
                        .map(|i| evt.set_image(i));

                    evt.status =
                        EventStatus::from_schema(data_event["eventStatus"].as_str().unwrap());
                    evt.start_date = NaiveDateTime::parse_from_str(
                        data_event["startDate"].as_str().unwrap(),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap()
                }
                if sub_html.contains("event-category konzert") {
                    evt.evt_type = EventType::Concert;
                }

                if name.to_lowercase().starts_with("abgesagt!") {
                    evt.status = EventStatus::Cancelled;
                } else if name.to_lowercase().starts_with("ausverkauft") {
                    evt.status = EventStatus::SoldOut;
                }

                result.push(evt);
            }
        }

        result
    }
}
