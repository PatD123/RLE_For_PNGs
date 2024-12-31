# PNGRLE

Trying to do image compression (though idk if that's possible) for PNGs
using run length encoding. Learning Rust....

## Methodology
To do RLE encoding for PNGs, I realized I had to decode the PNG entirely and then apply
RLE encoding to it. At this stage, the decoding is basically just a PPM image, however, I
had to store header info for each PNG (as this info would necessary for reencoding). So I created
a file format called ```.pnle``` where the first several bytes are metadata and the following data would
basically just be the decoded RGB values. I applied RLE compression which is pretty cool for each channel
separately and stored the compressed versions one after each other (R_compr --> G_compr --> B_compr).
To re-encode, I encoded metadata and then uncompressed RLE and BOOM! We get the original image back.
Learned a lot about PNGs and even though I couldn't implement some of the LZ Compression and stuff, it was 
cool to learn about

## About PNGs
PNGs consist of 
1) A PNG identifier: 137 80 78 71 13 10 26 10 (**8 bytes**)
2) An IHDR chunk: All the metadata of the PNG (https://www.rfc-editor.org/rfc/rfc2083.html#section-12.11)
3) IDAT Chunk(s): A full chunk contains compressed versions of the RGB values
4) IEND: End of the PNG.

All IHDR chunks have **13 bytes**, so when the IHDR chunk is initially streamed in,
the **first 4 bytes** say that we have 13 bytes for us to read. After comes the IHDR 
identifier, which is also **4 bytes**. Next comes **13 bytes** of compressed data. 
Lastly, is the **4 bytes** of CRC. 

So in total, all the metadata comes out to be **8 + 4 + 4 + 13 + 4 = 33**. So when I decode and 
re-encode, you'll see this hardcoded number in to represent me reading in metadata first. This metadata
also contains how big the original image size is so this info helps to decompress the RLE encoding.

## About PNLEs
First 33 bytes is the metadata talked about before. The next set of bytes (**img_width * img_height * 3**) 
represent that total compressed using RLE encoding. I loop 3 three times for each channel. 

### RLE
If consecutive values of the same channel are different, we immediately just write that byte value. 
Otherwise, we keep a running count of the number of times we've seen this byte value. When we write the
count and byte value, it's in the format of *(0 byte_val cnt)*. If we encounter a 0 byte in the original, 
uncompressed format, we do the same thing but it would be *(0 0 1)*. The 1 depends on whether or not it's
consecutive.

## Decompression
1) Read first 33 bytes of metadata
2) De-RLE-code.
3) Using ```png```, encode and use original PNG compression to return to a *.png*

## Results
Works decently well compared to PPM, but of course loses out to the original PNG format.
For example, using the turtle.png image, the ppm is over 7MB, while the PNLE is a little over
2 MB, while the PNG is 0.67 MB.
