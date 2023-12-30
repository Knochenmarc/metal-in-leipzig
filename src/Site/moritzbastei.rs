use std::borrow::Borrow;
use std::collections::HashMap;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::tools::date::parse_short_date;
use crate::{Event, Location, Site, HTTP};

pub struct Moritzbastei {
    location: Location,
}

impl Moritzbastei {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "mb".to_string(),
                name: "Moritzbastei".to_string(),
                website: "https://www.moritzbastei.de/".to_string(),
            },
        }
    }
}

impl Site for Moritzbastei {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();

        let security_token = {
            let response = http.get("https://www.moritzbastei.de/");
            let security_reg = Regex::new("(?i)\"security\":\"([a-z0-9]+?)\"").unwrap();
            let capture = security_reg.captures(response.as_str()).unwrap();
            capture.get(1).unwrap().as_str().to_string()
        };

        let mut payload = HashMap::new();
        payload.insert("action", "event_ajax_action_callback");
        payload.insert("security", security_token.as_str());
        let json = http.post_json(
            "https://www.moritzbastei.de/wp-admin/admin-ajax.php?offset=0&limit=100",
            payload,
        );

        let reg = Regex::new("(?is)<img.*?src=\"(?P<img>.*?)\".*?(?P<date>\\d\\d\\.\\d\\d\\.\\d\\d\\d\\d).*?<h3.*?<a href=\"(?P<url>.*?)\">(?P<name>.*?)</a>").unwrap();

        for content in json["content"].as_array().unwrap() {
            let html = content.as_str().unwrap();
            if html.contains("#Metal") || html.contains("#Heavy Metal") {
                let captures = reg.captures(html).unwrap();
                let img = captures
                    .name("img")
                    .unwrap()
                    .as_str()
                    .replace("-100x100", "")
                    .to_string();
                result.push(Event::new(
                    decode_html_entities(captures.name("name").unwrap().as_str()).to_string(),
                    parse_short_date(captures.name("date").unwrap().as_str()).and_hms(0, 0, 0),
                    self.location.borrow(),
                    captures.name("url").unwrap().as_str().to_string(),
                    Some(img),
                ));
            }
        }

        result
    }
}
