use std::borrow::Borrow;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::site::Filter;
use crate::tools::HTTP;
use crate::Event;

pub struct EventimLight {
    collected_events: Vec<Value>,
}

impl EventimLight {
    pub fn new(venue: String, http: &HTTP) -> Self {
        lazy_static! {
            static ref REG: Regex =
                Regex::new("(?i)window.__INITIAL_STATE__=(\\{.*?\\});").unwrap();
        }

        let mut collected_events: Vec<Value> = Vec::new();

        let html = http.get(("https://www.eventim-light.com/de/a/".to_owned() + &venue).as_str());

        let matches = REG.captures(html.borrow()).unwrap();
        let text: &str = matches.get(1).map_or("", |m| m.as_str());
        let json: Value = serde_json::from_str(text).unwrap();
        for (_, event) in json["events"]["cache"].as_object().unwrap() {
            collected_events.push(event.to_owned());
        }

        Self { collected_events }
    }
}

impl Filter for EventimLight {
    fn is_it_metal(&self, evt: &Event) -> bool {
        let date = evt.date.format("%Y-%m-%d").to_string();
        let date = date.as_str();
        for collected_event in self.collected_events.iter() {
            if collected_event["category"].as_str().unwrap() == "Hard & Heavy"
                && collected_event["start"].to_string().starts_with(date)
            {
                return true;
            }
        }

        false
    }
}
