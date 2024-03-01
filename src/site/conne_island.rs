use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::tixforgigs::fetch_tixforgigs_event;
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

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let tix_reg = Regex::new("https://www.tixforgigs.com/de-de/Event/(\\d+)").unwrap();

        let xml_reg =
            Regex::new("(?si)<item>\\s*<title>(?P<title>.*?)</title>.*?<description>(?P<description>.*?)</description>.*?<link>(?P<link>.*?)</link>")
                .unwrap();
        let strip_html = Regex::new("(?si)</?.*?>").unwrap();

        let xml = self
            .insecure_http
            .get("https://conne-island.de/rss.php?genre=Metal")
            .unwrap();
        for item in xml_reg.captures_iter(xml.as_str()) {
            let title = decode_html_entities(item.name("title").unwrap().as_str()).to_string();
            let title = strip_html.replace_all(title.as_str(), "").to_string();

            let mut event = Event::new(
                title[12..].to_string(),
                parse_short_date(title[..10].borrow()),
                self.location.borrow(),
                item.name("link")
                    .unwrap()
                    .as_str()
                    .replace(r"http://", r"https://"),
                None,
            );
            event.description =
                Some(decode_html_entities(item.name("description").unwrap().as_str()).to_string());

            let id = event
                .url
                .replace("https://www.conne-island.de/termin/nr", "")
                .replace(".html", "");
            let ticket_info = self
                .insecure_http
                .get(format!("https://conne-island.de/ticket_info.php?nr={}", id).as_str())
                .unwrap();

            if let Some(caps) = tix_reg.captures(ticket_info.as_str()) {
                // let caps = tix_reg.captures(ticket_info.as_str()).unwrap();
                let tixforgigs_event = caps.get(1).unwrap().as_str();
                let tixforgigs_data = fetch_tixforgigs_event(http, tixforgigs_event);

                event.set_image(tixforgigs_data["image"].as_str().unwrap().to_string());
                for performer in tixforgigs_data["performer"].as_array().unwrap() {
                    event.add_band(performer["name"].as_str().unwrap().to_string());
                }
            }

            result.push(event);
        }

        result
    }
}
