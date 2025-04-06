use crate::event::{Event, Location};
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::date::parse_iso_date;
use crate::tools::Http;
use html_escape::decode_html_entities;
use regex::Regex;
use serde_json::Value;
use std::borrow::Borrow;

pub(crate) struct TVClub<'l> {
    location: Location<'l, 'l, 'l>,
}

impl TVClub<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "tv",
                name: "TV-Club Leipzig",
                website: "https://www.tv-club-leipzig.de/",
            },
        }
    }
}

impl Site for TVClub<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let html = http.get("https://www.tv-club-leipzig.de/events/").unwrap();

        let json_reg = Regex::new(r"(?i)\[\{&quot;.*;&quot;}]").unwrap();
        let json_encoded = json_reg.captures(&html).unwrap().get(0).unwrap().as_str();
        let json = decode_html_entities(json_encoded).to_string();
        let json: Value = serde_json::from_str(json.as_str()).unwrap();

        let title_reg = Regex::new(r"(?i) am \d\d\.\d\d\.\d\d\d\d").unwrap();
        let today = chrono::Utc::now().date_naive();

        let mut result: Vec<Event> = vec![];
        let has_metal_band = HasMetalBands {};

        for v in json.as_array().unwrap().iter() {
            let mut title = v.get("title").unwrap().as_str().unwrap().to_string();
            let lowered_title = title.to_lowercase();
            if lowered_title.contains("party")
                || lowered_title.contains("semester")
                || lowered_title.contains("fasching")
                || title.contains("Stand-Up Comedy")
            {
                continue;
            }
            title = title.replace(" Live ", " ");
            let title = title_reg.replace_all(&title, "").to_string();

            let mut date = v.get("dateGMT").unwrap().as_str().unwrap().to_string();
            if date.starts_with("2024-")
                && v.get("modifiedDate")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
                    .starts_with("2025-")
            {
                date = date.replace("2024-", "2025-");
            }
            let date = date.get(0..10).unwrap().to_string();
            let date = parse_iso_date(date.as_str());
            if today.gt(date.date().borrow()) {
                continue;
            }

            let img = v
                .get("thumbnail")
                .unwrap()
                .as_object()
                .unwrap()
                .get("url")
                .unwrap();
            let img = (img.as_str()).map(|str| str.to_string());

            let excerpt = v.get("excerpt").unwrap().as_str().unwrap().to_string();
            let lowered_excerpt = excerpt.to_lowercase();

            let mut event = Event::new(
                title.clone(),
                date,
                self.location.borrow(),
                v.get("link").unwrap().as_str().unwrap().to_string(),
                img,
            );

            if lowered_excerpt.contains("metal") || lowered_excerpt.contains("rock") {
                result.push(event);
            } else {
                event.add_band(title);
                for band in event.bands.iter_mut() {
                    spirit_of_metal::find_band(band, http);
                    metallum::find_band(band, http);
                }
                if has_metal_band.is_it_metal(event.borrow()) {
                    result.push(event);
                }
            }
        }

        result
    }
}
