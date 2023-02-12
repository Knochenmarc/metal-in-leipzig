use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::tools::date::parse_german_date;
use crate::{Event, Http, Location, Site};

const URL: &str = "https://hellraiser-leipzig.de";

pub struct Hellraiser<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Hellraiser<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "hr",
                name: "Hellraiser Leipzig",
                website: URL,
            },
        }
    }
}

impl Site for Hellraiser<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let reg = Regex::new("(?si)<li class=\"product.*?<a href=\"(?P<url>.*?)\".*?<img.*?src=\"(?P<img>.*?)\".*?<h2.*?>(?P<name>.*?)</h2>.*?<div class=\"date-published\">.*?, (?P<date>.*?)</div>.*?</li>").unwrap();

        for i in 1..10 {
            let html = http
                .get(&(URL.to_string() + "/produkt-kategorie/tickets/page/" + &*i.to_string()))
                .unwrap();

            for capture in reg.captures_iter(html.as_str()) {
                let name = capture.name("name").unwrap().as_str().to_string();
                let name = name.replace("Ticket &#8222;", "");
                let name = name.replace("&#8220;", "");

                let evt = Event::new(
                    decode_html_entities(name.as_str()).to_string(),
                    parse_german_date(capture.name("date").unwrap().as_str())
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
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
