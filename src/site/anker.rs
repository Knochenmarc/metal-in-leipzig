use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::{Filter, Site};
use crate::tools::date::parse_german_date;
use crate::tools::Http;

pub struct Anker<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Anker<'_> {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "ank",
                name: "der ANKER",
                website: "https://anker-leipzig.de",
            },
        }
    }
}

impl Site for Anker<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let eventim = Eventim::new("der-anker-leipzig-7330", http.borrow());

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
                    let name = item["title"]["rendered"].as_str().unwrap().to_string();
                    let mut evt = Event::new(
                        decode_html_entities(&name).to_string(),
                        parse_german_date(&captures[2]).and_hms(0, 0, 0),
                        self.location.borrow(),
                        api_link,
                        None,
                    );

                    if eventim.is_it_metal(evt.borrow()) {
                        match item["_links"]["wp:featuredmedia"][0]["href"].as_str() {
                            None => (),
                            Some(url) => evt.set_image(
                                http.get_json(url)["guid"]["rendered"]
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                            ),
                        };

                        result.push(evt);
                    }

                    break 'api_loop;
                }
            }
        }

        result
    }
}
