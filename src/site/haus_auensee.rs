use html_escape::decode_html_entities;
use reqwest::header;
use reqwest::header::HeaderMap;
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::site::eventim::Eventim;
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands};
use crate::tools::date::parse_short_date;
use crate::{Event, Http, Location, Site};

pub struct HausAuensee<'l> {
    location: Location<'l, 'l, 'l>,
    eventim_slug: String,
}

impl HausAuensee<'_> {
    pub(crate) fn new_auensee() -> Self {
        Self {
            location: Location {
                slug: "ha",
                name: "Haus Auensee",
                website: "https://www.haus-auensee-leipzig.de/",
            },
            eventim_slug: String::from("haus-auensee-leipzig-7301"),
        }
    }
    pub(crate) fn new_park() -> Self {
        Self {
            location: Location {
                slug: "pb",
                name: "Parkbühne im Clara-Zetkin-Park",
                website: "https://www.parkbuehne-leipzig.com/",
            },
            eventim_slug: String::from("parkbuehne-clara-zetkin-park-leipzig-7223"),
        }
    }
}

impl HausAuensee<'_> {
    fn load_events(&self, http: &Http, category: &str) -> Vec<Event> {
        // same backend as https://www.mawi-concert.de/

        let mut result = Vec::new();

        let base_url = self.location.website;

        let get_headers = http
            .get_headers(&(base_url.to_string() + "?menus_id=2"))
            .unwrap();
        let cookie = get_headers.get("set-cookie").unwrap().to_str().unwrap();

        let mut request_headers = HeaderMap::new();
        request_headers.insert(header::COOKIE, cookie.parse().unwrap());
        request_headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());

        let mut page = 1;
        loop {
            let page_number = page.to_string();

            let mut payload = HashMap::new();
            payload.insert("category", category);
            payload.insert("pageSize", "16");
            payload.insert("pageNumber", page_number.as_str());

            let data = http.post_json(
                &(base_url.to_string() + "worker/get_events_inc.php"),
                payload,
                request_headers.clone(),
            );

            let items = data
                .as_object()
                .unwrap()
                .get("items")
                .unwrap()
                .as_array()
                .unwrap();
            for item in items {
                let item = item.as_object().unwrap();
                let subtitle = item.get("subtitle").unwrap().as_str().unwrap();
                let parts: Vec<&str> = subtitle.split(" | ").collect();
                let date = parts.first().unwrap();
                if date.len() > 10 {
                    continue;
                }

                let title = item.get("title").unwrap().as_str().unwrap().to_string();
                let mut evt = Event::new(
                    decode_html_entities(title.as_str()).to_string(),
                    parse_short_date(date),
                    self.location.borrow(),
                    base_url.to_string()
                        + "?menus_id=2&solo=1&id="
                        + item
                            .get("id")
                            .unwrap()
                            .as_i64()
                            .unwrap()
                            .to_string()
                            .as_str(),
                    Some(base_url.to_string() + item.get("image").unwrap().as_str().unwrap()),
                );

                evt.add_band(evt.name.clone());

                result.push(evt);
            }

            let total = data
                .as_object()
                .unwrap()
                .get("total")
                .unwrap()
                .as_i64()
                .unwrap();

            if total <= (page * 16) {
                break;
            }

            page += 1;
        }

        result
    }
}

impl Site for HausAuensee<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut hard_heavy = self.load_events(http, "15");

        let eventim = Eventim::new(self.eventim_slug.as_str(), http);
        let has_metal_band = HasMetalBands {};

        let mut misc_result = self.load_events(http, "9");
        for evt in misc_result.iter_mut() {
            if eventim.is_it_metal(evt.borrow()) {
                hard_heavy.push(evt.clone());
                continue;
            }

            for band in evt.bands.iter_mut() {
                spirit_of_metal::find_band(band, http);
                metallum::find_band(band, http);
            }

            if has_metal_band.is_it_metal(evt.borrow()) {
                hard_heavy.push(evt.clone());
            }
        }

        hard_heavy
    }
}
