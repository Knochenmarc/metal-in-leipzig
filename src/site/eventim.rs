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
            static ref REG: Regex = Regex::new(r#"<link rel="next" href="(.*?)""#).unwrap();
        }

        let mut collected_events: Vec<Value> = Vec::new();

        let mut next: String = String::from("/city/leipzig-10/venue/");
        next.push_str(venue);
        next.push_str("/?maincategoryId=1&shownonbookable=true&subcategoryId=2");

        loop {
            let mut url: String = String::from("https://www.eventim.de");
            url.push_str(next.as_str());
            let plain_html = http.get(&url).unwrap();
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

        Self { collected_events }
    }

    pub fn get_raw(self) -> Vec<Value> {
        self.collected_events
    }
}

impl Filter for Eventim {
    fn is_it_metal(&self, evt: &Event) -> bool {
        let event_name = evt.name.to_lowercase();
        for collected_event in &self.collected_events {
            let eventim_name = collected_event["name"].as_str().unwrap().to_lowercase();
            let performer_name = collected_event["performer"]["name"]
                .as_str()
                .unwrap()
                .to_lowercase();

            if eventim_name.contains(&event_name)
                || event_name.contains(&eventim_name)
                || performer_name.contains(&event_name)
                || event_name.contains(&performer_name)
            {
                return true;
            }
            for band in evt.bands.iter() {
                let band_name = band.name.to_lowercase();
                if eventim_name.contains(&band_name)
                    || band_name.contains(&eventim_name)
                    || performer_name.contains(&band_name)
                    || band_name.contains(&performer_name)
                {
                    return true;
                }
            }
        }

        false
    }
}
