use crate::event::{Event, Location};
use crate::site::anker::Anker;
use crate::site::Site;
use crate::tools::{ImageOptimizer, HTTP};

mod event;
mod site;
mod tools;

fn main() {
    let http = HTTP::new();

    let mut locations: Vec<Location> = vec![];
    let mut events: Vec<Event> = vec![];

    let sites = [Anker::new()];
    for site in &sites {
        events.extend(site.fetch_events());
        locations.extend(site.get_locations());
    }

    // events.sort_by(|a, b| b.date.cmp(&a.date));
    // locations.sort_by(|a, b| b.name.cmp(&a.name));

    for event in &mut events {
        println!("event");

        match &mut event.image {
            None => {}
            Some(img) => {
                println!("optimize");
                ImageOptimizer::optimize(img, &http)
            }
        }
    }
}
