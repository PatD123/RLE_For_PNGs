use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::env;
use std::process;

use image_compression::Config;

mod utils;

fn run(config: Config) {
    // Decode the png
    let decoder = png::Decoder::new(File::open(&config.input_filename).unwrap());

    if config.func == "pngrle" {
        let reader = decoder.read_info().unwrap();

        let info = reader.info().clone();
        let bytes = utils::get_png_bytes(reader);

        // To make a new .pnle, we need all the IHDR bytes (8 + 4 + 4 + 13 + 4)
        let mut meta_buf = [0; 33];
        let mut f = File::open(&config.input_filename).unwrap();
        f.read(&mut meta_buf);

        // Write PNG header bytes into .pnle
        let mut file = File::options().create(true).write(true).open(&config.output_filename).unwrap();
        file.write(&meta_buf);

        // Compression and RLE here.
        image_compression::pngrle(info, &bytes, config);
    }
    else if config.func == "depngrle" {
        image_compression::depngrle(config);
    }
    else if config.func == "ppmconvert" {
        let reader = decoder.read_info().unwrap();
        image_compression::png_to_ppm(reader, config);
    }
    else {
        println!("Not an option");
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);
}