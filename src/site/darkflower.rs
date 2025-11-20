use crate::event::{Event, Location};
use crate::site::Site;
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;
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
        let mut flyer = item.get(&("flyer_hochkant_flloor_".to_owned() + floor));
        if flyer.is_none() {
            flyer = item.get(&("flyer_hochkant_floor_".to_owned() + floor));
        }
        let image = flyer
            .unwrap()
            .as_object()
            .unwrap()
            .get("guid")
            .unwrap()
            .as_str()
            .unwrap();

        let name = item
            .get(&("veranstaltungsname_floor_".to_owned() + floor))
            .unwrap()
            .as_str()
            .unwrap();

        let date = item
            .get("datum_der_veranstaltung")
            .unwrap()
            .as_str()
            .unwrap();
        let time = item.get("einlass").unwrap().as_str().unwrap();
        let link = item.get("link").unwrap().as_str().unwrap();

        Event::new(
            name.to_string(),
            parse_iso_datetime(format!("{date}T{time}").as_str()).unwrap(),
            self.get_location(),
            link.to_string(),
            Some(image.to_string()),
        )
    }
}

impl Site for Darkflower<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let response =
            http.get_json("https://www.darkflower.de/wp-json/wp/v2/veranstaltung?per_page=50");

        let mut added_ids: Vec<u64> = Vec::new();

        for item in response.unwrap().as_array().unwrap() {
            let item = item.as_object().unwrap();

            let id = item.get("id").unwrap().as_u64().unwrap();
            if added_ids.contains(&id) {
                continue;
            }

            let name1 = item
                .get("veranstaltungsname_floor_1")
                .unwrap()
                .as_str()
                .unwrap();
            if !name1.is_empty()
                && (self.is_metal(item.get("veranstaltungsname_floor_1"))
                    || self.is_metal(item.get("beschreibung_floor_1"))
                    || self.is_metal(item.get("musikrichtung_auf_floor_1")))
            {
                result.push(self.build_event(item, "1"));
                added_ids.push(id);
            }

            let name2 = item
                .get("veranstaltungsname_floor_2")
                .unwrap()
                .as_str()
                .unwrap();
            if !name2.is_empty()
                && name1 != name2
                && (self.is_metal(item.get("veranstaltungsname_floor_2"))
                    || self.is_metal(item.get("beschreibung_floor_2"))
                    || self.is_metal(item.get("musikrichtung_auf_floor_2")))
            {
                result.push(self.build_event(item, "2"));
                added_ids.push(id);
            }
        }

        result
    }
}
