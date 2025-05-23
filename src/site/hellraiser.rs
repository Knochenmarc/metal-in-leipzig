use chrono::{Datelike, NaiveDateTime, Timelike};
use html_escape::decode_html_entities;
use regex::Regex;
use std::borrow::Borrow;

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

        let reg = Regex::new("(?si)<li class=\"product.*?<a href=\"(?P<url>.*?)\".*?<img.*?data-src=\"(?P<img>.*?)\".*?<h2.*?>(?P<name>.*?)</h2>.*?<div class=\"date-published\">.*?, (?P<date>.*?)</div>(\\s*<div class=\"various-date\">.*?, (?P<variousfrom>.*?) bis .*?, (?P<variousto>.*?)</div>)?.*?</li>").unwrap();

        for i in 1..10 {
            let html = http
                .get(&(URL.to_string() + "/produkt-kategorie/tickets/page/" + &*i.to_string()))
                .unwrap();

            for capture in reg.captures_iter(html.as_str()) {
                let name = capture.name("name").unwrap().as_str().to_string();
                let name = name.replace("Ticket &#8222;", "");
                let name = name.replace("&#8220;", "");

                let mut end_date: Option<NaiveDateTime> = None;
                let start_date = if capture.name("variousfrom").is_some()
                    && capture.name("variousto").is_some()
                {
                    end_date = parse_german_date(capture.name("variousto").unwrap().as_str())
                        .with_hour(23)
                        .unwrap()
                        .with_minute(59);
                    parse_german_date(capture.name("variousfrom").unwrap().as_str())
                        .with_year(end_date.unwrap().year())
                        .unwrap()
                } else {
                    parse_german_date(capture.name("date").unwrap().as_str())
                };

                let mut evt = Event::new(
                    decode_html_entities(name.as_str()).to_string(),
                    start_date,
                    self.location.borrow(),
                    capture.name("url").unwrap().as_str().to_string(),
                    Some(capture.name("img").unwrap().as_str().to_string()),
                );
                evt.end_date = end_date;

                result.push(evt);
            }

            if !html.contains("class=\"next page-numbers\"") {
                break;
            }
        }

        result
    }
}
