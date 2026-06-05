use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;
use regex::Regex;
use serde_json::{Map, Value};
use std::borrow::Borrow;

pub struct Darkflower<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Darkflower<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "df",
                name: "Darkflower",
                website: "https://darkflower.club/",
            },
        }
    }

    fn is_metal(&self, text: Option<&Value>) -> bool {
        text.unwrap()
            .as_str()
            .unwrap_or("")
            .to_lowercase()
            .contains("metal")
    }

    fn build_event(&self, item: &Map<String, Value>, floor: &str) -> Event {
        // jep, da gibts nen Typo in der api >.<
        let flyer = item.get(&("flyerFloor".to_owned() + floor));
        let image = flyer.unwrap().as_object().map(|flyer| {
            flyer
                .get("sourceUrl")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string()
        });

        let name = item
            .get(&("veranstaltungsnameFloor".to_owned() + floor))
            .unwrap()
            .as_str()
            .unwrap();

        let date = item.get("datum").unwrap().as_str().unwrap();
        let time = item.get("einlass").unwrap().as_str().unwrap();
        let id = item.get("databaseId").unwrap().as_u64().unwrap();
        let floor_name = match floor {
            "1" => "main",
            "2" => "second",
            _ => "",
        };

        let link = format!(
            "https://darkflower.de/veranstaltung/{}?floor={}",
            id, floor_name
        );

        Event::new(
            name.to_string(),
            parse_iso_datetime(format!("{date}T{time}").as_str()).unwrap(),
            self.get_location(),
            link.to_string(),
            image,
        )
    }
}

impl Site for Darkflower<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let html = http.get("https://darkflower.de/terminkalender").unwrap();

        let reg: Regex =
            Regex::new("(?si)data: \\[null,\\{type:\"data\",data:(\\{.*?}),uses:").unwrap();
        let json = reg
            .captures(html.as_str())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let json: Value = serde_json5::from_str(json).unwrap();

        let mut added_ids: Vec<u64> = Vec::new();

        for item in json.get("events").unwrap().as_array().unwrap() {
            let item = item.as_object().unwrap();

            let id = item.get("databaseId").unwrap().as_u64().unwrap();
            if added_ids.contains(&id) {
                continue;
            }

            let name1 = item
                .get("veranstaltungsnameFloor1")
                .unwrap()
                .as_str()
                .unwrap_or("");
            if !name1.is_empty()
                && (self.is_metal(item.get("veranstaltungsnameFloor1"))
                    || self.is_metal(item.get("beschreibungFloor1"))
                    || self.is_metal(item.get("musikrichtungFloor1")))
            {
                result.push(self.build_event(item, "1"));
                added_ids.push(id);
            }

            let name2 = item
                .get("veranstaltungsnameFloor2")
                .unwrap()
                .as_str()
                .unwrap_or("");
            if !name2.is_empty()
                && name1 != name2
                && (self.is_metal(item.get("veranstaltungsnameFloor2"))
                    || self.is_metal(item.get("beschreibungFloor2"))
                    || self.is_metal(item.get("musikrichtungFloor2")))
            {
                result.push(self.build_event(item, "2"));
                added_ids.push(id);
            }
        }

        result
    }
}
