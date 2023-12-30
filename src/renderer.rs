use std::fs::{remove_file, File};

use chrono::Utc;
use chrono_tz::Europe::Berlin;
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use serde_json::Value;

use crate::event::{Event, Location};

fn prepare_file(file: &str) -> File {
    remove_file("public/".to_owned() + &file).unwrap();
    File::create("public/".to_owned() + file).unwrap()
}

pub(crate) fn render(events: Vec<Vec<Event>>, locations: Vec<Location>) {
    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "/app/templates/")
        .unwrap();

    let now = Utc::now().with_timezone(&Berlin);
    let mut data = Map::new();
    data.insert(
        "update_time".to_string(),
        Value::String(now.format("%d.%m. %H:%M").to_string()),
    );
    data.insert("locations".to_string(), to_json(locations));
    data.insert("event_group".to_string(), to_json(events));

    hb.render_to_write("index", &data, prepare_file("index.html"))
        .unwrap();
    hb.render_to_write("recht", &data, prepare_file("recht.html"))
        .unwrap();
}
