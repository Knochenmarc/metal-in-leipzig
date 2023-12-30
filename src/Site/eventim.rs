use std::borrow::Borrow;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::event::Event;
use crate::site::{Filter, parse_linked_data_events};
use crate::tools::HTTP;

pub struct Eventim {
    collected_events: Vec<Value>,
}

impl Eventim {
    pub fn new(venue: String, http: &HTTP) -> Self {
        lazy_static! {
            static ref REG: Regex = Regex::new(r#"<link rel="next" href="(.*?s)""#).unwrap();
        }

        let mut collected_events: Vec<Value> = Vec::new();

        let mut next: String = String::from("/city/leipzig-10/venue/");
        next.push_str(venue.as_str());
        next.push_str("/?maincategoryId=1&shownonbookable=true&subcategoryId=2");

        'crawler: while {
            let mut url: String = String::from("https://www.eventim.de");
            url.push_str(next.as_str());
            let plain_html = http.get(&url);
            let events = parse_linked_data_events(plain_html.borrow());
            collected_events.extend(events);
            if false == REG.is_match(plain_html.borrow()) {
                break 'crawler;
            }

            let matches = REG.captures(plain_html.borrow()).unwrap();
            let text1 = matches.get(1).map_or("", |m| m.as_str());
            next = String::from(text1);
            next.push_str("&shownonbookable=true");
        } {}

        Self { collected_events }
    }
}

impl Filter for Eventim {
    fn is_it_metal(&self, evt: &Event) -> bool {
        let date = evt.date.format("%Y-%m-%d").to_string();
        for collected_event in &self.collected_events {
            if collected_event["startDate"]
                .as_str()
                .unwrap()
                .starts_with(date.as_str())
            {
                return true;
            }
        }

        return false;
    }
}
