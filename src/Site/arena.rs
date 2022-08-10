use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::{Filter, Site};
use crate::tools::date::parse_short_date;
use crate::tools::HTTP;

const URL: &str = "https://www.quarterback-immobilien-arena.de";

pub(crate) struct Arena {
    location: Location,
    location_id: i8,
    eventim_id: String,
}

impl Arena {
    pub(crate) fn new_red_bull() -> Self {
        Self {
            location: Location {
                slug: "rb".to_string(),
                name: "Red Bull Arena".to_string(),
                website: URL.to_owned() + "/red-bull-arena",
            },
            location_id: 2,
            eventim_id: "red-bull-arena-16304".to_string(),
        }
    }

    pub(crate) fn new_quarterback() -> Self {
        Self {
            location: Location {
                slug: "qi".to_string(),
                name: "QUARTERBACK Immobilien ARENA".to_string(),
                website: URL.to_owned() + "/quarterback-immobilien-arena",
            },
            location_id: 1,
            eventim_id: "quarterback-immobilien-arena-leipzig-383".to_string(),
        }
    }

    pub(crate) fn new_festwiese() -> Self {
        Self {
            location: Location {
                slug: "fw".to_string(),
                name: "Festwiese Leipzig".to_string(),
                website: URL.to_owned() + "/festwiese-leipzig",
            },
            location_id: 3,
            eventim_id: "festwiese-leipzig-7410".to_string(),
        }
    }
}

impl Site for Arena {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();
        let mut result = Vec::new();

        let eventim = Eventim::new(self.eventim_id.to_string(), http.borrow());
        let reg: Regex = Regex::new(
            "(?si)<div class=\"event\".*?<a href=\"(.*?)\">.*?<source srcset=\"(.*?)\" media=\"[(]max-width: 320px[)]\">.*?<div>\\w+,\\s+(\\d\\d\\.\\d\\d\\.\\d\\d\\d\\d)</div>.*?<h2>(.*?)</h2>",
        )
        .unwrap();

        for page in 1..100 {
            let html = http.get(&(URL.to_owned() + "/events-tickets/events?tx_ifabeventmanagementextend_searchfilter[eventSearch][category]=1"
            +"&tx_ifabeventmanagementextend_searchfilter[eventSearch][location]="+ &self.location_id.to_string()
            +"&tx_ifabeventmanagement_events[@widget_0][currentPage]="+&page.to_string())
            );

            for captures in reg.captures_iter(html.as_str()) {
                let evt = Event::new(
                    decode_html_entities(&captures[4]).to_string(),
                    parse_short_date(&captures[3]).and_hms(0, 0, 0),
                    self.location.borrow(),
                    URL.to_owned() + captures[1].borrow(),
                    Option::Some(URL.to_owned() + captures[2].borrow()),
                );
                if eventim.is_it_metal(evt.borrow()) {
                    result.push(evt);
                }
            }
            if !html.contains("<li class=\"next\">") {
                break;
            }
        }

        result
    }
}
