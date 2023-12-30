use std::borrow::Borrow;

use chrono::{Datelike, NaiveDate};
use html_escape::decode_html_entities;
use regex::Regex;

use crate::event::{Event, Location};
use crate::site::eventim::Eventim;
use crate::site::eventim_light::EventimLight;
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands, Site};
use crate::tools::HTTP;

pub struct Felsenkeller {
    location: Location,
}

impl Felsenkeller {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "fk".to_string(),
                name: "Felsenkeller".to_string(),
                website: "https://felsenkeller-leipzig.com".to_string(),
            },
        }
    }
}

impl Site for Felsenkeller {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();
        let mut result = Vec::new();

        let split_name = Regex::new(r"\s&\sBand|\s[&x+|•]\s").unwrap();
        let reg: Regex = Regex::new(r#"(?is)<div class="wp-block-columns has-3-columns.*?data-cat="\D*?".*?<img src='(?P<img>.*?)'.*?<span class="date">(?P<date>.*?)</span>.*?<p class="event-name">(?P<name>.*?)</?span.*?class="event-details">(?P<detail>.*?)data-url(?:.*?href="(?P<tix>.*?)" target="_blank">Tickets</a>)?"#).unwrap();
        let html = http.get("https://www.felsenkeller-leipzig.com/programm/");

        let eventim = Eventim::new("felsenkeller-leipzig-7394".to_string(), http.borrow());
        let eventim_light =
            EventimLight::new("573474f9e4b0e47b2924e6a3".to_string(), http.borrow());
        let has_metal_band = HasMetalBands {};

        let this_year = chrono::Utc::now().year();
        let next_year = this_year + 1;
        let mut had_december = false;

        for captures in reg.captures_iter(html.as_str()) {
            let raw_date = captures.name("date").unwrap().as_str();
            let year = if &raw_date[3..5] == "12" {
                had_december = true;
                this_year
            } else if had_december {
                next_year
            } else {
                this_year
            };
            let date = NaiveDate::from_ymd(
                year,
                raw_date[3..5].parse().unwrap(),
                raw_date[0..2].parse().unwrap(),
            );

            let image = match captures.name("img") {
                None => None,
                Some(payload) => Some(payload.as_str().to_string()),
            };

            let name = {
                let mut name =
                    decode_html_entities(captures.name("name").unwrap().as_str()).to_string();
                name = name.replace(" (Ausverkauft)", "");
                name = name.replace("Abgesagt: ", "");
                name = name.replace("Vortrag: ", "");
                name = name.replace("Verschoben: ", "");
                name = name.replace("Kultursommer: ", "");
                name.trim().to_string()
            };

            let mut evt = Event::new(
                name.clone(),
                date.and_hms(0, 0, 0),
                self.location.borrow(),
                "https://www.felsenkeller-leipzig.com/programm/".to_string(),
                image,
            );

            let chunks: Vec<&str> = split_name.split(name.as_str()).collect();
            for chunk in chunks {
                if chunk != ""
                    && !chunk.contains("Tour")
                    && !chunk.contains("TOUR")
                    && !chunk.contains("Live")
                    && !chunk.contains("LIVE")
                    && !chunk.contains(this_year.to_string().as_str())
                    && !chunk.contains(next_year.to_string().as_str())
                {
                    evt.add_band(chunk.to_string());
                }
            }

            for band in evt.bands.iter_mut() {
                spirit_of_metal::find_band(band, http.borrow());
                metallum::find_band(band, http.borrow());
            }

            let detail = captures.name("detail").unwrap().as_str();
            let tix_url = match captures.name("tix") {
                None => "",
                Some(m) => m.as_str(),
            };
            if (tix_url.contains("www.eventim.de") && eventim.is_it_metal(evt.borrow()))
                || (tix_url.contains("eventim-light.com")
                    && eventim_light.is_it_metal(evt.borrow()))
                || tix_url.contains("impericon.com")
                || detail.contains("Avocado Booking")
                || detail.contains("metal.de")
                || (has_metal_band.is_it_metal(evt.borrow())
                    && !detail.to_lowercase().contains("pop-band"))
            {
                result.push(evt);
            }
        }

        result
    }
}