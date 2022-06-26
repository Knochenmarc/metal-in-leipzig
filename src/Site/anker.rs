use std::borrow::Borrow;

use regex::Regex;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::{Filter, Site};
use crate::tools::date::parse_german_date;
use crate::tools::HTTP;

pub struct Anker {
    location: Location,
}

impl Anker {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "ank".to_string(),
                name: "der ANKER".to_string(),
                website: "https://anker-leipzig.de".to_string(),
            },
        }
    }
}

impl Site for Anker {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();

        let eventim = Eventim::new("der-anker-leipzig-7330".to_string(), http.borrow());

        let api = http.get_json(
            "https://anker-leipzig.de/wp-json/wp/v2/event_listing?per_page=100"
                .to_string()
                .as_str(),
        );
        let api_items = api.as_array().unwrap();

        // api doesnt provide actual dates :(
        let html = http.get("https://anker-leipzig.de/va/veranstaltungen/");
        let reg: Regex = Regex::new("(?si)wpem-single-event-widget.*?<a href=\"(.*?)\".*?wpem-event-date-time-text\">.*?,\\s(.*?)<").unwrap();

        for captures in reg.captures_iter(html.as_str()) {
            let html_link = captures[1].to_string();

            'api_loop: for item in api_items {
                let api_link = item["link"].as_str().unwrap().to_string();
                if api_link.eq(&html_link) {
                    let image_url = match item["_links"]["wp:featuredmedia"][0]["href"].as_str() {
                        None => None,
                        Some(url) => Some(
                            http.get_json(url)["guid"]["rendered"]
                                .as_str()
                                .unwrap()
                                .to_string(),
                        ),
                    };

                    let evt = Event::new(
                        item["title"]["rendered"].as_str().unwrap().to_string(), //TODO: decode html
                        parse_german_date(&captures[2]).and_hms(0, 0, 0),
                        self.location.borrow(),
                        api_link,
                        image_url,
                    );

                    if eventim.is_it_metal(evt.borrow()) {
                        result.push(evt);
                    }

                    break 'api_loop;
                }
            }
        }

        return result;
    }
}
