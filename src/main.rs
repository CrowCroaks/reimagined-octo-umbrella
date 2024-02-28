use structopt::StructOpt;
use image::{io::Reader as ImgReader, Pixel, RgbaImage};
use std::{env::current_dir, path::PathBuf};

#[derive(StructOpt)]
#[structopt(name = "Data about image")]

struct ImgData {
    
    //Path to image file
    #[structopt(short = "p", long, default_value = "images/pepa.jpg")]
    img_path: PathBuf,

    //Path to metadata file
    #[structopt(short, long, default_value = "images/")]
    metadata_path: String,

}

impl ImgData {

    fn prove(&self) -> anyhow::Result<()> {
        let img = ImgReader::open(&self.img_path)?.decode()?.to_rgba8();
        let new_img = red_channel(img);
        new_img.save("images/new_pepa.jpg");
        Ok(())
    }

    fn verify(&self) {

    }
}

fn red_channel(img: RgbaImage) -> RgbaImage {
    let mut new_img = img.clone();
    for pixel in new_img.pixels_mut() {
        for c in pixel.channels_mut()[1..3].iter_mut(){
            *c = 0;
        }
    }
    new_img
}

fn main() {
    let data = ImgData::from_args();
    data.prove();
}
