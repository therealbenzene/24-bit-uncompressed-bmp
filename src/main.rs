use std::{fs::File, io, io::Write};

struct BmpHeader {
    bitmat_signature_bytes: [u8; 2],
    file_size: u32,
    creator_1: u16,
    creator_2: u16,
    pixel_offset: u32,
}

impl BmpHeader {
    fn new() -> BmpHeader {
        BmpHeader {
            bitmat_signature_bytes: [66, 77],
            file_size: 54 + 786432,
            creator_1: 0,
            creator_2: 0,
            pixel_offset: 54,
        }
    }
}

struct BmpDibHeader {
    header_size: u32,
    width: i32,
    height: i32,
    num_planes: u16,
    bits_per_pixel: u16,
    compress_type: u32,
    data_size: u32,
    hres: i32,
    vres: i32,
    num_colors: u32,
    num_imp_colors: u32,
}

impl BmpDibHeader {
    fn new() -> BmpDibHeader {
        BmpDibHeader {
            header_size: 40,
            width: 512,
            height: 512,
            num_planes: 1,
            bits_per_pixel: 24,
            compress_type: 0,
            data_size: 0,
            hres: 3780,
            vres: 3780,
            num_colors: 0,
            num_imp_colors: 0,
        }
    }
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    fn new() -> Pixel {
        Pixel {
            b: 192,
            g: 107,
            r: 92,
        }
    }
}

// #[rustfmt::skip]
// unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8] {

//     std::slice::from_raw_parts(
//         ptr as *const _ as *const u8,
//         std::mem::size_of::<T>()
//     )
// }

fn main() {
    let mut f = File::create("image.bmp").unwrap();

    let header = BmpHeader::new();
    let header_dip = BmpDibHeader::new();

    f.write(&header.bitmat_signature_bytes).unwrap();
    f.write(&header.file_size.to_le_bytes()).unwrap();
    f.write(&header.creator_1.to_le_bytes()).unwrap();
    f.write(&header.creator_2.to_le_bytes()).unwrap();
    f.write(&header.pixel_offset.to_le_bytes()).unwrap();

    f.write(&header_dip.header_size.to_le_bytes()).unwrap();
    f.write(&header_dip.width.to_le_bytes()).unwrap();
    f.write(&header_dip.height.to_le_bytes()).unwrap();
    f.write(&header_dip.num_planes.to_le_bytes()).unwrap();
    f.write(&header_dip.bits_per_pixel.to_le_bytes()).unwrap();
    f.write(&header_dip.compress_type.to_le_bytes()).unwrap();
    f.write(&header_dip.data_size.to_le_bytes()).unwrap();
    f.write(&header_dip.hres.to_le_bytes()).unwrap();
    f.write(&header_dip.vres.to_le_bytes()).unwrap();
    f.write(&header_dip.num_colors.to_le_bytes()).unwrap();
    f.write(&header_dip.num_imp_colors.to_le_bytes()).unwrap();

    // let num_of_pixel = header_dip.width * header_dip.height;

    for j in 0..=header_dip.height {
        for i in 0..header_dip.width {
            //
            let r = i as f32 / (header_dip.width - 1) as f32;
            let g = j as f32 / (header_dip.height - 1) as f32;
            let b = 0.25;

            let mut pixel = Pixel::new();
            pixel.r = (255.990_f32 * r) as u8;
            pixel.g = (255.990_f32 * g) as u8;
            pixel.b = (255.990_f32 * b) as u8;

            f.write(&pixel.b.to_le_bytes()).unwrap();
            f.write(&pixel.g.to_le_bytes()).unwrap();
            f.write(&pixel.r.to_le_bytes()).unwrap();
        }

        let percentage = (j as f32 / header_dip.height as f32) * 100_f32;
        print!("\rScanlines Progress {:.0}%", percentage);
        io::stdout().flush().unwrap();
    }

    println!();
}
