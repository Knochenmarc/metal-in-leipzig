use crate::event::Image;
use crate::tools::Http;
use std::env::temp_dir;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

const LOCAL_DIR: &str = "public/";

pub fn optimize_image(img: &mut Image, http: &Http) {
    let local_path = LOCAL_DIR.to_string() + &img.public_avif_url;
    let local_path = Path::new(local_path.as_str());

    if !local_path.exists() {
        let tmp_path = temp_dir().to_str().unwrap().to_string() + "/" + &img.hash.to_string();
        let tmp_path = Path::new(tmp_path.as_str());
        {
            let raw = http.get_raw(&img.remote_url);
            if raw.is_ok() {
                let mut tmp = File::create(tmp_path).expect("could not create file");
                tmp.write_all(&raw.unwrap())
                    .expect("could not write temporary image file");

                tmp.sync_data().expect("file sync failed");
            }
        }

        if tmp_path.exists() {
            //todo: run async
            let mut magick = Command::new("magick")
                .args([
                    tmp_path.to_str().unwrap(),
                    "-resize",
                    "435",
                    "-strip", // removes metadata
                    "-define",
                    "heic:speed=1",
                    local_path.to_str().unwrap(),
                ])
                .spawn()
                .expect("could not start avif conversion");
            magick.wait().unwrap();

            remove_file(tmp_path).unwrap();
        }
    }

    if local_path.exists() {
        let (width, height) = get_dimension(local_path.to_str().unwrap());
        if width > 0 && height > 0 {
            img.set_size(width, height);
        }
    }
}

fn get_dimension(file_path: &str) -> (u32, u32) {
    let width_output = Command::new("magick")
        .args(["identify", "-format", "%w", file_path])
        .output();
    let height_output = Command::new("magick")
        .args(["identify", "-format", "%h", file_path])
        .output();
    let width = String::from_utf8(width_output.unwrap().stdout).unwrap();
    let width = u32::from_str(width.as_str()).unwrap();
    let height = String::from_utf8(height_output.unwrap().stdout).unwrap();
    let height = u32::from_str(height.as_str()).unwrap();

    (width, height)
}
