use std::{
    env,
};

use image::{
    io::Reader,
    DynamicImage,
    GrayImage,
    Luma,
};

fn er_di(img: &GrayImage, window_indices: &Vec<(i32,i32)>, er_or_di: bool) -> GrayImage {
    let mut out_img: GrayImage = GrayImage::new(img.width(), img.height());
    let mut pixel: Luma<u8>;
    for x in 0..img.width() {
        for y in 0..img.height() {
            pixel = *img.get_pixel(x,y);
            for (wx,wy) in window_indices {
                let wx_px: i32 = x as i32+wx;
                let wy_px: i32 = y as i32+wy;
                if !(wx_px < 0 || wx_px >= img.width() as i32 || wy_px < 0 || wy_px >= img.height() as i32) {
                    let window_px = img.get_pixel(wx_px as u32, wy_px as u32);
                    if (er_or_di && window_px.0 < pixel.0) || ((!er_or_di) && window_px.0 > pixel.0) {
                        pixel = *window_px;
                    }
                }
            }
            out_img.put_pixel(x,y,pixel);
        }
    }
    out_img
}

fn erosion(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    er_di(img, window_indices, true)
}

fn dilation(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    er_di(img, window_indices, false)
}

fn opening(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    dilation(&erosion(img, window_indices), window_indices)
}

fn closing(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    erosion(&dilation(img, window_indices), window_indices)
}

fn opening_with_reconstruction(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    // TODO
    let er_img: &GrayImage = &erosion(img, window_indices);
    let mut out_img: GrayImage = GrayImage::new(img.width(), img.height());
    out_img
}

fn closing_with_reconstruction(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    // TODO
    let di_img: &GrayImage = &dilation(img, window_indices);
    let mut out_img: GrayImage = GrayImage::new(img.width(), img.height());
    out_img
}

fn square(window_size: u32) -> Vec<(i32,i32)> {
    let mut window_indices: Vec<(i32, i32)> = Vec::new();
    for x in -(window_size as i32)+1..window_size as i32{
        for y in -(window_size as i32)+1..window_size as i32 {
            window_indices.push((x,y));
        }
    }
    window_indices
}

fn circle(window_size: u32) -> Vec<(i32,i32)> {
    let mut window_indices: Vec<(i32, i32)> = Vec::new();
    for (x,y) in square(window_size) {
        let dist_sq: i32 = x.pow(2) + y.pow(2);
        if dist_sq <= (window_size as i32 - 1).pow(2) {
            window_indices.push((x,y));
        }
    }
    window_indices
}

fn plus(window_size: u32) -> Vec<(i32,i32)> {
    let mut window_indices: Vec<(i32, i32)> = Vec::new();
    for x in -(window_size as i32)+1..window_size as i32{
        if x != 0 {
            window_indices.push((x,0));
        }
        window_indices.push((0,x));
    }
    window_indices
}

fn usage() {
    println!("usage:\n");
    print!("    mathematical-morphology [input_image_path] [window_size] [window] [operator]");
    print!(" [output_image_path]\n\n");
    println!("available windows:");
    println!("  - square");
    println!("  - circle");
    println!("  - plus\n");
    println!("available operators:");
    println!("  - erosion");
    println!("  - dilation");
    println!("  - opening");
    println!("  - closing");
    println!("  - opening-with-reconstruction");
    println!("  - closing-with-reconstruction");
}

fn main() {
    if env::args().len() != 6 {
        usage();
        panic!("incorrect arguments")
    }

    let args: Vec<String> = env::args().collect();
    let decoded_image: DynamicImage = Reader::open(&args[1]).unwrap().decode().unwrap();
    let img: &GrayImage = decoded_image.as_luma8().unwrap();
    let window_size: u32 = args[2].parse::<u32>().unwrap();
    let window_indices_fn: fn(u32) -> Vec<(i32,i32)> = match args[3].as_ref() {
        "square" => square,
        "circle" => circle,
        "plus" => plus,
        _=> {
            usage();
            panic!("unknown window {}", args[3])
        },
    };
    let window_indices: Vec<(i32,i32)> = window_indices_fn(window_size);
    let out_img_fn: fn(&GrayImage, &Vec<(i32,i32)>) -> GrayImage = match args[4].as_ref() {
        "erosion" => erosion,
        "dilation" => dilation,
        "opening" => opening,
        "closing" => closing,
        "opening-with-reconstruction" => opening_with_reconstruction,
        "closing-with-reconstruction" => closing_with_reconstruction,
        _ => {
            usage();
            panic!("unknown operator {}", args[4])
        },
    };
    let out_img: GrayImage = out_img_fn(img, &window_indices);

    out_img.save(&args[5]).unwrap();
}
