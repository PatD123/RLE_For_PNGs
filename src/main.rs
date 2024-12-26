use std::fs::File;
use std::io::Write;

fn png_to_ppm(data: &[u8]) -> Result<(), std::io::Error> {
    let width = data.len() / 3;

    let mut f = File::create("test_images/test.ppm")?;

    // Writing PPM P3 header
    let buf = ["P3\n", &width.to_string(), " 1\n", "255\n"];
    for s in buf.iter() {
        println!("{:?}", s.as_bytes());
        f.write(s.as_bytes());
    }

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

fn main() {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("test_images/turtle.png").unwrap());

    let mut reader = decoder.read_info().unwrap();

    // Allocate the output buffer.
    // let mut buf = vec![0; reader.output_buffer_size()];

    // Read the next frame. An A PNG might contain multiple frames.
    // let info = reader.next_frame(&mut buf).unwrap();

    // Grab the bytes of the image.
    // let bytes = &buf[..info.buffer_size()]; // H x W x 3

    let res = reader.next_row();
    match res {
        Ok(Some(r)) => {
            match png_to_ppm(r.data()) {
                Ok(o) => println!("Converted a row of png to ppm"),
                Err(e) => println!("{}", e),
            }
        },
        _ => println!("PNG reader couldn't read a row of png."),
    };

    // println!("{:?}", res.len());
}
