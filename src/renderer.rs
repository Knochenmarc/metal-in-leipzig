use std::collections::{BTreeMap, HashMap};
use std::fs::{remove_file, File};

use chrono::{Datelike, NaiveDate, Utc, Weekday};
use chrono_tz::Europe::Berlin;
use handlebars::{no_escape, to_json, Handlebars};
use serde_json::value::Map;
use serde_json::Value;

use crate::event::{Event, Location};

fn prepare_file(file: &str) -> File {
    remove_file("public/".to_owned() + &file).unwrap_or(());
    File::create("public/".to_owned() + file).unwrap()
}

pub(crate) fn render(events: BTreeMap<NaiveDate, Vec<Event>>, locations: Vec<&Location>) {
    let mut hb = Handlebars::new();
    hb.register_template_file("event_list", "templates/event_list.hbs")
        .expect("template not found");
    hb.register_template_file("filter", "templates/filter.hbs")
        .expect("template not found");
    hb.register_template_file("head", "templates/head.hbs")
        .expect("template not found");
    hb.register_template_file("header", "templates/header.hbs")
        .expect("template not found");
    hb.register_template_file("index", "templates/index.hbs")
        .expect("template not found");
    hb.register_template_file("recht", "templates/recht.hbs")
        .expect("template not found");
    hb.register_template_file("filter-js", "public/js/filter.js")
        .expect("template not found");
    hb.register_template_file("style-css", "public/style.css")
        .expect("template not found");
    hb.register_escape_fn(no_escape);

    let now = Utc::now().with_timezone(&Berlin);
    let mut data = Map::new();
    data.insert(
        "update_time".to_string(),
        Value::String(now.format("%d.%m. %H:%M").to_string()),
    );
    data.insert(
        "date_modified".to_string(),
        Value::String(now.format("%Y-%m-%dT%H:%M:S").to_string()),
    );
    data.insert("locations".to_string(), to_json(locations));

    let mut date_slugs = HashMap::new();
    let mut dates = HashMap::new();
    for date in events.keys() {
        date_slugs.insert(date, date.format("%Y%m%d").to_string());

        let weekday = match date.weekday() {
            Weekday::Mon => "Mo.",
            Weekday::Tue => "Di.",
            Weekday::Wed => "Mi.",
            Weekday::Thu => "Do.",
            Weekday::Fri => "Fr.",
            Weekday::Sat => "Sa.",
            Weekday::Sun => "So.",
        };
        let full_date: String = if date.year() == Utc::now().year() {
            date.format(" %d.%m.").to_string()
        } else {
            date.format(" %d.%m. %Y").to_string()
        };
        dates.insert(date, weekday.to_owned() + &full_date);
    }

    data.insert("dates".to_string(), to_json(dates));
    data.insert("date_slugs".to_string(), to_json(date_slugs));
    data.insert("event_group".to_string(), to_json(events));

    hb.render_to_write("index", &data, prepare_file("index.html"))
        .unwrap();
    hb.render_to_write("recht", &data, prepare_file("recht.html"))
        .unwrap();
}
