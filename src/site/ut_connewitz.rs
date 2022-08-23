use regex::Regex;

use crate::{Event, Location, Site, HTTP};

pub(crate) struct UTConnewitz {
    location: Location,
}

impl UTConnewitz {
    pub fn new() -> Self {
        Self {
            location: Location {
                slug: "ut".to_string(),
                name: "UT Connewitz".to_string(),
                website: "https://utconnewitz.de/".to_string(),
            },
        }
    }
}
impl Site for UTConnewitz {
    fn get_locations(&self) -> Vec<Location> {
        return vec![self.location.clone()];
    }

    fn fetch_events(&self, http: &HTTP) -> Vec<Event> {
        let mut result = Vec::new();

        let this_month_html =
            http.get("https://utconnewitz.de/index.php?article_id=1&category=MUSIK");

        let this_month: u8 = {
            let month_reg =
                Regex::new("(?i)<a href=\".*?month=(\\d+)\" class=\"active\">").unwrap();
            let captures = month_reg.captures(this_month_html.as_str()).unwrap();
            captures.get(1).unwrap().as_str().parse().unwrap()
        };
        let next_month = if this_month + 1 < 13 {
            this_month + 1
        } else {
            1
        };

        let htmls = [
            this_month_html,
            http.get(
                format!(
                    "{}{}",
                    "https://utconnewitz.de/index.php?article_id=1&clang=0&category=MUSIK&month=",
                    next_month
                )
                .as_str(),
            ),
            http.get("https://utconnewitz.de/index.php?article_id=151&clang=0&category=MUSIK"),
        ];

        for html in htmls {}

        result
    }
}
