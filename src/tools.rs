use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use image::GenericImageView;
use reqwest::blocking::Client;
use serde_json::Value;

use crate::event::Image;

const LOCAL_DIR: &str = "/public/";

pub struct HTTP {
    client: Client,
}

impl HTTP {
    pub(crate) fn new() -> HTTP {
        HTTP {
            client: Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> String {
        println!("get: {:?}", url);

        return self.client.get(url)
            .header("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:96.0) Gecko/20100101 Firefox/96.0")
            .header("Accept","text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "de,en-US;q=0.7,en;q=0.3")
            .header("Accept-Encoding", "identity")
            .header("DNT", "1")
            .header("Pragma", "no-cache")
            .header("Cache-Control", "no-cache")
            .send().unwrap().text().unwrap();
    }

    pub fn get_json(&self, url: &str) -> Value {
        let resp = self.get(url);
        return serde_json::from_str(resp.as_str()).unwrap();
    }
}

pub struct ImageOptimizer {}

impl ImageOptimizer {
    pub fn optimize(img: &mut Image, http: &HTTP) {
        if false == Path::new((LOCAL_DIR.to_owned() + &img.public_jpg_url).as_str()).exists() {
            let tmp_path = "/tmp/".to_owned() + &img.hash;
            let raw = http.get(&img.remote_url);
            {
                let mut tmp = File::create(&tmp_path).expect("could not create file");
                tmp.write_all(raw.as_ref())
                    .expect("could not write temporary image file");
                tmp.sync_data().expect("file sync failed");
            }

            let mut c1 = Command::new("convert")
                .arg("-resize")
                .arg("300")
                .arg("-strip")
                .arg(&tmp_path)
                .arg(LOCAL_DIR.to_owned() + &img.public_jpg_url)
                .spawn()
                .expect("could not start jpeg conversion");
            let mut c2 = Command::new("convert")
                .arg("-resize")
                .arg("300")
                .arg("-strip")
                .arg("-define")
                .arg("heic:speed=2")
                .arg(&tmp_path)
                .arg(LOCAL_DIR.to_owned() + &img.public_avif_url)
                .spawn()
                .expect("could not start avif conversion");

            c1.wait().unwrap();
            c2.wait().unwrap();

            remove_file(tmp_path).unwrap();
        }

        let (width, height) = image::open(LOCAL_DIR.to_owned() + &*img.public_jpg_url)
            .unwrap()
            .dimensions();
        if width > 0 && height > 0 {
            img.set_size(width, height);
        }
    }
}
