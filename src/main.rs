use std::{
    env,
    cmp,
    time,
};

use image::{
    io::Reader,
    DynamicImage,
    GrayImage,
    Luma,
};

// apply the erosion or dilation morphological operators if 'er_or_di' is true or false
// respectively
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

// apply the opening by reconstruction or closing by reconstruction morphological operators if
// 'op_or_cl' is true or false respectively
fn op_cl_rec(img: &GrayImage, window_indices: &Vec<(i32,i32)>, op_or_cl: bool) -> GrayImage {
    let start_time: time::Instant = time::Instant::now();
    let mut out_img: GrayImage = if op_or_cl {
        erosion(img, window_indices)
    } else {
        dilation(img, window_indices)
    };
    let mut change: bool;
    let mut i: u32 = 0;
    loop {
        i+=1;
        change = false;
        let new_img: GrayImage = if op_or_cl {
            dilation(&out_img, &square(2))
        } else {
            erosion(&out_img, &square(2))
        };
        for x in 0..out_img.width() {
            for y in 0..out_img.height() {
                let original_px_value: u8 = out_img.get_pixel(x,y).0[0];
                let new_px_value: u8 = if op_or_cl {
                    cmp::min(img.get_pixel(x,y).0[0], new_img.get_pixel(x,y).0[0])
                } else {
                    cmp::max(img.get_pixel(x,y).0[0], new_img.get_pixel(x,y).0[0])
                };
                if original_px_value != new_px_value {
                    change = true;
                    out_img.put_pixel(x,y,Luma([new_px_value]));
                }
            }
        }
        if change {
            break;
        }
    }
    let total_time: time::Duration = start_time.elapsed();
    println!("number of iterations:  {}", i);
    println!("total time:  {} ms", total_time.as_millis());
    out_img
}

fn opening_by_reconstruction(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    op_cl_rec(img, window_indices, true)
}

fn closing_by_reconstruction(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    op_cl_rec(img, window_indices, false)
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
    println!("  - opening-by-reconstruction");
    println!("  - closing-by-reconstruction");
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
        _=> panic!("unknown window {}", args[3]),
    };
    let window_indices: Vec<(i32,i32)> = window_indices_fn(window_size);
    let out_img_fn: fn(&GrayImage, &Vec<(i32,i32)>) -> GrayImage = match args[4].as_ref() {
        "erosion" => erosion,
        "dilation" => dilation,
        "opening" => opening,
        "closing" => closing,
        "opening-by-reconstruction" => opening_by_reconstruction,
        "closing-by-reconstruction" => closing_by_reconstruction,
        _ => panic!("unknown operator {}", args[4]),
    };
    let out_img: GrayImage = out_img_fn(img, &window_indices);

    out_img.save(&args[5]).unwrap();
}
