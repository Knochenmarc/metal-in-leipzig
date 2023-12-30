use std::env::temp_dir;
use std::fs::{remove_file, File};
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::Command;

use jpeg_decoder::Decoder;

use crate::event::Image;
use crate::tools::HTTP;

const LOCAL_DIR: &str = "public/";

pub fn optimize_image(img: &mut Image, http: &HTTP) {
    let local_path = LOCAL_DIR.to_string() + &img.public_jpg_url;
    let local_path = Path::new(local_path.as_str());

    if false == local_path.exists() {
        let tmp_path = temp_dir().to_str().unwrap().to_string() + "/" + &img.hash;
        {
            let raw: Vec<u8> = http.get_raw(&img.remote_url);
            let mut tmp = File::create(&tmp_path).expect("could not create file");
            tmp.write_all(&*raw)
                .expect("could not write temporary image file");

            tmp.sync_data().expect("file sync failed");
        }

        let mut c1 = Command::new("convert")
            .args([
                "-resize",
                "300",
                "-strip",
                &tmp_path,
                local_path.to_str().unwrap(),
            ])
            .spawn()
            .expect("could not start jpeg conversion");
        let mut c2 = Command::new("convert")
            .args([
                "-resize",
                "300",
                "-strip",
                "-define",
                "heic:speed=2",
                &tmp_path,
                (LOCAL_DIR.to_owned() + &img.public_avif_url).as_str(),
            ])
            .spawn()
            .expect("could not start avif conversion");

        c1.wait().unwrap();
        c2.wait().unwrap();

        remove_file(tmp_path).unwrap();
    }

    let (width, height) = get_dimension(local_path.to_str().unwrap());
    if width > 0 && height > 0 {
        img.set_size(width, height);
    }
}

fn get_dimension(file_path: &str) -> (u32, u32) {
    let file = File::open(file_path).expect("failed to open file");
    let mut decoder = Decoder::new(BufReader::new(file));
    decoder.read_info().expect("failed to read metadata");
    let metadata = decoder.info().expect("failed extracting image metadata");
    (metadata.width as u32, metadata.height as u32)
}
