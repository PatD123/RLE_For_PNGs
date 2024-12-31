use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::io::BufWriter;
use needle::BoyerMoore;

// Util function to check if hard-coded byte offsets seem to be correct.
fn check_bytes(){
    let data = fs::read("output_images/test.pnle").expect("Couldn't open file");
    let needle = BoyerMoore::new(&b"\x49\x44\x41\x54"[..]);
    match needle.find_in(&data).next() {
        Some(a) => println!("{}", a),
        None => println!("Couldn't find anything"),
    }
    println!("{:?}", &data[175..250]);
    //[86, 90, 73, 107, 111]
}

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