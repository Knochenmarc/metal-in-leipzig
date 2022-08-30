use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::tools::date::parse_short_date;
use crate::{Event, Location, Site, HTTP};

pub(crate) struct Tankbar<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Tankbar<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "tb",
                name: "TankBar Leipzig",
                website: "https://tankbar-leipzig.de/",
            },
        }
    }
}

impl Site for Tankbar<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &HTTP) -> Vec<Event> {
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
