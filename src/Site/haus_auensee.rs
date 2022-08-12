use std::borrow::Borrow;

use chrono::NaiveDate;
use html_escape::decode_html_entities;
use regex::{Match, Regex};

use crate::site::eventim::Eventim;
use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands};
use crate::{Event, Location, Site, HTTP};

const URL: &str = "https://haus-auensee-leipzig.de/";

pub struct HausAuensee {
    location: Location,
}

impl HausAuensee {
    pub(crate) fn new() -> Self {
        Self {
            location: Location {
                slug: "ha".to_string(),
                name: "Haus Auensee".to_string(),
                website: URL.to_string(),
            },
        }
    }
}

fn parse_int(data: Option<Match>) -> u32 {
    data.unwrap().as_str().to_string().parse().unwrap()
}

impl Site for HausAuensee {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self) -> Vec<Event> {
        let http = HTTP::new();

        let mut result = Vec::new();

        let html = http.get(&*(URL.to_string() + "/?categorie=1"));

        let wrap_reg = Regex::new("(?is)<div class=\"md-col md-col-8\">.*?</a>\\s+</div>").unwrap();
        let main_reg = Regex::new(
            "(?is)<a href=\"(?P<url>[^<>]*?)\" class=\"dates-overview-item .*?>(?P<day>[0-2][0-9])[.]<.*?>(?P<month>[0-1][0-9])[.](?P<year>[0-9][0-9])<.*?<h3.*?>(?P<name>.*?)</h3>",
        )
        .unwrap();
        let image_reg = Regex::new("(?i)<img src=\"(.*?)\".*class=\"block col-12\"").unwrap();
        let split_reg = Regex::new(r"\s[+&]\s").unwrap();

        let eventim = Eventim::new("haus-auensee-leipzig-7301".to_string(), http.borrow());
        let has_metal_band = HasMetalBands {};

        let html = wrap_reg
            .captures(html.as_str())
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();

        for captures in main_reg.captures_iter(html) {
            let name = decode_html_entities(captures.name("name").unwrap().as_str()).to_string();
            let year: i32 = parse_int(captures.name("year")) as i32;
            let month: u32 = parse_int(captures.name("month"));
            let day: u32 = parse_int(captures.name("day"));
            let url = URL.to_string() + &*captures.name("url").unwrap().as_str().to_string();
            let mut evt = Event::new(
                name.clone(),
                NaiveDate::from_ymd(2000 + year, month, day).and_hms(0, 0, 0),
                self.location.borrow(),
                url.clone(),
                None,
            );

            for chunk in split_reg.split(name.as_str()) {
                if chunk != "" {
                    evt.add_band(chunk.to_string());
                }
            }

            for band in evt.bands.iter_mut() {
                spirit_of_metal::find_band(band, http.borrow());
                metallum::find_band(band, http.borrow());
            }

            if eventim.is_it_metal(evt.borrow()) || has_metal_band.is_it_metal(evt.borrow()) {
                let sub_page = http.get(url.as_str());
                match image_reg.captures(sub_page.as_str()) {
                    None => {}
                    Some(cap) => {
                        evt.set_image(URL.to_string() + &*cap.get(1).unwrap().as_str().to_string());
                    }
                }

                result.push(evt);
            }
        }

        result
    }
}
