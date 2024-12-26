use std::fs::File;

fn png_to_ppm(data: &[u8]) {
    println!("{}", data.len());
    
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
        Ok(Some(r)) => png_to_ppm(r.data()),
        _ => (),
    }

    // println!("{:?}", res.len());
}
