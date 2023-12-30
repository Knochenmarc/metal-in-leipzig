use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::site::mawi::Mawi;
use crate::site::Filter;
use crate::tools::date::parse_german_date;
use crate::{Event, Location, Site, HTTP};

pub(crate) struct Parkbuehne<'h> {
    location: Location,
    insecure_http: &'h HTTP,
}

impl<'a> Parkbuehne<'a> {
    pub fn new(insecure_http: &'a HTTP) -> Self {
        Self {
            location: Location {
                slug: "pb".to_string(),
                name: "Parkbühne im Clara-Zetkin-Park".to_string(),
                website: "https://www.parkbuehne-leipzig.com".to_string(),
            },
            insecure_http,
        }
    }
}

impl<'a> Site for Parkbuehne<'a> {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self, http: &HTTP) -> Vec<Event> {
        let html = http.get("https://www.parkbuehne-leipzig.com/wordpress/veranstaltungen/");
        let reg = Regex::new("(?is)<article\\s.*?<img\\s.*?src=\"(?P<img>.*?)\".*?<h3\\s.*?<a href=\"(?P<url>.*?)\">(?P<name>.*?)</a></h3>.*?, (?P<date>\\d\\d? [a-z]{2,3} \\d\\d\\d\\d).*?</article>").unwrap();
        let mawi = Mawi::new("Parkbühne Clara-Zetkin-Park", self.insecure_http.borrow());

        let mut result = Vec::new();
        for capture in reg.captures_iter(html.as_str()) {
            let date = parse_german_date(capture.name("date").unwrap().as_str());
            let evt = Event::new(
                decode_html_entities(capture.name("name").unwrap().as_str()).to_string(),
                date.and_hms(0, 0, 0),
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
