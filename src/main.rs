use std::{cell::RefCell, fs::File, io, io::Write};

struct BmpHeader {
    bitmap_signature_bytes: [u8; 2],
    file_size: u32,
    creator_1: u16,
    creator_2: u16,
    pixel_offset: u32,
}

impl BmpHeader {
    fn new(size: u32) -> BmpHeader {
        BmpHeader {
            bitmap_signature_bytes: [66, 77],
            file_size: 54 + size,
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
    h_res: i32,
    v_res: i32,
    num_colors: u32,
    num_imp_colors: u32,
}

impl BmpDibHeader {
    fn new(width: i32, height: i32) -> BmpDibHeader {
        BmpDibHeader {
            header_size: 40,
            width,
            height,
            num_planes: 1,
            bits_per_pixel: 24,
            compress_type: 0,
            data_size: 0,
            h_res: 2835,
            v_res: 2835,
            num_colors: 0,
            num_imp_colors: 0,
        }
    }
}

struct Pixel {
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

struct Image {
    width: i32,
    height: i32,
    f: RefCell<File>,
}

impl Image {
    fn new(width: i32, height: i32, path: String) -> Image {
        let file = RefCell::new(File::create(path).unwrap());

        let size = (width * height * 3) as u32;

        let header = BmpHeader::new(size);
        let header_dip = BmpDibHeader::new(width, height);

        file.borrow_mut()
            .write(&header.bitmap_signature_bytes)
            .unwrap();

        file.borrow_mut()
            .write(&header.file_size.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header.creator_1.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header.creator_2.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header.pixel_offset.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.header_size.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.width.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.height.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.num_planes.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.bits_per_pixel.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.compress_type.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.data_size.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.h_res.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.v_res.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.num_colors.to_le_bytes())
            .unwrap();

        file.borrow_mut()
            .write(&header_dip.num_imp_colors.to_le_bytes())
            .unwrap();

        return Image {
            width,
            height,
            f: file,
        };
    }

    fn init() {}

    fn write(&self, pixel: &Pixel) {
        self.f.borrow_mut().write(&pixel.b.to_le_bytes()).unwrap();
        self.f.borrow_mut().write(&pixel.g.to_le_bytes()).unwrap();
        self.f.borrow_mut().write(&pixel.r.to_le_bytes()).unwrap();
    }
}

fn main() {
    let image = Image::new(512, 512, String::from("image.bmp"));

    for j in 0..=image.width {
        for i in 0..image.width {
            //
            let r = i as f32 / (image.width - 1) as f32;
            let g = j as f32 / (image.height - 1) as f32;
            let b = 0.25;

            let mut pixel = Pixel::new();
            pixel.r = (255.999 as f32 * r) as u8;
            pixel.g = (255.999 as f32 * g) as u8;
            pixel.b = (255.999 as f32 * b) as u8;

            image.write(&pixel);
        }

        let percentage = (j as f32 / image.height as f32) * 100_f32;
        print!("\rScanline Progress {percentage:.0}%");
        io::stdout().flush().unwrap();
    }

    println!();
}
