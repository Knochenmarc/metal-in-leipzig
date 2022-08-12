use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::event::{Event, Location};

pub(crate) mod anker;
pub(crate) mod arena;
pub(crate) mod bandcommunity;
pub(crate) mod conne_island;
pub(crate) mod darkflower;
mod eventim;
mod eventim_light;
pub(crate) mod felsenkeller;
pub(crate) mod haus_auensee;
mod metallum;
mod spirit_of_metal;

pub trait Site {
    fn get_locations(&self) -> Vec<Location>;
    fn fetch_events(&self) -> Vec<Event>;
}

trait Filter {
    fn is_it_metal(&self, evt: &Event) -> bool;
}

fn parse_linked_data_events(html: &str) -> Vec<Value> {
    lazy_static! {
        static ref REG: Regex =
            Regex::new(r#"(?si)<script type=[""']application/ld\+json[""']>(.*?)</script>"#)
                .unwrap();
    }

    let mut result: Vec<Value> = Vec::new();

    for cap in REG.captures_iter(html) {
        let json = cap[1].replace("\t", " ");
        let doc: Value = serde_json::from_str(json.as_str()).unwrap();
        if doc.is_array() {
            for event in doc.as_array().unwrap() {
                let typ = event["@type"].as_str().unwrap();
                if typ == "Festival" || typ.contains("Event") {
                    result.push(event.to_owned())
                }
            }
        } else if doc.is_object() {
            let typ = doc["@type"].as_str().unwrap();
            if typ == "Festival" || typ.contains("Event") {
                result.push(doc)
            }
        }
    }

    return result;
}

struct HasMetalBands {}

impl Filter for HasMetalBands {
    fn is_it_metal(&self, evt: &Event) -> bool {
        for band in evt.bands.iter() {
            if band.spirit_link.is_some() || band.metallum_link.is_some() {
                return true;
            }
        }

        false
    }
}
