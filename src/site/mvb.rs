use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_short_date;
use crate::tools::Http;
use serde_json::{Map, Value};
use std::borrow::Borrow;

pub struct MVB<'l> {
    location: Location<'l, 'l, 'l>,
}

impl MVB<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "mvb",
                name: "M.V.B.",
                website: "https://mvb-leipzig.de/",
            },
        }
    }

    fn get_value(&self, fields: &Map<String, Value>, key: &str) -> String {
        fields
            .get(key)
            .unwrap()
            .as_object()
            .unwrap()
            .get("stringValue")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}
impl Site for MVB<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let url = "https://firestore.googleapis.com/v1/projects/mvb-leipzig-303a7/databases/(default)/documents/events";
        let data = http.get_json(url).unwrap();
        data.as_object()
            .unwrap()
            .get("documents")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .for_each(|e| {
                let fields = e
                    .as_object()
                    .unwrap()
                    .get("fields")
                    .unwrap()
                    .as_object()
                    .unwrap();
                let info = self.get_value(fields, "info");
                if info.to_lowercase().contains("metal") {
                    result.push(Event::new(
                        self.get_value(fields, "title"),
                        parse_short_date(self.get_value(fields, "date").as_str()),
                        self.location.borrow(),
                        "https://mvb-leipzig.de/".to_string(),
                        Some(self.get_value(fields, "img")),
                    ));
                }
            });

        result
    }
}
