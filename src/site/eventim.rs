use std::borrow::Borrow;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::event::Event;
use crate::site::{parse_linked_data_events, Filter};
use crate::tools::Http;

pub struct Eventim {
    collected_events: Vec<Value>,
}

impl Eventim {
    pub fn new(venue: &str, http: &Http) -> Self {
        lazy_static! {
            static ref REG: Regex = Regex::new(r#"<link rel="next" href="(.*?s)""#).unwrap();
        }

        let mut collected_events: Vec<Value> = Vec::new();

        let mut next: String = String::from("/city/leipzig-10/venue/");
        next.push_str(venue);
        next.push_str("/?maincategoryId=1&shownonbookable=true&subcategoryId=2");

        loop {
            let mut url: String = String::from("https://www.eventim.de");
            url.push_str(next.as_str());
            let plain_html = http.get(&url);
            let mut events = parse_linked_data_events(plain_html.borrow());
            collected_events.append(&mut events);

            if !REG.is_match(plain_html.borrow()) {
                break;
            }

            let matches = REG.captures(plain_html.borrow()).unwrap();
            let text1 = matches.get(1).map_or("", |m| m.as_str());
            next = String::from(text1);
            next.push_str("&shownonbookable=true");
        }

        // TODO: gets each event twice as jsonld

        Self { collected_events }
    }
}

impl Filter for Eventim {
    fn is_it_metal(&self, evt: &Event) -> bool {
        let date = evt.start_date.format("%Y-%m-%d").to_string();
        let date = date.as_str();
        for collected_event in &self.collected_events {
            if collected_event["startDate"]
                .as_str()
                .unwrap()
                .starts_with(date)
            {
                return true;
            }
        }

        false
    }
}
