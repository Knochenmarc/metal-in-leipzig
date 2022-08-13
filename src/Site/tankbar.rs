use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::tools::date::parse_short_date;
use crate::{Event, Location, Site, HTTP};

pub(crate) struct Tankbar {
    location: Location,
}

impl Tankbar {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "tb".to_string(),
                name: "TankBar Leipzig".to_string(),
                website: "https://tankbar-leipzig.de/".to_string(),
            },
        }
    }
}

impl Site for Tankbar {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();
        let mut result = Vec::new();

        let html = http.get("https://tankbar-leipzig.de/tankevents/");
        let reg = Regex::new("(?is)<span class=\"elementor-icon-list-text\">(?P<date>\\d\\d\\.\\d\\d\\.)(?P<year>\\d\\d) : (?P<name>.*?)</span>").unwrap();
        for capture in reg.captures_iter(html.as_str()) {
            let name = capture.name("name").unwrap().as_str();
            if name.contains("Metal") || name.contains("Rock") {
                let name = name.replace("<br>", "");
                let date = capture.name("date").unwrap().as_str();
                let year = capture.name("year").unwrap().as_str();

                result.push(Event::new(
                    decode_html_entities(name.trim()).to_string(),
                    parse_short_date(&*(date.to_owned() + "20" + year)).and_hms(0, 0, 0),
                    self.location.borrow(),
                    "https://tankbar-leipzig.de/tankevents/".to_string(),
                    None,
                ));
            }
        }

        result
    }
}
