extern crate dirs;
extern crate image;

use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Rgba;
use image::SubImage;
use std::env;
use std::path::Path;

enum BorderDir {
    LEFT,
    TOP,
    RIGHT,
    BOTTOM,
}

fn equal_pixels(ref_pixel: &Rgba<u8>, img: SubImage<&DynamicImage>) -> bool {

    let all_pixels = img.pixels();

    // Return false if any of the pixels differ from the first one.
    for (_x, _y, pixel) in all_pixels {
        if pixel != *ref_pixel {
            return false;
        }
    }

    // All pixels are the same!
    true
}

fn calculate_border(img: &DynamicImage, pixel: Rgba<u8>, border: &BorderDir) -> u32 {

    let (dim_x, dim_y): (u32, u32) = img.dimensions();
    let mut border_size: u32 = 0;

    loop {
        // Check a new set of pixels from the correct border.
        let same_color: bool = equal_pixels(&pixel, match border {
            BorderDir::LEFT => img.view(border_size, 0, 1, dim_y),
            BorderDir::TOP => img.view(0, border_size, dim_x, 1),
            BorderDir::RIGHT => img.view(dim_x-border_size-1, 0, 1, dim_y),
            BorderDir::BOTTOM => img.view(0, dim_y-border_size-1, dim_x, 1)
        });

        // Loop as long as the border is monochromic.
        match same_color {
            true => border_size += 1,
            false => break
        }
    }

    border_size
}

fn cut_border(mut img: DynamicImage) -> (DynamicImage, Rgba<u8>) {

    // The only pixel that exists for sure, used to detect the border.
    let pixel: Rgba<u8> = img.get_pixel(0, 0);

    let default_color = Rgba::<u8>([255, 255, 255, 255]);
    let (dim_x, dim_y): (u32, u32) = img.dimensions();

    // Calculate the size of a monochromic border from all sides.
    let l: u32 = calculate_border(&img, pixel, &BorderDir::LEFT);
    let t: u32 = calculate_border(&img, pixel, &BorderDir::TOP);
    let r: u32 = calculate_border(&img, pixel, &BorderDir::RIGHT);
    let b: u32 = calculate_border(&img, pixel, &BorderDir::BOTTOM);

    // Return the color of the monochromic border if it's found, otherwise a default color.
    match (l>0, t>0, r>0, b>0) {
        (true, true, true, true) => {
            let cut_image: DynamicImage = img.crop(l, t, dim_x-l-r, dim_y-t-b);
            return (cut_image, pixel);
        },
        _ => {
            println!("No monochromic border, padding with white");
            return (img, default_color);
        }
    }
}

fn pad_image(cut_img: DynamicImage, padding_color: Rgba<u8>, arg_padding: u32) -> DynamicImage {

    // Create a new, empty RGBA image with correct dimensions.
    let (cut_x, cut_y): (u32, u32) = cut_img.dimensions();
    let mut padded_img: DynamicImage = DynamicImage::new_rgba8(
        cut_x + 2*arg_padding,
        cut_y + 2*arg_padding,
    );

    // Replace empty pixels with padding color.
    let (padded_x, padded_y): (u32, u32) = padded_img.dimensions();
    for x in 0 .. padded_x {
        for y in 0 .. padded_y {
            // TODO: Pixel iteration?
            padded_img.put_pixel(x, y, padding_color);
        }
    }

    // Copy the cut image on top of the padded one.
    padded_img.copy_from(&cut_img, arg_padding, arg_padding).unwrap();

    padded_img
}

fn save_image(image: &DynamicImage, arg_input: &str, arg_output: &str) {

    // Form the filename which the image is saved as.
    let filename = Path::new(arg_input).file_name().unwrap();
    let full_filename = format!("{}/{}", arg_output, filename.to_str().unwrap());

    image.save(&full_filename).unwrap();
    println!("Saved the result as {}", &full_filename);
}

fn help() {
    println!("Replace uneven padding with unified one.

Usage:  rpad <input> [output] [size]

input   (required) path to input image
output  (optional) path to output directory, default ~
size    (optional) padding size in pixels, default 30");
}

fn error(message: &str) {
    eprintln!("Error: {}", message);
}

fn main() {

    // Read the command line arguments.
    let args: Vec<String> = env::args().collect();

    // Initialize arguments to their default values.
    let mut arg_padding: u32 = 30;
    let home_path = dirs::home_dir().unwrap();
    let mut arg_output = home_path.to_str().unwrap();

    let warn_padding = || { println!("Using default padding {} px", arg_padding) };
    let warn_output = || { println!("Using default output path {}", arg_output) };

    // Make sure all required parameters are fulfilled and replace default values if needed.
    match args.len() {
        2 => {
            warn_padding();
            warn_output();
        },
        3 => {
            // Try interpreting as padding argument.
            match args[2].parse::<u32>() {
                Ok(padding) => {
                    arg_padding = padding;
                    warn_output();
                },
                // If not padding, it must be output directory.
                Err(_) => {
                    arg_output = &args[2];
                    warn_padding();
                }
            }
        },
        4 => {
            arg_output = &args[2];
            arg_padding = args[3].parse::<u32>().unwrap();
            if !Path::new(arg_output).exists() { error("Output path not available"); return; };
        },
        _ => {
            help();
            return;
        }
    }

    // Load the image.
    let arg_input = &args[1];
    let original_img = image::open(arg_input).unwrap();

    // Process the image.
    let (cut_img, padding_color): (DynamicImage, Rgba<u8>) = cut_border(original_img);
    let padded_img = pad_image(cut_img, padding_color, arg_padding);

    save_image(&padded_img, arg_input, arg_output);
}
