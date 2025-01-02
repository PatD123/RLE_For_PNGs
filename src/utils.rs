use std::fs;
use std::fs::File;
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

pub fn get_png_bytes(mut reader: png::Reader<File>) -> Vec<u8> {
    // Get all the RGB bytes
    let mut buf = vec![0; reader.output_buffer_size()];
    let frame = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..frame.buffer_size()];

    bytes.to_vec()
}