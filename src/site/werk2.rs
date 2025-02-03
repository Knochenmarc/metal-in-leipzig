use std::borrow::Borrow;

use regex::Regex;

use crate::tools::date::parse_short_date;
use crate::{Event, Http, Location, Site};

const URL: &str = "https://www.werk-2.de";

pub(crate) struct Werk2<'l> {
    location: Location<'l, 'l, 'l>,
}

impl Werk2<'_> {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "w2",
                name: "WERK 2",
                website: URL,
            },
        }
    }
}

impl Site for Werk2<'_> {
    fn get_location(&self) -> &Location {
        self.location.borrow()
    }

    fn fetch_events(&self, http: &Http) -> Vec<Event> {
        let mut result = Vec::new();

        let list_reg = Regex::new("(?is)<ul .*?vak_liste.*?>(.*?)</ul>").unwrap();
        let reg = Regex::new("(?is)<img.*? src='(?P<img>.*?)'.?/>.*?<p class='typen'>(?P<typen>.*?)</p>.*?<h2><a href='(?P<url>.*?)'>(?P<name>.*?)</a>.*?<p>(?P<date>\\d\\d\\.\\d\\d\\.\\d\\d)").unwrap();

        let html = http
            .get("https://www.werk-2.de/programm/suche/?q=Metal")
            .unwrap();
        let list_html = list_reg.captures(&html).unwrap().get(1).unwrap().as_str();

        for captures in reg.captures_iter(list_html) {
            let url = captures.name("url").unwrap().as_str();
            let name = captures.name("name").unwrap().as_str();
            let img = captures.name("img").unwrap().as_str();
            let typen = captures.name("typen").unwrap().as_str();
            let date = captures.name("date");

            if date.is_some()
                && typen.starts_with("Programm > ")
                && typen.to_lowercase().contains("metal")
            {
                // http head image
                let image = {
                    let formats = ["_detail", "_403"];
                    let urls = formats.map(|s| format!("{}{}", URL, img.replace("_liste_250", s)));

                    let mut result = format!("{}{}", URL, img);
                    for url in urls {
                        if http.exists(url.as_str()) {
                            result = url;
                            break;
                        }
                    }

                    Some(result)
                };

                let date = date.unwrap().as_str();
                let date = if date.len() == 8 {
                    let mut longer_date = date[..6].to_owned();
                    longer_date.push_str("20");
                    longer_date.push_str(&date[6..]);
                    parse_short_date(longer_date.as_str())
                } else {
                    parse_short_date(date)
                };

                let evt = Event::new(
                    name.to_string(),
                    date,
                    self.location.borrow(),
                    format!("{}{}", URL, url),
                    image,
                );
                result.push(evt);
            }
        }

        result
    }
}
