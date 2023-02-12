use std::borrow::Borrow;

use chrono::NaiveDateTime;

use crate::event::EventType;
use crate::{Event, Http, Location, Site};

pub(crate) struct Soltmann<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Soltmann<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "sm",
                name: "Soltmann",
                website: "https://www.soltmann-bar.de/",
            },
        }
    }
}

impl Site for Soltmann<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        // doc: https://www.soltmann-bar.de/wp-json/tribe/events/v1/doc

        let search_tags = ["metal", "rock", "punk", "thrash"];
        let mut collected_tags: Vec<i64> = vec![];

        for search_tag in search_tags {
            let mut tags = http
                .get_json(
                    &("https://www.soltmann-bar.de/wp-json/tribe/events/v1/tags?per_page=50&search="
                        .to_owned() + &search_tag),
                )
                .expect("soltmann: could not fetch tags")["tags"]
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item["id"].as_i64().unwrap())
                .collect::<Vec<i64>>()
                .to_owned();
            collected_tags.append(&mut tags);
        }

        let tags_param: Vec<String> = collected_tags
            .iter()
            .map(|id| "&tags[]=".to_owned() + id.to_string().as_str())
            .collect();
        let events = http
            .get_json(
                &("https://www.soltmann-bar.de/wp-json/tribe/events/v1/events?per_page=100"
                    .to_owned()
                    + tags_param.join("").as_str()),
            )
            .expect("soltmann: could not fetch events");

        for event in events["events"].as_array().unwrap() {
            let mut evt = Event::new(
                event["title"].as_str().unwrap().to_string(),
                NaiveDateTime::parse_from_str(
                    event["start_date"].as_str().unwrap(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                self.location.borrow(),
                event["url"].as_str().unwrap().to_string(),
                Some(event["image"]["url"].as_str().unwrap().to_string()),
            );

            evt.end_date = Some(
                NaiveDateTime::parse_from_str(
                    event["end_date"].as_str().unwrap(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
            );

            evt.evt_type = match event["categories"][0]["slug"].as_str() {
                Some("konzert") => EventType::Concert,
                Some("party") => EventType::Party,
                Some("stream") => EventType::Online,
                Some("festival") => EventType::Festival,
                _ => EventType::Unknown,
            };

            result.push(evt);
        }

        result
    }
}
