use std::borrow::Borrow;

use chrono::NaiveTime;
use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_short_date;
use crate::tools::HTTP;

pub struct Bandcommunity {
    location: Location,
}

impl Bandcommunity {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "bc".to_string(),
                name: "Bandcommunity Leipzig".to_string(),
                website: "https://bandcommunity-leipzig.org/".to_string(),
            },
        }
    }
}

impl Site for Bandcommunity {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self, http: &HTTP) -> Vec<Event> {
        let mut result = Vec::new();
        let html = http.get("https://bandcommunity-leipzig.org/blog.html");
        let reg: Regex = Regex::new("(?si)<div class=\"event layout_upcoming upcoming.*?<a\\s+href=\"(.*?)\"\\s+title=\"(.*?) [(].*?(\\d\\d\\.\\d\\d\\.\\d\\d\\d\\d)[, ]+(\\d\\d:\\d\\d)?.*?[)].*?\">") .unwrap();
        let img_reg: Regex = Regex::new("(?si)<div class=\"image\"><img src=\"(.*?)\"").unwrap();

        for captures in reg.captures_iter(html.as_str()) {
            let url = "https://bandcommunity-leipzig.org/".to_owned() + &captures[1].to_string();
            let event_page = http.get(&url);

            let mut image_url = Option::None;
            let image_cap = img_reg.captures(event_page.as_str());
            if image_cap.is_some() {
                image_url = Option::Some(
                    "https://bandcommunity-leipzig.org/".to_owned()
                        + &image_cap.unwrap()[1].to_string(),
                );
            }

            let mut time = NaiveTime::from_hms(0, 0, 0);
            if captures.get(4).is_some() {
                time = NaiveTime::parse_from_str(&captures[4], "%H:%M").unwrap();
            }

            let evt = Event::new(
                decode_html_entities(&captures[2]).to_string(),
                parse_short_date(&captures[3]).and_time(time),
                self.location.borrow(),
                url,
                image_url,
            );
            result.push(evt);
        }

        result
    }
}
