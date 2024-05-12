use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::event::BandInfo;
use crate::Http;

pub fn find_band(band: &mut BandInfo, http: &Http) {
    lazy_static! {
        static ref REG: Regex =
            Regex::new("(?is)<div class=\"BandResult\"><a href=\"(?P<url>.*?)\".*?<h3>(?P<name>.*?)(?: \\(.*?\\))?</h3>(?P<genre>.*?) - .*?<br/>").unwrap();
    }

    let name = &band.name;

    if name.len() < 6 {
        // name is too short to get a valid hit anyway
        return;
    }

    let url = format!(
        "{}{}{}{}",
        r"https://www.spirit-of-metal.com/liste_groupe.php?recherche_groupe=",
        name,
        r"&lettre=",
        name
    );
    let response = http.get(url.as_str());

    if response.is_err() {
        return;
    }

    let html = response.unwrap();

    let res: Vec<Captures> = REG.captures_iter(html.as_str()).collect();
    // Later: ungenau für links, aber genau genug für is-metal-check
    if res.len() >= 1 {
        let first = res.first().unwrap();
        band.genre = Some(first.name("genre").unwrap().as_str().to_string());
        band.metallum_link = Some(first.name("url").unwrap().as_str().to_string());
    }
}
