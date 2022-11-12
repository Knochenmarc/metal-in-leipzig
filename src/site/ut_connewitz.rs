use std::borrow::Borrow;

use chrono::Datelike;
use regex::Regex;

use crate::site::{metallum, spirit_of_metal, Filter, HasMetalBands};
use crate::tools::date::parse_german_date;
use crate::{Event, Http, Location, Site};

pub(crate) struct UTConnewitz<'l> {
    location: Location<'l, 'l, 'l>,
}

impl UTConnewitz<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "ut",
                name: "UT Connewitz",
                website: "https://utconnewitz.de/",
            },
        }
    }
}
impl Site for UTConnewitz<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let split_name = Regex::new(r", | & ").unwrap();
        let clear_name = Regex::new(r" \(.*?\)$").unwrap();
        let month_reg =
            Regex::new("(?i)<a href=\".*?month=(\\d+)\" class=\"active\">(.*?)</a>").unwrap();

        let this_month_html = http
            .get("https://utconnewitz.de/index.php?article_id=1&category=MUSIK")
            .unwrap();

        let this_month: u8 = {
            let captures = month_reg.captures(this_month_html.as_str()).unwrap();
            captures.get(1).unwrap().as_str().parse().unwrap()
        };
        let next_month = if this_month + 1 < 13 {
            this_month + 1
        } else {
            1
        };

        let next_month_url = format!(
            "https://utconnewitz.de/index.php?article_id=1&clang=0&category=MUSIK&month={}",
            next_month
        );

        let this_year = chrono::Utc::now().year();
        let next_year = this_year + 1;

        let reg = Regex::new("(?is)<div class=\"event\" .*?</a><a name=\"(?P<id>\\d+)\"></a>.*?<div class=\"day\\s*\">(?P<day>\\d+)</div>.*?<div class=\"title-title\">(?P<title>.*?)</div>(?:.*?<img src=\"(?P<img>.*?)\")?").unwrap();
        let more_reg = Regex::new("(?is)(?:<h1>(?P<month>\\w+?)</h1></div>\\s*<div class=\"line\"></div>\\s*)?<div class=\"event\" .*?</a><a name=\"(?P<id>\\d+)\"></a>.*?<div class=\"day\\s*\">(?P<day>\\d+)</div>.*?<div class=\"title music\">.*?<br/>\\s+(?P<title>.*?)\\s+</div>(?:\\s*<div id=\"event\\d+\" class=\"dateinfo tmusic\" style=\"display: none\">\\s*<div class=\"image\">\\s*<img src=\"(?P<img>.*?)\")?").unwrap();
        let urls = [
            (
                "https://utconnewitz.de/index.php?article_id=1&category=MUSIK",
                &reg,
            ),
            (next_month_url.as_str(), &reg),
            (
                "https://utconnewitz.de/index.php?article_id=151&clang=0&category=MUSIK",
                &more_reg,
            ),
        ];

        let mut had_december = false;
        let has_metal_bands = HasMetalBands {};

        for (url, reg) in urls {
            let html = http.get(url).unwrap();

            let mut month = match month_reg.captures(html.as_str()) {
                Some(cap) => cap.get(2).unwrap().as_str(),
                None => "",
            };

            for capture in reg.captures_iter(html.as_str()) {
                let id = capture.name("id").unwrap().as_str();
                let title = capture.name("title").unwrap().as_str();
                let day = capture.name("day").unwrap().as_str();
                let img = capture
                    .name("img")
                    .map(|cap| format!("https://utconnewitz.de/{}", cap.as_str()));

                if capture.name("month").is_some() {
                    month = capture.name("month").unwrap().as_str().trim();
                }

                if !had_december && month == "December" {
                    had_december = true;
                }

                let year = if had_december && month != "December" {
                    next_year
                } else {
                    this_year
                };

                let mut evt = Event::new(
                    title.to_string(),
                    parse_german_date(format!("{} {} {}", day, month, year).as_str())
                        .and_hms(0, 0, 0),
                    self.location.borrow(),
                    format!("{}&event={}#{}", url, id, id),
                    img,
                );

                let chunks: Vec<&str> = split_name.split(title).collect();
                for chunk in chunks {
                    let chunk = clear_name.replace(chunk, "").to_string();
                    if !chunk.is_empty() {
                        evt.add_band(chunk);
                    }
                }

                for band in evt.bands.iter_mut() {
                    spirit_of_metal::find_band(band, http.borrow());
                    metallum::find_band(band, http.borrow());
                }

                if has_metal_bands.is_it_metal(evt.borrow()) {
                    result.push(evt);
                }
            }
        }

        result
    }
}
