use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_short_date;
use crate::tools::Http;

pub struct ConneIsland<'l, 'h> {
    location: Location<'l, 'l, 'l>,
    insecure_http: &'h Http,
}

impl<'a> ConneIsland<'_, 'a> {
    pub(crate) fn new(insecure_http: &'a Http) -> Self {
        Self {
            location: Location {
                slug: "ci",
                name: "Conne Island",
                website: "https://conne-island.de",
            },
            insecure_http,
        }
    }
}

impl<'a> Site for ConneIsland<'_, 'a> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, _http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let reg =
            Regex::new("(?si)<item>\\s*<title>(?P<title>.*?)</title>.*?<link>(?P<link>.*?)</link>")
                .unwrap();
        let strip_html = Regex::new("(?si)</?.*?>").unwrap();

        let xml = self
            .insecure_http
            .get("https://conne-island.de/rss.php?genre=Metal");
        for item in reg.captures_iter(xml.as_str()) {
            let title = decode_html_entities(item.name("title").unwrap().as_str()).to_string();
            let title = strip_html.replace_all(title.as_str(), "").to_string();
            result.push(Event::new(
                title[12..].to_string(),
                parse_short_date(title[..10].borrow()).and_hms(0, 0, 0),
                self.location.borrow(),
                item.name("link")
                    .unwrap()
                    .as_str()
                    .replace(r"http://", r"https://"),
                Option::None,
            ));
        }

        result
    }
}
