use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::BufWriter;
use std::env;

mod utils;


fn write_to_ppm(data: &[u8]) -> Result<(), std::io::Error> {
    let mut f = File::options().append(true).open("test_images/test.ppm")?;

    // Writing PPM data
    for (i, d) in data.iter().enumerate() {
        let mut s = d.to_string();
        if i > 0 && i % 3 == 0 {
            s.push_str("\n");
        }
        else{
            s.push_str(" ");
        }
        f.write(s.as_bytes());
    }

    // End
    Ok(())
}

fn png_to_ppm(mut reader: png::Reader<File>) -> Result<(), std::io::Error>{

    let info = reader.info();
    let width = info.width;
    let height = info.height;

    let mut f = File::create("test_images/test.ppm")?;

    // Writing PPM P3 header
    let buf = ["P3\n", & width.to_string(), & format!(" {}\n", height.to_string()), "255\n"];
    for s in buf.iter() {
        println!("{:?}", s.as_bytes());
        f.write(s.as_bytes());
    }

    loop {
        let res = reader.next_row();
        match res {
            Ok(Some(r)) => {
                match write_to_ppm(r.data()) {
                    Ok(_) => println!("Converted a row of png to ppm"),
                    Err(e) => println!("{}", e),
                }
            },
            _ => break,
        };
    }
    

    Ok(())
}

fn pngrle(img_info: png::Info, data: &[u8]) -> Result<(), png::EncodingError>{
    // Rewrite to a new .pnle (for png rle)!
    let file = File::options().write(true).create(true).open("output_images/turtle.pnle").unwrap();
    let ref mut w = BufWriter::new(file);

    let encoder = png::Encoder::with_info(w, img_info)?;
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data);
    // writer.write_image_data(&data[0..data.len()-1]);

    Ok(())
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");

    let mut decoder = png::Decoder::new(File::open("test_images/turtle.png").unwrap());
    let mut reader = decoder.read_info().unwrap();

    // TODO
    // Seems like with the commented code very below, we can decode a png, extract all the metadata and the idat chunks
    // then encode them again to get a valid PNG. However, we can't rewrite all the metadata, and then not write all of
    // our data, and still manage to use the decoder.read_info() function.

    // Best soln I have is to write all the headers into a temp file with everything the same except the IDAT chunks.
    // The new IDAT chunks should reflect RLE. 
    // To re-encode the file, we open the temp file, expand all the rle's and write into a new png file.

    let mut buf = vec![0; reader.output_buffer_size()];
    let frame = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..frame.buffer_size()];
    let info = reader.info().clone();

    pngrle(info, bytes);
}

fn test() {
    let mut decoder = png::Decoder::new(File::open("output_images/turtle.pnle").unwrap());
    let mut reader = decoder.read_info().unwrap();
    println!("{:?}", reader.info());
}
