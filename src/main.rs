use std::{
    env,
};

use image::{
    io::Reader,
    DynamicImage,
    GrayImage,
};

fn erosion(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    // TODO
    let mut out_img: GrayImage = GrayImage::new(img.width(), img.height());
    out_img
}

fn dilation(img: &GrayImage, window_indices: &Vec<(i32,i32)>) -> GrayImage {
    // TODO
    let mut out_img: GrayImage = GrayImage::new(img.width(), img.height());
    out_img
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
    // TODO
    Vec::new()
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
    print!("[output_image_path]\n\n");
    println!("windows:\n");
    println!("    square");
    println!("    circle");
    println!("    plus");
    println!("operators:\n");
    println!("    erosion");
    println!("    dilation");
    println!("    opening");
    println!("    closing");
    println!("    opening-with-reconstruction");
    println!("    closing-with-reconstruction");
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
    let window_indices: Vec<(i32,i32)> = match args[3].as_ref() {
        "square" => square(window_size),
        "circle" => circle(window_size),
        "plus" => plus(window_size),
        _=> {
            usage();
            panic!("unknown window {}", args[3])
        },
    };
    let out_img: GrayImage = match args[4].as_ref() {
        "erosion" => erosion(img, &window_indices),
        "dilation" => dilation(img, &window_indices),
        "opening" => opening(img, &window_indices),
        "closing" => closing(img, &window_indices),
        "opening-with-reconstruction" => opening_with_reconstruction(img, &window_indices),
        "closing-with-reconstruction" => closing_with_reconstruction(img, &window_indices),
        _ => {
            usage();
            panic!("unknown operator {}", args[4])
        },
    };

    out_img.save(&args[5]).unwrap();
}
