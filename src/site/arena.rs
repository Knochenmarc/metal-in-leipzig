use std::borrow::Borrow;

use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::{Filter, Site};
use crate::tools::date::parse_short_date;
use crate::tools::Http;

const URL: &str = "https://www.quarterback-immobilien-arena.de";

pub(crate) struct Arena<'l> {
    location: Location<'l, 'l, 'l>,
    location_id: i8,
    eventim_id: &'l str,
}

impl Arena<'_> {
    pub(crate) fn new_red_bull() -> Self {
        Self {
            location: Location {
                slug: "rb",
                name: "Red Bull Arena",
                website: "https://www.quarterback-immobilien-arena.de/red-bull-arena",
            },
            location_id: 2,
            eventim_id: "red-bull-arena-16304",
        }
    }

    pub(crate) fn new_quarterback() -> Self {
        Self {
            location: Location {
                slug: "qi",
                name: "QUARTERBACK Immobilien ARENA",
                website: "https://www.quarterback-immobilien-arena.de/quarterback-immobilien-arena",
            },
            location_id: 1,
            eventim_id: "quarterback-immobilien-arena-leipzig-383",
        }
    }

    pub(crate) fn new_festwiese() -> Self {
        Self {
            location: Location {
                slug: "fw",
                name: "Festwiese Leipzig",
                website: "https://www.quarterback-immobilien-arena.de/festwiese-leipzig",
            },
            location_id: 3,
            eventim_id: "festwiese-leipzig-7410",
        }
    }
}

impl Site for Arena<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let eventim = Eventim::new(self.eventim_id, http.borrow());
        let reg: Regex = Regex::new(
            "(?si)<div class=\"event\".*?<a href=\"(.*?)\">.*?<source srcset=\"(.*?)\" media=\"[(]max-width: 320px[)]\">.*?<div>\\w+,\\s+(\\d\\d\\.\\d\\d\\.\\d\\d\\d\\d)</div>.*?<h2>(.*?)</h2>",
        )
        .unwrap();

        for page in 1..100 {
            let html = http.get(&(URL.to_owned() + "/events-tickets/events?tx_ifabeventmanagementextend_searchfilter[eventSearch][category]=1"
            +"&tx_ifabeventmanagementextend_searchfilter[eventSearch][location]="+ &self.location_id.to_string()
            +"&tx_ifabeventmanagement_events[@widget_0][currentPage]="+&page.to_string())
            ).unwrap();

            for captures in reg.captures_iter(html.as_str()) {
                let evt = Event::new(
                    decode_html_entities(&captures[4]).to_string(),
                    parse_short_date(&captures[3]),
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
