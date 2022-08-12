use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::tools::date::parse_german_date;
use crate::{Event, Location, Site, HTTP};

const URL: &str = "https://hellraiser-leipzig.de";

pub struct Hellraiser {
    location: Location,
}

impl Hellraiser {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "hr".to_string(),
                name: "Hellraiser Leipzig".to_string(),
                website: URL.to_string(),
            },
        }
    }
}

impl Site for Hellraiser {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();

        let reg = Regex::new("(?si)<li class=\"product.*?<a href=\"(?P<url>.*?)\".*?<img.*?src=\"(?P<img>.*?)\".*?<h2.*?>(?P<name>.*?)</h2>.*?<div class=\"date-published\">.*?, (?P<date>.*?)</div>.*?</li>").unwrap();

        for i in 1..10 {
            let html =
                http.get(&(URL.to_string() + "/produkt-kategorie/tickets/page/" + &*i.to_string()));

            for capture in reg.captures_iter(html.as_str()) {
                let name = capture.name("name").unwrap().as_str().to_string();
                let name = name.replace("Ticket &#8222;", "");
                let name = name.replace("&#8220;", "");

                let evt = Event::new(
                    decode_html_entities(name.as_str()).to_string(),
                    parse_german_date(capture.name("date").unwrap().as_str()).and_hms(0, 0, 0),
                    self.location.borrow(),
                    capture.name("url").unwrap().as_str().to_string(),
                    Some(capture.name("img").unwrap().as_str().to_string()),
                );

                result.push(evt);
            }

            if !html.contains("class=\"next page-numbers\"") {
                break;
            }
        }

        result
    }
}
