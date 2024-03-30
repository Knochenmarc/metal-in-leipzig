use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::{Event, Http, Location, Site};
use crate::site::Filter;
use crate::site::mawi::Mawi;
use crate::tools::date::parse_german_date;

pub(crate) struct Parkbuehne<'l> {
    location: Location<'l, 'l, 'l>,
    insecure_http: Http,
}

impl<'a> Parkbuehne<'_> {
    pub fn new(insecure_http: Http) -> Self {
        Self {
            location: Location {
                slug: "pb",
                name: "Parkbühne im Clara-Zetkin-Park",
                website: "https://www.parkbuehne-leipzig.com",
            },
            insecure_http,
        }
    }
}

impl<'a> Site for Parkbuehne<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let html = http
            .get("https://www.parkbuehne-leipzig.com/wordpress/veranstaltungen/")
            .unwrap();
        let reg = Regex::new("(?is)<article\\s.*?<img\\s.*?src=\"(?P<img>.*?)\".*?<h3\\s.*?<a href=\"(?P<url>.*?)\">(?P<name>.*?)</a></h3>.*?, (?P<date>\\d\\d? [a-z]{2,3} \\d\\d\\d\\d).*?</article>").unwrap();
        let mawi = Mawi::new("Parkbühne Clara-Zetkin-Park", self.insecure_http.borrow());

        let mut result = Vec::new();
        for capture in reg.captures_iter(html.as_str()) {
            let date = parse_german_date(capture.name("date").unwrap().as_str());
            let evt = Event::new(
                decode_html_entities(capture.name("name").unwrap().as_str()).to_string(),
                date.and_hms_opt(0, 0, 0).unwrap(),
                self.location.borrow(),
                capture.name("url").unwrap().as_str().to_string(),
                Some(capture.name("img").unwrap().as_str().to_string()),
            );
            if mawi.is_it_metal(evt.borrow()) {
                result.push(evt);
            }
        }

        result
    }
}
