use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::Read;

pub struct Config {
    pub func: String,
    pub input_filename: String,
    pub output_filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS");
        }

        let func = args[1].clone();
        let filename = args[2].clone();

        let mut input_filename = String::new();
        let mut output_filename = String::new();
        if func == "pngrle" {
            input_filename = format!("test_images/{0}", filename);
            output_filename = format!("output_images/{0}.pnle", &filename[..filename.len() - 4]);
        }
        else if func == "depngrle" {
            input_filename = format!("output_images/{0}.pnle", &filename[..filename.len() - 4]);
            output_filename = format!("output_images/{0}.png", &filename[..filename.len() - 4]);
        }
        else if func == "ppmconvert" {
            input_filename = format!("test_images/{0}", filename);
            output_filename = format!("output_images/{0}.ppm", &filename[..filename.len() - 4]);
        }

        println!("Input filename is {0}", input_filename);
        println!("Output filename is {0}", output_filename);

        Ok(
            Config {
                func: func,
                input_filename: input_filename,
                output_filename: output_filename,
            }
        )
    }
}

pub fn write_to_pnle(mut file: &File, val: u8, cnt: usize){
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

pub fn pngrle(img_info: png::Info, data: &[u8], config: Config) -> Result<(), png::EncodingError>{
    // Rewrite to a new .pnle (for png rle)!
    let file = File::options().append(true).open(&config.output_filename).unwrap();

    // RLE Here
    let n: usize = img_info.height as usize;
    let m: usize = 3 * img_info.width as usize; // PNG n x m
    for k in 0..3 { // For each channel
        for i in 0..n { // For rows
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

pub fn depngrle(config: Config) {
    // env::set_var("RUST_BACKTRACE", "full");
    let mut decoder = png::Decoder::new(File::open(&config.input_filename).unwrap());
    let header = decoder.read_header_info().unwrap();

    // Testing decompression
    decompress(config, header);
}

pub fn decompress(config: Config, header: &png::Info) -> std::io::Result<()>{
    let mut file = File::open(config.input_filename).unwrap();

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

            for _ in 0..cnt {
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
    let file = File::create(config.output_filename).unwrap();
    let ref mut w = BufWriter::new(file);

    // Write only image size as specified in the header info
    let encoder = png::Encoder::with_info(w, header.clone())?;
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data_buf[..width * height * 3]);

    Ok(())
}

pub fn write_to_ppm(mut f: &File, data: &[u8]) -> Result<(), std::io::Error> {
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

pub fn png_to_ppm(mut reader: png::Reader<File>, config: Config) -> Result<(), std::io::Error>{

    let info = reader.info();
    let width = info.width;
    let height = info.height;

    let mut f = File::create(config.output_filename)?;

    // Writing PPM P3 header
    let buf = ["P3\n", & width.to_string(), & format!(" {}\n", height.to_string()), "255\n"];
    for s in buf.iter() {
        f.write(s.as_bytes());
    }

    loop {
        let res = reader.next_row();
        match res {
            Ok(Some(r)) => {
                match write_to_ppm(&f, r.data()) {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            },
            _ => break,
        };
    }
    

    Ok(())
}
