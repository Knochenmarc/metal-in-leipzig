use std::borrow::Borrow;

use chrono::Datelike;
use regex::Regex;

use crate::tools::date::parse_german_date;
use crate::{Event, Location, Site, HTTP};

const URL: &str = "https://www.werk-2.de";

pub(crate) struct Werk2 {
    location: Location,
}

impl Werk2 {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "w2".to_string(),
                name: "WERK 2".to_string(),
                website: URL.to_string(),
            },
        }
    }
}

impl Site for Werk2 {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();
        let mut result = Vec::new();

        let reg = Regex::new("(?is)<div class='monat'>(?P<month>.*?)</div>.*?<div class='tag'>(?P<day>\\d\\d)</div>.*?<p class='typen'>.*?Metal.*?</p>.*?<h2><a href='(?P<url>.*?)'>(?P<name>.*?)</a>.*?<img .*?src='(?P<img>.*?)'").unwrap();

        let mut has_december = false;
        let this_year = chrono::Utc::now().year();
        let next_year = this_year + 1;

        let urls = [URL, "https://www.werk-2.de/programm/vorschau"];
        for url in urls {
            let html = http.get(url);

            for captures in reg.captures_iter(html.as_str()) {
                let month = captures.name("month").unwrap().as_str();
                let day = captures.name("day").unwrap().as_str();
                let url = captures.name("url").unwrap().as_str();
                let name = captures.name("name").unwrap().as_str();
                let img = captures.name("img").unwrap().as_str();

                if has_december == false && month == "Dezember" {
                    has_december = true;
                }
                let year = if has_december && month != "Dezember" {
                    next_year
                } else {
                    this_year
                };

                let evt = Event::new(
                    name.to_string(),
                    parse_german_date(format!("{}. {} {}", day, month, year).as_str())
                        .and_hms(0, 0, 0),
                    self.location.borrow(),
                    format!("{}{}", URL, url),
                    Some(format!("{}{}", URL, img.replace("_liste", "_detail"))),
                );
                result.push(evt);
            }
        }

        result
    }
}
