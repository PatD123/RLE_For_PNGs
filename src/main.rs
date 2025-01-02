use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::io::BufWriter;
use std::env;
use std::io;
use std::process;

mod utils;

fn write_to_pnle(mut file: &File, val: u8, cnt: usize){
    let cnt = cnt as u8;
    // Write however much it is
    if cnt == 1 {
        // If byte is 0, then we do 0 0 1
        if val == 0 {
            file.write(b"\0");
            file.write(&[val]);
            file.write(&[cnt]);
        }
        // Otherwise we just write the singular byte
        else {
            file.write(&[val]);
        }
    }
    else {
        file.write(b"\0");
        file.write(&[val]);
        file.write(&[cnt]);    
    }
}

fn pngrle(img_info: png::Info, data: &[u8]) -> Result<(), png::EncodingError>{
    // Rewrite to a new .pnle (for png rle)!
    let mut file = File::options().append(true).open("output_images/turtle.pnle").unwrap();

    // RLE Here
    let n: usize = img_info.height as usize;
    let m: usize = 3 * img_info.width as usize; // PNG n x m
    for k in (0..3){ // For each channel
        for i in (0..n) { // For rows
            let mut cnt = 1;
            let offset = i * m;

            for j in (k + 3..m).step_by(3) { // For RGB values
                let j = j as usize;

                // (0 val cnt) - RLE
                if data[offset + j] == data[offset + j - 3] {
                    // If consecutive r or g or b vals are equal, we incr cnt.
                    cnt += 1;
                }
                else{
                    write_to_pnle(&file, data[offset + j - 3], cnt);
                    cnt = 1;
                }
            }
            write_to_pnle(&file, data[offset + m - (3 - k)], cnt);
        }
    }

    Ok(())
}

struct Config {
    func: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS");
        }

        let func = args[1].clone();
        let filename = args[2].clone();

        Ok(
            Config {
                func: func,
                filename: filename,
            }
        )
    }
}

fn run(config: Config) {
    
    // Decode the png
    let mut decoder = png::Decoder::new(File::open("test_images/turtle.png").unwrap());
    let mut reader = decoder.read_info().unwrap();

    // Get all the RGB(A) bytes
    let mut buf = vec![0; reader.output_buffer_size()];
    let frame = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..frame.buffer_size()];
    let info = reader.info().clone();

    // To make a new .pnle, I need all the IHDR bytes (8 + 4 + 4 + 13 + 4)
    let mut buf = [0; 33];
    let f = File::open("test_images/turtle.png").unwrap();
    let mut handle = f.take(33);
    match handle.read(&mut buf) {
        Ok(_) => println!("{:?}", buf),
        Err(e) => println!("Error"),
    };

    // Write PNG header bytes into .pnle
    let mut file = File::options().create(true).write(true).open("output_images/turtle.pnle").unwrap();
    file.write(&buf);

    // Compression and RLE here.
    pngrle(info, bytes);
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);

    // TODO
    // - Turn into a CLI tool
    //      - PNG to PPM
    //      - PNG to RLE
    //      - RLE to PNG
    // - Add command line arguments
    // - Refactor and modularize code
}

fn test() {
    // env::set_var("RUST_BACKTRACE", "full");
    let mut decoder = png::Decoder::new(File::open("output_images/turtle.pnle").unwrap());
    let mut header = decoder.read_header_info().unwrap();

    // Testing decompression
    decompress(header);
}

fn decompress(header: &png::Info) -> io::Result<()>{
    let mut file = File::open("output_images/turtle.pnle").unwrap();

    // Get metadata first
    let mut meta_buf = [0; 33];
    file.read(&mut meta_buf);

    println!("HEADER : {:?}", meta_buf);

    // Decompressing RLE encoding @ byte 33
    let width: usize = header.width as usize;
    let height: usize = header.height as usize;
    let max_size = width * height * 3 * 3; // * 3 for RGB * 3 for RLE for all 0
    let mut data_buf = vec![0; max_size];
    // Read into data_buf
    let num_bytes = file.read(&mut data_buf)?;

    println!("NUM_BYTES: {}", num_bytes);

    // Starting decompression
    let mut temp_buf = Vec::new();
    let mut i = 0;
    while i < max_size {
        // If singular not repeated byte, we instantly push
        if data_buf[i] != 0 {
            temp_buf.push(data_buf[i]);
            i += 1;
        }
        // Otherwise, we take the next 3.
        else {
            let val = data_buf[i + 1];
            let cnt = data_buf[i + 2];

            if val == 0 && cnt == 0 {
                break;
            }

            for _ in (0..cnt) {
                temp_buf.push(val);
            }

            i += 3;
        }
    }

    // Merge all RGB values again
    let mut i: usize = 0;
    let r = &temp_buf[0..width * height];
    let g = &temp_buf[width * height..2*width * height];
    let b = &temp_buf[2*width * height..];

    // Zip all the iterators into tuples
    let zipped = r.iter().zip(g).zip(b);
    for ((r, g), b) in zipped {
        data_buf[i] = *r;
        data_buf[i + 1] = *g;
        data_buf[i + 2] = *b;
        i += 3;
    }

    // Get file and make a buffered writer
    let path = Path::new("output_images/turtle.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    // Write only image size as specified in the header info
    let encoder = png::Encoder::with_info(w, header.clone())?;
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data_buf[..width * height * 3]);

    Ok(())
}
