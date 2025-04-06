use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::event::{Event, Location};
use crate::Http;

pub(crate) mod anker;
pub(crate) mod arena;
pub(crate) mod bandcommunity;
pub(crate) mod conne_island;
pub(crate) mod darkaffair;
pub(crate) mod darkflower;
mod eventim;
mod facebook;
pub(crate) mod felsenkeller;
pub(crate) mod festivals;
pub(crate) mod forum;
mod google_calendar;
pub(crate) mod haus_auensee;
pub(crate) mod hellraiser;
mod metallum;
pub(crate) mod moritzbastei;
pub(crate) mod muehlkeller;
pub(crate) mod noels;
pub(crate) mod soltmann;
mod spirit_of_metal;
pub(crate) mod taeubchenthal;
mod tixforgigs;
pub(crate) mod tv_club;
pub(crate) mod ut_connewitz;
pub(crate) mod wavegothictreffen;
pub(crate) mod werk2;

pub trait Site {
    fn get_location(&self) -> &Location;
    fn fetch_events(&self, http: &Http) -> Vec<Event>;
}

trait Filter {
    fn is_it_metal(&self, evt: &Event) -> bool;
}

fn parse_linked_data_events(html: &str) -> Vec<Value> {
    lazy_static! {
        static ref REG: Regex =
            Regex::new(r#"(?si)<script type=[""']application/ld\+json[""']>(.*?)</script>"#)
                .unwrap();

        // bandcommunity: spotify plugin breaks json
        static ref CLEANUP: Regex =
            Regex::new(r#"(?si)<div class="cmplz-placeholder-parent">.*?</div>|<iframe.*?</iframe>"#).unwrap();
    }

    let mut result: Vec<Value> = Vec::new();

    for cap in REG.captures_iter(html) {
        let mut json = cap[1].replace("\t", " ");
        json = CLEANUP.replace_all(&json, "").to_string();

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

    result
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
