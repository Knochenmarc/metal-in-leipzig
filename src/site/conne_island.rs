use std::borrow::Borrow;

use regex::Regex;

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_short_date;
use crate::tools::HTTP;

pub struct ConneIsland<'h> {
    location: Location,
    insecure_http: &'h HTTP,
}

impl<'a> ConneIsland<'a> {
    pub(crate) fn new(insecure_http: &'a HTTP) -> Self {
        Self {
            location: Location {
                slug: "ci".to_string(),
                name: "Conne Island".to_string(),
                website: "https://conne-island.de".to_string(),
            },
            insecure_http,
        }
    }
}

impl<'a> Site for ConneIsland<'a> {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self, _http: &HTTP) -> Vec<Event> {
        let mut result = Vec::new();

        let reg =
            Regex::new("(?si)<item>\\s*<title>(?P<title>.*?)</title>.*?<link>(?P<link>.*?)</link>")
                .unwrap();

        let xml = self
            .insecure_http
            .get("https://conne-island.de/rss.php?genre=Metal");
        for item in reg.captures_iter(xml.as_str()) {
            let title = item.name("title").unwrap().as_str();
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
