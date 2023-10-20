use std::borrow::Borrow;
use std::collections::HashMap;

use html_escape::decode_html_entities;

use crate::event::{Event, Location};
use crate::site::{parse_linked_data_events, Site};
use crate::tools::date::parse_iso_datetime;
use crate::tools::Http;

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
}

impl Site for Darkflower<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();
        let mut payload = HashMap::new();
        payload.insert("cals[evcal_calendar_707][sc][cal_id]", "");
        let json = http.post_json(
            "https://darkflower.club/?evo-ajax=eventon_init_load",
            payload,
        );
        let html = json["cals"]["evcal_calendar_707"]["html"].as_str().unwrap();
        for data_event in parse_linked_data_events(html) {
            let name = data_event["name"].as_str().unwrap();
            if !name.contains("Darkflower Electro Party") && !name.contains("Synth-Electro Party") {
                let date = parse_iso_datetime(data_event["startDate"].as_str().unwrap());
                match date {
                    Ok(start_date) => {
                        result.push(Event::new(
                            decode_html_entities(decode_html_entities(name).to_mut()).to_string(),
                            start_date,
                            self.location.borrow(),
                            "https://darkflower.club/veranstaltungen/".to_string(), //data_event["url"].as_str().unwrap().to_string(),
                            Option::Some(data_event["image"].as_str().unwrap().to_string()),
                        ));
                    }
                    Err(_) => {
                        println!(
                            "[df] error parsing date: {}",
                            data_event["startDate"].as_str().unwrap()
                        )
                    }
                }
            }
        }

        result
    }
}
