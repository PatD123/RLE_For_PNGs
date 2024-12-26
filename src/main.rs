use std::fs::File;
use std::io::Write;

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

    while true {
        let res = reader.next_row();
        match res {
            Ok(Some(r)) => {
                match write_to_ppm(r.data()) {
                    Ok(o) => println!("Converted a row of png to ppm"),
                    Err(e) => println!("{}", e),
                }
            },
            _ => break,
        };
    }
    

    Ok(())
}

fn main() {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("test_images/turtle.png").unwrap());

    let mut reader = decoder.read_info().unwrap();

    png_to_ppm(reader);
}
